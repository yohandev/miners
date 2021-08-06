mod block_state;

use proc_macro2::Span;
use proc_macro_crate::{ crate_name, FoundCrate };
use syn::{Ident, parse_macro_input};
use quote::quote;

#[proc_macro_derive(State, attributes(prop))]
pub fn derive_block_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let input = parse_macro_input!(input as block_state::DeriveInput);

    let name = input.ident;
    let path = match crate_name("miners_common").unwrap()
    {
        FoundCrate::Itself => quote!{ crate::world::block },
        FoundCrate::Name(name) =>
        {
            let ident = Ident::new(&name, Span::call_site());

            quote!{ #ident::world::block }
        }
    };

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