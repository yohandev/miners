use proc_macro_crate::FoundCrate;

/// Get the normalized path of a module given its path and crate name
pub fn mod_path(name: &str, path: &str) -> syn::Path
{
    let root = match proc_macro_crate::crate_name(name).unwrap()
    {
        FoundCrate::Itself => "crate".into(),
        FoundCrate::Name(name) => name,
    };
    syn::parse_str(&*(root + "::" + path)).unwrap()
}