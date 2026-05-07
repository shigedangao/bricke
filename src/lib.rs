use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, parse_macro_input};

pub(crate) mod attributes;
pub(crate) mod fields;
pub(crate) mod item;

use attributes::BrickeAttributes;
use item::ProcessItem;

/// bricke proc macro is a macro which generates a struct or enum with the specified attributes.
/// This allows to convert a struct to another struct which may contains similar fields while also
/// enabled to convert a field to another type or rename that field. Below are some examples below. Although more detailed examples can be found on the [README](https://github.com/shigedangao/bricke)
///
/// # Examples
///
/// ## Map a struct to another struct
///
/// ```
/// use bricke::bricke;
///
/// struct Foo {
///     a: i32,
///     b: String,
/// }
///
/// #[bricke(converter="From", source="Foo")]
/// struct Bar {
///     a: i32,
///     #[bricke_field(rename="b")]
///     c: String,
/// }
/// ```
///
/// ## Map a struct with a transform function
///
/// ```
/// use bricke::bricke;
///
/// struct Bar {
///     a: i32
/// }
///
/// fn add_self(a: i32) -> i32 {
///     a + a
/// }
///
/// #[bricke(converter="From", source="Bar")]
/// struct Target {
///     #[bricke_field(transform_fn="add_self")]
///     a: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn bricke(args: TokenStream, target: TokenStream) -> TokenStream {
    let input_kind = parse_macro_input!(target as Item);

    let mut attrs = BrickeAttributes::default();
    let bricke_parser = syn::meta::parser(|meta| attrs.parse(&meta));
    parse_macro_input!(args with bricke_parser);

    let expanded = match input_kind {
        Item::Struct(mut item) => item.process(attrs, item::SupportedType::Struct),
        Item::Enum(mut item) => item.process(attrs, item::SupportedType::Enum),
        _ => unimplemented!("Type is not supported"),
    };

    quote! {
        #expanded
    }
    .into()
}
