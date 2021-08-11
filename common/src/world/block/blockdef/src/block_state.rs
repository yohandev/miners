use std::ops::{ Range, RangeInclusive };
use std::convert::{ TryFrom, TryInto };

use syn::spanned::Spanned;
use syn::{ Data, Fields, Index, LitInt, Member, RangeLimits, Token, Type, Variant };
use syn::parse::{Parse, ParseBuffer, ParseStream};
use proc_macro2::Ident;

/// The struct deriving `block::State`
pub struct DeriveInput
{
    pub ident: Ident,
    pub fields: Vec<Field>,
}

/// A field within a struct deriving `block::State`
pub struct Field
{
    pub attr: Attribute,
    pub ident: Member,
    pub ty: Type
}

/// `#[prop(...)]` attribute
pub enum Attribute
{
    /// `#[prop(!)]`
    /// Marks this field as unsized or too large
    Never,
    /// `#[prop(0..16)]`
    /// Indicate this field's valid integer range
    Range(LitRange),
    /// `#[prop(Foo | Bar | Baz)]`
    /// Indicates this field's valid `enum` variants
    Enum(Vec<Variant>),
}

/// Utility: A range literal
pub struct LitRange
{
    /// `..` or `..=`
    pub limits: RangeLimits,
    /// Lower bound
    pub from: LitInt,
    /// Upper bound
    pub to: LitInt,
    /// Parsed range
    range: Range<i32>,
}

/// Short-hand for returning spanned errors
macro_rules! emit_error
{
    ($span:expr, $msg:expr) =>
    {
        return Err(syn::Error::new(syn::spanned::Spanned::span($span), $msg))
    };
}

impl Parse for DeriveInput
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let input = input.parse::<syn::DeriveInput>()?;

        // No generics
        if !input.generics.params.is_empty()
        {
            emit_error!(&input.generics, "Generics not yet supported");
        }

        // Identifier
        let ident = input.ident;
        // Fields
        let fields = match input.data
        {
            Data::Struct(structure) =>
            {
                match structure.fields
                {
                    Fields::Named(fields) => fields.named
                        .into_iter()
                        .map(|f| Field::try_new(f.ident.clone().unwrap(), f))
                        .collect::<syn::Result<Vec<_>>>()?,
                    Fields::Unnamed(fields) => fields.unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(i, f)| Field::try_new(Index::from(i), f))
                        .collect::<syn::Result<Vec<_>>>()?,
                    Fields::Unit => Default::default(),
                }
            },
            Data::Enum(e) => emit_error!(&e.enum_token, "`enum`s not yet supported"),
            Data::Union(u) => emit_error!(&u.union_token, "`union`s not yet supported"),
        };
        
        Ok(Self { ident, fields })
    }
}

impl Field
{
    /// Parse a `block::State` implementor field from a generic field
    fn try_new<T: Into<Member>>(ident: T, field: syn::Field) -> syn::Result<Self>
    {
        let span = field.ident.span();

        if let Some(attr) = field.attrs
            .into_iter()
            .find(|a| a.path.is_ident("prop"))
        {
            Ok(Self
            {
                attr: attr.try_into()?,
                ident: ident.into(),
                ty: field.ty,
            })
        }
        else
        {
            emit_error!(&span, " All fields deriving `block::State` must be annotated with `#[prop(...)]`")
        }
    }
}

impl TryFrom<syn::Attribute> for Attribute
{
    type Error = syn::Error;

    fn try_from(attr: syn::Attribute) -> syn::Result<Self>
    {
        if !attr.path.is_ident("prop")
        {
            panic!("Expected `#[prop(...)]` attribute")
        }

        // `#[prop(!)]`
        if let Ok(_) = attr.parse_args::<Token!(!)>()
        {
            Ok(Self::Never)
        }
        // `#[prop(0..16)]`
        else if let Ok(range) = attr.parse_args::<LitRange>()
        {
            Ok(Self::Range(range))
        }
        // `#[prop(Foo | Bar | Baz)]`
        else if let Ok(variants) = attr.parse_args_with(|parse: &ParseBuffer|
            parse.parse_terminated::<Variant, Token!(|)>(Variant::parse))
        {
            Ok(Self::Enum(variants.into_iter().collect()))
        }
        // `#[prop(???)]`
        else
        {
            emit_error!(&attr.path, format!("Expected one of:\n{}\n{}\n{}",
                "`#[prop(!)]`           - Field isn't sized or too large",
                "`#[prop(0..16)]`       - Field is an integer range",
                "`#[prop(Foo | Bar)]`   - Field accepts these `enum` variants",
            ))
        }
    }
}

impl Attribute
{
    pub fn bit_size(&self) -> Option<usize>
    {
        match self
        {
            Attribute::Never => None,
            Attribute::Range(range) => Some(range.range().len()),
            Attribute::Enum(variants) => Some(variants.len()),
        }
    }
}

impl Parse for LitRange
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let from = input.parse::<Option<LitInt>>()?;
        let limits = input.parse::<RangeLimits>()?;
        let to = input.parse::<Option<LitInt>>()?;

        if from.is_none() || to.is_none()
        {
            emit_error!(&input.span(), "Open-ended ranges unsupported, use `#[prop(!)]` instead")
        }
        let from = from.unwrap();
        let to = to.unwrap();
        let range = match limits
        {
            RangeLimits::HalfOpen(_) => from.base10_parse()?..to.base10_parse()?,
            RangeLimits::Closed(_) => from.base10_parse()?..(to.base10_parse::<i32>()? - 1),
        };

        Ok(Self { limits, from, to, range })
    }
}

impl LitRange
{
    pub fn range(&self) -> Range<i32>
    {
        self.range.clone()
    }

    pub fn range_inclusive(&self) -> RangeInclusive<i32>
    {
        self.range.start..=(self.range.end + 1)
    }
}