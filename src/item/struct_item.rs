use super::{FIELD_NAME, ProcessItem};
use crate::attributes::BrickeAttributes;
use crate::fields::BrickeFieldArgs;
use crate::item::SupportedType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, Token, punctuated::Punctuated, spanned::Spanned};

impl ProcessItem for ItemStruct {
    fn process(&mut self, attrs: BrickeAttributes, supported_type: SupportedType) -> TokenStream {
        let mut processed_fields = Vec::with_capacity(self.fields.len());

        for field in &self.fields {
            let name = field
                .ident
                .clone()
                .expect("Expect to found an identifier e.g: `name`");

            let mut field_attrs = Vec::with_capacity(field.attrs.len());

            for attr in &field.attrs {
                // We parse the attributes only for the `bricke_field` attribute e.g: `#[bricke_field(transform_fn = "fn")]`
                if attr.path().is_ident(FIELD_NAME) {
                    let meta: Punctuated<BrickeFieldArgs, Token![,]> = attr
                        .parse_args_with(Punctuated::parse_terminated)
                        .map_err(|err| {
                            syn::Error::new(
                                attr.span(),
                                format!("Unable to parse struct attribute {err}"),
                            )
                        })
                        .unwrap();

                    field_attrs.extend(meta);
                }
            }

            processed_fields.push(BrickeFieldArgs::create_struct_template(&name, field_attrs));
        }

        // Use to remove the attributes bricke_field from the AST so that it doesn't get printed
        self.fields.iter_mut().for_each(|field| {
            field.attrs.retain(|attr| !attr.path().is_ident(FIELD_NAME));
        });

        let expanded =
            attrs.generate_conversion_template(&self.ident, &processed_fields, &supported_type);

        quote! {
            #self
            #expanded
        }
    }
}
