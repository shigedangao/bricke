use super::{BrickeFieldArgs, ERROR_PARSE_FN, Ident, Span, TokenStream, quote};
use syn::{LitStr, Path};

impl BrickeFieldArgs {
    /// Create the struct template which will be used inside the field to map the path src: target
    ///
    /// # Arguments
    /// * `name` - The name of the struct template.
    /// * `fields` - The fields of the struct template.
    pub(crate) fn create_struct_template(name: &Ident, fields: Vec<Self>) -> TokenStream {
        let mut from_field_name: Option<Ident> = Some(name.clone());
        let mut f: Option<Path> = None;
        let mut to_skip = false;
        let mut is_fallible = false;
        let mut default_value: Option<LitStr> = None;

        for field in fields {
            if let Self::Rename(n) = &field {
                from_field_name = Some(Ident::new(&n.value(), Span::call_site()));
            }

            if let Self::ConvertFieldFn(fn_str) = &field {
                f = fn_str
                    .parse_with(syn::Path::parse_mod_style)
                    .map_err(|_| syn::Error::new(fn_str.span(), ERROR_PARSE_FN))
                    .ok();
            }

            if let Self::IsFallible(r) = &field {
                is_fallible = r.value();
            }

            // In the case where we exclude the field, we just skip to output that field.
            if let Self::Ignore = &field {
                to_skip = true;
            }

            // Set default value whenever we have a default value specified (usually for Optional fields).
            if let Self::DefaultValue(val) = &field {
                default_value = Some(val.clone());
            }
        }

        // If we have a default value, use it; otherwise, use the field value from the argument.
        let arg_call = match default_value {
            Some(v) => quote! { Some(#v.into()) },
            None => quote! { arg.#from_field_name },
        };

        let res_call = if is_fallible {
            quote! { (#arg_call)? }
        } else {
            quote! { (#arg_call) }
        };

        if to_skip {
            quote! { #name: Default::default() }
        } else {
            match f {
                Some(f) => quote! { #name: #f #res_call },
                None => quote! { #name: #arg_call },
            }
        }
    }
}
