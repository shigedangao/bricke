use super::ProcessItem;
use crate::{
    attributes::BrickeAttributes,
    fields::BrickeFieldArgs,
    item::{FIELD_NAME, SupportedType},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Fields, Ident, ItemEnum, Lifetime, Result, Token, punctuated::Punctuated, spanned::Spanned,
};

#[derive(Debug)]
pub enum EnumInnerFields {
    Unnamed(TokenStream),
    Named(TokenStream),
    Unit,
}

impl ProcessItem for ItemEnum {
    fn process(
        mut self,
        attrs: BrickeAttributes,
        supported_type: SupportedType,
    ) -> Result<proc_macro2::TokenStream> {
        let target_lifetimes: Punctuated<Lifetime, Token![,]> = self
            .generics
            .lifetimes()
            .map(|v| v.lifetime.clone())
            .collect();

        let mut field_tk = Vec::with_capacity(self.variants.len());
        for item in &mut self.variants {
            let parsed_enum_fields = process_enum_inner_fields(&item.fields);
            let mut field_attrs = Vec::with_capacity(item.attrs.len());

            for attr in &item.attrs {
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
                        })?;

                    field_attrs.extend(meta);
                }
            }

            field_tk.push(BrickeFieldArgs::create_enum_template(
                &item.ident,
                attrs.source.as_ref(),
                field_attrs,
                parsed_enum_fields,
            ));

            // Remove the #[bricke(field)] attribute from the variants before passing to the TokenStream
            item.attrs.retain(|attr| !attr.path().is_ident(FIELD_NAME));
        }

        let expanded = attrs.generate_conversion_template(
            &self.ident,
            &field_tk,
            &supported_type,
            &target_lifetimes,
        );

        Ok(quote! {
            #self
            #expanded
        })
    }
}

/// Process the enum fields e.g `Enum::Variant(arg1, arg2)`
///
/// # Description
/// This function will take the enum fields and process them into a `TokenStream`
/// - Unnamed fields will generate a tuple of arguments in the following format (`arg_0`, `arg_1`, ...)
/// - Unit will just produce an empty `TokenStream`
/// - Named fields will generate a tuple of arguments in the following format (`arg_0`, `arg_1`, ...)
fn process_enum_inner_fields(fields: &Fields) -> EnumInnerFields {
    match fields {
        Fields::Unnamed(un) => {
            let parsed_fields: Vec<TokenStream> = un
                .unnamed
                .iter()
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
                .iter()
                .enumerate()
                .map(|(idx, field)| match &field.ident {
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
