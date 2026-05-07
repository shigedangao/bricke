use crate::item::SupportedType;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitStr, Result, Type, meta::ParseNestedMeta, spanned::Spanned};

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
}

impl BrickeAttributes {
    pub fn parse(&mut self, meta: &ParseNestedMeta) -> Result<()> {
        if meta.path.get_ident().is_none() {
            return Err(syn::Error::new(meta.path.span(), "Unknown attribute"));
        }

        let ident = meta.path.get_ident().unwrap();
        match ident.to_string().as_str() {
            "converter" => {
                let converter: LitStr = meta.value()?.parse()?;
                self.converter = match converter.value().as_str() {
                    "TryFrom" => ConverterType::TryFrom,
                    _ => ConverterType::From,
                };

                Ok(())
            }
            "source" => {
                let source: Option<LitStr> = meta.value()?.parse()?;
                if let Some(src) = source {
                    self.source = Some(Ident::new(&src.value(), Span::call_site()));
                }

                Ok(())
            }
            "try_error_kind" => {
                self.error_kind = Some(meta.value()?.parse()?);

                Ok(())
            }
            _ => Err(syn::Error::new(ident.span(), "Unknown attribute")),
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
    ) -> TokenStream {
        // Generate the conversion template for the list of fields that has been transformed
        let (source, fields) = if let Some(source) = &self.source {
            match supported_type {
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
            }
        } else {
            unimplemented!("Expect supported_type to be a struct or an enum")
        };

        match self.converter {
            ConverterType::From => {
                quote! {
                    impl From<#source> for #target_ident {
                        fn from(arg: #source) -> Self {
                            #fields
                        }
                    }
                }
            }
            ConverterType::TryFrom => {
                let error_kind = self
                    .error_kind
                    .as_ref()
                    .expect("Expect try_error_kind to be provided");
                let error_kind_ident: Type =
                    syn::parse_str(&error_kind.value()).expect("Expect to parse error_kind");

                quote! {
                    impl TryFrom<#source> for #target_ident {
                        type Error = #error_kind_ident;

                        fn try_from(arg: #source) -> Result<Self, Self::Error> {
                            Ok(#fields)
                        }
                    }
                }
            }
        }
    }
}
