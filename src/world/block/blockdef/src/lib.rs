mod block_state;

use quote::quote;
use syn::{ parse_macro_input };

#[proc_macro_derive(State, attributes(prop))]
pub fn derive_block_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let input = parse_macro_input!(input as block_state::DeriveInput);

    let name = input.ident;
    let path = quote! { crate::world::block };

    let expanded = quote!
    {
        #[automatically_derived]
        impl #path::State for #name
        {
            // temporary
            const REPR: #path::Repr<Self> = #path::Repr::Ptr;
        }
    };

    expanded.into()
}