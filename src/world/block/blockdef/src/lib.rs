mod block_state;
mod util;

#[proc_macro_derive(State, attributes(prop))]
pub fn derive_block_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let input = syn::parse_macro_input!(input as block_state::DeriveInput);

    let name = input.ident;

    let block_path = util::mod_path("miners_common", "world::block");
    let util_path = util::mod_path("miners_common", "util");

    // Size, in bits, of block state
    let bitsize = input.fields
        .iter()
        .map(|f| f.attr.bit_size())
        .sum::<Option<usize>>();
    // `block::Repr::Val` - state is less than 6 bits
    let repr = if bitsize.is_some() && bitsize.unwrap() <= 6
    {
        let mut offset = 0;
        let into_packed = input.fields
            .iter()
            .map(|f| impl_into_packed(f, &mut offset));
        let mut offset = 0;
        let from_packed = input.fields
            .iter()
            .map(|f| impl_from_packed(f, &mut offset));

        quote::quote!
        {
            Repr::Val
            {
                into_packed: |this|
                {
                    let mut buf = <#util_path::Bits::<6> as Default>::default();
                    #(#into_packed)*
                    buf
                },
                from_packed: |packed| Self { #(#from_packed),* },
            }
        }
    }
    // `block::Repr::Ptr` - state is more than 6 bits
    else
    {
        quote::quote! { Repr::Ptr }
    };

    let expanded = quote::quote!
    {
        #[automatically_derived]
        impl #block_path::State for #name
        {
            // temporary
            const REPR: #block_path::Repr<Self> = #block_path::#repr;
        }
    };

    expanded.into()
}

/// Implementation of `block::Repr::Val::into_packed` for a field given
/// its bit offset
fn impl_into_packed(field: &block_state::Field, offset: &mut usize) -> proc_macro2::TokenStream
{
    let name = &field.ident;
    let size = field.attr.bit_size().unwrap();
    let ty = &field.ty;

    let out = match &field.attr
    {
        block_state::Attribute::Range(range) =>
        {
            let range = range.range_inclusive();
            let from = range.start();
            let to = range.end();

            // buf.set<0, 2>(match self.foo
            // {
            //      n @ 4..8 => n - 4,
            //      _ => 0
            // })
            quote::quote!
            {{
                const FROM: #ty = #from as _;
                const TO: #ty = #to as _;
                buf.set::<#offset, { #offset + #size }>(match this.#name
                {
                    n @ FROM..=TO => (n - FROM) as u8,
                    _ => 0u8,
                });
            }}
        },
        block_state::Attribute::Enum(variants) =>
        {
            // Branches of match block below
            let branches = variants
                .iter()
                .enumerate()
                .map(|(idx, variant)| quote::quote!
            {
                <#ty>::#variant => #idx as u8
            });

            // buf.set<0, 2>(match self.foo
            // {
            //      Foo::Bar => 0,
            //      Foo::Baz => 1,
            //      Foo::Bat => 2,
            //      _ => 0,
            // })
            quote::quote!
            {
                buf.set::<#offset, { #offset + #size }>(match this.#name
                {
                    #(#branches),*,
                    _ => 0u8
                });
            }
        },
        _ => unreachable!()
    };
    *offset += size;
    out
}

/// Implementation of `block::Repr::Val::from_packed` for a field given
/// its bit offset
fn impl_from_packed(field: &block_state::Field, offset: &mut usize) -> proc_macro2::TokenStream
{
    let name = &field.ident;
    let size = field.attr.bit_size().unwrap();
    let ty = &field.ty;

    let out = match &field.attr
    {
        block_state::Attribute::Range(range) =>
        {
            let from = *range.range_inclusive().start();

            // foo: buf.get<0, 2>() as i32 + 4 as i32 // <-- from
            quote::quote!
            {
                #name: packed.get::<#offset, { #offset + #size }>() as #ty + #from as #ty
            }
        },
        block_state::Attribute::Enum(variants) =>
        {
            // Branches of match below
            let branches = variants
                .iter()
                .enumerate()
                .map(|(idx, v)| (idx as u8, v))
                .map(|(idx, variant)| quote::quote!
            {
                #idx => <#ty>::#variant
            });
            let default = &variants[0];

            // foo: match buf.get<0, 2>
            // {
            //      0 => Foo::Bar,
            //      1 => Foo::Baz,
            //      2 => Foo::Bat.
            //      _ => Foo::Bar,
            // }
            quote::quote!
            {
                #name: match packed.get::<#offset, { #offset + #size }>()
                {
                    #(#branches),*,
                    _ => <#ty>::#default
                }
            }
        },
        _ => unreachable!()
    };
    *offset += size;
    out
}