use syn::parse::{ Parse, ParseStream };

pub struct MacroInput
{
    /// Some expression that's `&'static str`
    pub id: syn::Expr,
    /// Some expression that's `Into<Cow<'static, str>>`
    pub name: syn::Expr,
    /// The concrete structure implementing `block::State`
    pub ty: syn::ItemStruct,
}

/// Short-hand for returning spanned errors
macro_rules! emit_error
{
    ($span:expr, $msg:expr) =>
    {
        return Err(syn::Error::new(syn::spanned::Spanned::span(&$span), $msg))
    };
}

impl Parse for MacroInput
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        // `id: "wooden_planks"`
        let id = match input.parse::<syn::FieldValue>()
        {
            Ok(id) if matches!(id.member, syn::Member::Named(ref i) if i == "id") => id.expr,
            _ => emit_error!(input.span(), "Expected `id` field in this position"),
        };
        input.parse::<Option<syn::token::Comma>>()?;

        // `name: format!("{} Planks", self.variant)`
        let name = match input.parse::<syn::FieldValue>()
        {
            Ok(name) if matches!(name.member, syn::Member::Named(ref i) if i == "name") => name.expr,
            _ => emit_error!(input.span(), "Expected `name` field in this position"),
        };
        input.parse::<Option<syn::token::Comma>>()?;

        // ```
        // #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        // pub struct BlockWoodenPlanks { -- snip -- }
        // ```
        let ty = input.parse::<syn::ItemStruct>()?;

        if !input.is_empty()
        {
            panic!("Unexpected left over tokens")
        }

        Ok(Self { id, name, ty })
    }
}