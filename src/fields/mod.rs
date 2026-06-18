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
const EQ_FIELDS: [&str; 3] = ["transform_fn", "rename", "is_fallible"];

#[derive(Clone)]
pub enum BrickeFieldArgs {
    ConvertFieldFn(LitStr),
    Rename(LitStr),
    Exclude,
    IsFallible(LitBool),
}

impl Parse for BrickeFieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;

        if EQ_FIELDS.contains(&keyword.to_string().as_str()) {
            let _eq_token: Token![=] = input.parse()?;
        }

        match keyword {
            k if k == "transform_fn" => Ok(BrickeFieldArgs::ConvertFieldFn(input.parse()?)),
            k if k == "rename" => Ok(BrickeFieldArgs::Rename(input.parse()?)),
            k if k == "exclude" => Ok(BrickeFieldArgs::Exclude),
            k if k == "is_fallible" => Ok(BrickeFieldArgs::IsFallible(input.parse()?)),
            _ => Err(syn::Error::new(
                keyword.span(),
                format!("Attribute with name '{}' not supported", keyword),
            )),
        }
    }
}
