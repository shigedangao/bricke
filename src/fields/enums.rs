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
    /// * `enum_fields` - The inner fields of the enum.
    pub fn create_enum_template(
        name: &Ident,
        source: Option<&Ident>,
        fields: Vec<Self>,
        enum_fields: EnumInnerFields,
    ) -> TokenStream {
        let mut rename: Option<Ident> = Some(name.clone());
        let mut to_skip = false;
        let mut field_fn: Option<Path> = None;

        for field in fields {
            match field {
                // Process rename field
                Self::Rename(rename_field) => {
                    rename = Some(Ident::new(&rename_field.value(), Span::call_site()));
                }
                // Process ignore field
                Self::Ignore => to_skip = true,
                // Convert field function
                Self::ConvertFieldFn(fn_field) => {
                    field_fn = fn_field
                        .parse_with(syn::Path::parse_mod_style)
                        .map_err(|_| syn::Error::new(fn_field.span(), ERROR_PARSE_FN))
                        .ok();
                }
                _ => {}
            }
        }

        // If we need to skip a field, return an empty token stream
        if to_skip {
            return quote! {};
        }

        match field_fn {
            Some(f) => {
                enum_builder::generate_enum_fn(source, name, rename.as_ref(), &f, &enum_fields)
            }
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
        }
    }
}

mod enum_builder {
    use super::{EnumInnerFields, Ident, Path, TokenStream, quote};

    /// Generates a function call for an enum field.
    ///
    /// # Arguments
    ///
    /// * `source` - The source enum identifier.
    /// * `original_field_name` - The original field name.
    /// * `rename` - The renamed field name.
    /// * `fn_tmpl` - The function template path.
    /// * `enum_inner_fields` - The inner fields of the enum.
    pub fn generate_enum_fn(
        source: Option<&Ident>,
        original_field_name: &Ident,
        rename: Option<&Ident>,
        fn_tmpl: &Path,
        enum_inner_fields: &EnumInnerFields,
    ) -> TokenStream {
        let (source_idents, complete_fn_call) = match enum_inner_fields {
            EnumInnerFields::Unnamed(tk) => (
                quote! {#tk},
                quote! {
                    Self::#original_field_name(#fn_tmpl(#tk))
                },
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
