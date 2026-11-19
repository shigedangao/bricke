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
            match field {
                Self::Rename(ref name) => {
                    from_field_name = Some(Ident::new(name.value().as_str(), Span::call_site()))
                }
                Self::ConvertFieldFn(fn_str) => {
                    f = fn_str
                        .parse_with(syn::Path::parse_mod_style)
                        .map_err(|_| syn::Error::new(fn_str.span(), ERROR_PARSE_FN))
                        .ok();
                }
                Self::IsFallible(fallible) => {
                    is_fallible = fallible.value();
                }
                Self::Ignore => to_skip = true,
                Self::DefaultValue(value) => default_value = Some(value),
            }
        }

        if to_skip {
            return quote! { #name: Default::default() };
        }

        let arg_call = match default_value {
            Some(v) => quote! { Some(#v.into()) },
            None => quote! { arg.#from_field_name },
        };

        match f {
            Some(f) => {
                let fn_call_with_arg = if is_fallible {
                    quote! { (#arg_call)? }
                } else {
                    quote! { (#arg_call) }
                };

                quote! { #name: #f #fn_call_with_arg }
            }
            None => quote! { #name: #arg_call },
        }
    }
}
