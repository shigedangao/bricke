use crate::item::SupportedType;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Lifetime;
use syn::parse::Parse;
use syn::{
    Ident, LitStr, Result, Token, Type, meta::ParseNestedMeta, punctuated::Punctuated,
    spanned::Spanned,
};

// Attributes constants supported by the proc-macro
const ATTRIBUTE_CONVERTER: &str = "converter";
const ATTRIBUTE_SOURCE: &str = "source";
const ATTRIBUTE_ERROR_KIND: &str = "try_error_type";
const ATTRIBUTE_LIFETIMES: &str = "lifetimes";

#[derive(Default, PartialEq)]
pub enum ConverterType {
    #[default]
    From,
    TryFrom,
}

/// brickeAttributes is a struct that holds the attributes for the bricke proc macro.
///
/// - Converter refers to the type of conversion to be performed (`From` or `TryFrom`) default = `From`
/// - source refers to the struct or enum that the bricke will be converted from
/// - `error_kind` refers to the error kind that will be returned if the conversion fails (use in conjunction with `TryFrom`)
#[derive(Default)]
pub struct BrickeAttributes {
    pub converter: ConverterType,
    pub source: Option<Ident>,
    pub error_kind: Option<LitStr>,
    pub lifetimes: Option<Punctuated<Lifetime, Token![,]>>,
}

impl BrickeAttributes {
    pub fn parse(&mut self, meta: &ParseNestedMeta) -> Result<()> {
        if meta.path.get_ident().is_none() {
            return Err(syn::Error::new(meta.path.span(), "Unknown attribute"));
        }

        let ident = meta.path.get_ident().unwrap();
        match ident.to_string().as_str() {
            ATTRIBUTE_CONVERTER => {
                let converter: LitStr = meta.value()?.parse()?;
                self.converter = match converter.value().as_str() {
                    "TryFrom" => ConverterType::TryFrom,
                    _ => ConverterType::From,
                };

                Ok(())
            }
            ATTRIBUTE_SOURCE => {
                let source: Option<LitStr> = meta.value()?.parse()?;
                if let Some(src) = source {
                    self.source = Some(Ident::new(&src.value(), Span::call_site()));
                }

                Ok(())
            }
            ATTRIBUTE_ERROR_KIND => {
                self.error_kind = Some(meta.value()?.parse()?);

                Ok(())
            }
            ATTRIBUTE_LIFETIMES => {
                let lifetime_tokens: Punctuated<Lifetime, Token![,]> =
                    meta.value()?.parse_terminated(Lifetime::parse, Token![,])?;

                self.lifetimes = Some(lifetime_tokens);

                Ok(())
            }
            _ => Err(syn::Error::new(
                ident.span(),
                format!("Unknown attribute: {}", ident),
            )),
        }
    }

    /// Create the conversion template for the target item (struct or enum)
    ///
    /// # Arguments
    ///
    /// * `target_ident` - The target struct identifier
    /// * `transform_fields` - The transformed fields
    pub fn generate_conversion_template(
        &self,
        target_ident: &Ident,
        transform_fields: &[TokenStream],
        supported_type: &SupportedType,
        target_lifetimes: &Punctuated<Lifetime, Token![,]>,
    ) -> TokenStream {
        let Some(source) = &self.source else {
            unimplemented!("Expect supported_type to be a struct or an enum")
        };

        // Generate the conversion template for the list of fields that has been transformed
        let (source, fields) = match supported_type {
            SupportedType::Struct => (
                source,
                quote! {
                    Self {
                        #(#transform_fields),*
                    }
                },
            ),
            // In the case of the enum we want to use the match expression to convert the source enum to the target enum
            SupportedType::Enum => (
                source,
                quote! {
                    match arg {
                        #(#transform_fields),*
                    }
                },
            ),
        };

        // Create the lifetime annotations for the source
        let lifetime_annotations = match &self.lifetimes {
            Some(lf) => quote! {<#lf>},
            None => quote! {},
        };

        // Create the lifetime annotations for the target
        let target_lifetimes_output = if !target_lifetimes.is_empty() {
            quote! { <#target_lifetimes> }
        } else {
            quote! {}
        };

        match self.converter {
            ConverterType::From => {
                quote! {
                    impl #lifetime_annotations From<#source #lifetime_annotations> for #target_ident #target_lifetimes_output {
                        fn from(arg: #source #lifetime_annotations) -> Self {
                            #fields
                        }
                    }
                }
            }
            ConverterType::TryFrom => {
                let error_kind = self
                    .error_kind
                    .as_ref()
                    .expect("Expect try_error_type to be provided");
                let error_kind_ident: Type =
                    syn::parse_str(&error_kind.value()).expect("Expect to parse error_kind");

                quote! {
                    impl #lifetime_annotations TryFrom<#source #lifetime_annotations> for #target_ident #target_lifetimes_output {
                        type Error = #error_kind_ident;

                        fn try_from(arg: #source #lifetime_annotations) -> Result<Self, Self::Error> {
                            Ok(#fields)
                        }
                    }
                }
            }
        }
    }
}
