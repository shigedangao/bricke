use super::{BrickeFieldArgs, ERROR_PARSE_FN, Ident, Span, TokenStream, quote};
use syn::Path;

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

        for field in fields {
            if let Self::Rename(n) = field.clone() {
                from_field_name = Some(Ident::new(&n.value(), Span::call_site()));
            }

            if let Self::ConvertFieldFn(fn_str) = field.clone() {
                f = fn_str
                    .parse_with(syn::Path::parse_mod_style)
                    .map_err(|_| syn::Error::new(fn_str.span(), ERROR_PARSE_FN))
                    .ok();
            }

            if let Self::IsFallible(r) = field.clone() {
                is_fallible = r.value();
            }

            // In the case where we exclude the field, we just skip to output that field.
            if let Self::Exclude(e) = field.clone()
                && e.value()
            {
                to_skip = true;
            }
        }

        let res_call = if is_fallible {
            quote! { (arg.#from_field_name)? }
        } else {
            quote! { (arg.#from_field_name) }
        };

        if to_skip {
            quote! { #name: Default::default() }
        } else {
            match f {
                Some(f) => quote! { #name: #f #res_call },
                None => quote! { #name: arg.#from_field_name },
            }
        }
    }
}
