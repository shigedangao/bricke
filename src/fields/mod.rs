use crate::item::enum_item::EnumInnerFields;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitBool, LitStr, Result, Token, parse::Parse, parse::ParseStream};

pub mod enums;
pub mod structure;

// Constants
const ERROR_PARSE_FN: &str = "Expect a function call";

// Fields that needs the '=' token to be parsed in order to get the extracted value.
const FIELD_TRANSFORM: &str = "transform_fn";
const FIELD_RENAME: &str = "rename";
const FIELD_IGNORE: &str = "ignore";
const FIELD_IS_FALLIBLE: &str = "is_fallible";
const FIELD_DEFAULT_VALUE: &str = "default_value";

// Eq fields are fields that needs the '=' token to be parsed in order to get the extracted value.
const EQ_FIELDS: [&str; 4] = [
    FIELD_TRANSFORM,
    FIELD_RENAME,
    FIELD_IS_FALLIBLE,
    FIELD_DEFAULT_VALUE,
];

#[derive(Clone)]
pub enum BrickeFieldArgs {
    ConvertFieldFn(LitStr),
    Rename(LitStr),
    Ignore,
    IsFallible(LitBool),
    DefaultValue(LitStr),
}

impl Parse for BrickeFieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;

        if EQ_FIELDS.contains(&keyword.to_string().as_str()) {
            let _eq_token: Token![=] = input.parse()?;
        }

        match keyword {
            k if k == FIELD_TRANSFORM => Ok(BrickeFieldArgs::ConvertFieldFn(input.parse()?)),
            k if k == FIELD_RENAME => Ok(BrickeFieldArgs::Rename(input.parse()?)),
            k if k == FIELD_IGNORE => Ok(BrickeFieldArgs::Ignore),
            k if k == FIELD_IS_FALLIBLE => Ok(BrickeFieldArgs::IsFallible(input.parse()?)),
            k if k == FIELD_DEFAULT_VALUE => Ok(BrickeFieldArgs::DefaultValue(input.parse()?)),
            _ => Err(syn::Error::new(
                keyword.span(),
                format!("Attribute with name '{}' not supported", keyword),
            )),
        }
    }
}
