use super::ProcessItem;
use crate::{
    attributes::BrickeAttributes,
    fields::BrickeFieldArgs,
    item::{FIELD_NAME, SupportedType},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, Ident, ItemEnum, Token, punctuated::Punctuated, spanned::Spanned};

#[derive(Debug)]
pub enum EnumInnerFields {
    Unnamed(TokenStream),
    Named(TokenStream),
    Unit,
}

impl ProcessItem for ItemEnum {
    fn process(
        &mut self,
        attrs: BrickeAttributes,
        supported_type: SupportedType,
    ) -> proc_macro2::TokenStream {
        let target = self.ident.clone();

        let mut field_tk = Vec::with_capacity(self.variants.len());
        for item in self.variants.clone() {
            let field_name = item.ident;
            let parsed_enum_fields = process_enum_inner_fields(item.fields);

            let mut field_attrs = Vec::new();
            for attr in item.attrs {
                // Like the struct fields, we need to collect the #[bricke_field] attributes
                if attr.path().is_ident(super::FIELD_NAME) {
                    // Parse the #[bricke_field] attribute arguments separate by a comma and collect them
                    let meta: Punctuated<BrickeFieldArgs, Token![,]> = attr
                        .parse_args_with(Punctuated::parse_terminated)
                        .map_err(|err| {
                            syn::Error::new(
                                attr.span(),
                                format!("Unable to parse enum attributes {err}"),
                            )
                        })
                        .unwrap();

                    field_attrs.extend(meta);
                }
            }

            field_tk.push(BrickeFieldArgs::create_enum_template(
                &field_name,
                attrs.source.as_ref(),
                field_attrs,
                parsed_enum_fields,
            ));
        }

        let expanded = attrs.generate_conversion_template(&target, &field_tk, &supported_type);

        // Remove the #[bricke(field)] attribute from the variants before passing to the TokenStream
        self.variants.iter_mut().for_each(|field| {
            field.attrs.retain(|attr| !attr.path().is_ident(FIELD_NAME));
        });

        quote! {
            #self
            #expanded
        }
    }
}

/// Process the enum fields e.g `Enum::Variant(arg1, arg2)`
///
/// # Description
/// This function will take the enum fields and process them into a `TokenStream`
/// - Unnamed fields will generate a tuple of arguments in the following format (`arg_0`, `arg_1`, ...)
/// - Unit will just produce an empty `TokenStream`
/// - Named fields will generate a tuple of arguments in the following format (`arg_0`, `arg_1`, ...)
///
/// /!\ Named fields are not supported yet
fn process_enum_inner_fields(fields: Fields) -> EnumInnerFields {
    match fields {
        Fields::Unnamed(un) => {
            let parsed_fields: Vec<TokenStream> = un
                .unnamed
                .into_iter()
                .enumerate()
                .map(|(idx, field)| {
                    let ident = Ident::new(&format!("arg_{idx}"), field.span());

                    quote! { #ident }
                })
                .collect();

            EnumInnerFields::Unnamed(quote! {(#(#parsed_fields),*)})
        }
        // Return the same TokenStream for the name fields
        Fields::Named(nfields) => {
            let parsed_nfields: Vec<TokenStream> = nfields
                .named
                .into_iter()
                .enumerate()
                .map(|(idx, field)| match field.ident {
                    Some(ident) => {
                        let id = Ident::new(&ident.to_string(), ident.span());
                        quote! { #id }
                    }
                    None => quote! { #idx },
                })
                .collect();

            EnumInnerFields::Named(quote! {#(#parsed_nfields),*})
        }
        Fields::Unit => EnumInnerFields::Unit,
    }
}
