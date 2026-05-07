use super::{BrickeFieldArgs, ERROR_PARSE_FN, EnumInnerFields, Ident, Span, TokenStream, quote};
use syn::Path;

impl BrickeFieldArgs {
    /// Create the enum template which will be used inside the field to map the path src: target within a match statement.
    /// This will create an enum value for "each statement" e.g:
    ///    - `Source::Foo` => `Target::Foo`
    ///
    /// # Arguments
    /// * `name` - The name of the enum template.
    /// * `source` - The source of the enum template.
    /// * `fields` - The fields of the enum template.
    pub fn create_enum_template(
        name: Ident,
        source: Option<Ident>,
        fields: Vec<Self>,
        enum_fields: EnumInnerFields,
    ) -> TokenStream {
        let mut rename: Option<Ident> = Some(name.clone());
        let mut to_skip = false;
        let mut f: Option<Path> = None;

        for field in fields {
            if let Self::Rename(rename_field) = field.clone() {
                rename = Some(Ident::new(&rename_field.value(), Span::call_site()));
            }

            if let Self::Exclude(e) = field.clone()
                && e.value()
            {
                to_skip = true;
            }

            if let Self::ConvertFieldFn(fn_field) = field.clone() {
                f = fn_field
                    .parse_with(syn::Path::parse_mod_style)
                    .map_err(|_| syn::Error::new(fn_field.span(), ERROR_PARSE_FN))
                    .ok();
            }
        }

        match to_skip {
            true => quote! {},
            false => match f {
                Some(f) => enum_builder::generate_enum_fn(source, &name, rename, &f, &enum_fields),
                None => match enum_fields {
                    EnumInnerFields::Unnamed(unnamed_enum_fields) => {
                        quote! {
                            #source::#rename #unnamed_enum_fields => Self::#name #unnamed_enum_fields
                        }
                    }
                    EnumInnerFields::Named(named_enum_fields) => {
                        quote! {
                            #source::#rename{#named_enum_fields} => Self::#name {#named_enum_fields}
                        }
                    }
                    EnumInnerFields::Unit => {
                        quote! {
                            #source::#rename => Self::#name
                        }
                    }
                },
            },
        }
    }
}

mod enum_builder {
    use super::{EnumInnerFields, Ident, Path, TokenStream, quote};

    pub fn generate_enum_fn(
        source: Option<Ident>,
        original_field_name: &Ident,
        rename: Option<Ident>,
        fn_tmpl: &Path,
        enum_inner_fields: &EnumInnerFields,
    ) -> TokenStream {
        let (source_idents, complete_fn_call) = match enum_inner_fields {
            EnumInnerFields::Unnamed(tk) => (
                tk.clone(),
                quote! {
                Self::#original_field_name(#fn_tmpl(#tk)) },
            ),
            EnumInnerFields::Named(tk) => (
                quote! {
                    {#tk}
                },
                quote! {
                    #fn_tmpl (#tk)
                },
            ),
            EnumInnerFields::Unit => (
                quote! {},
                quote! {
                    #fn_tmpl (#source::#rename)
                },
            ),
        };

        quote! {
            #source::#rename #source_idents => #complete_fn_call
        }
    }
}
