use crate::attributes::BrickeAttributes;

pub(crate) mod enum_item;
pub(crate) mod struct_item;

/// `FIELD_NAME` is the field name used for bricke fields e.g: use inside of a struct
///
/// # Example
/// ```
/// use bricke::bricke;
///
/// struct Source {
///     name: String,
/// }
///
/// #[bricke(converter = "From", source = "Source")]
/// struct MyStruct {
///     #[bricke_field(rename = "name")]
///     foo: String,
/// }
/// ```
const FIELD_NAME: &str = "bricke_field";

/// `SupportedType` is an enum that defines the supported types for bricke items
///
/// /!\ So far the lib only supports structs and enums
#[derive(Clone)]
pub enum SupportedType {
    Struct,
    Enum,
}

/// `ProcessItem` is a trait that defines how to process a bricke item
///
/// Each item type (struct, enum) should implement this trait to define how to process its fields
pub(crate) trait ProcessItem {
    /// Process the item with the given attributes and supported type
    ///
    /// # Arguments
    ///
    /// * `attrs` - The attributes of the item
    /// * `supported_type` - The supported type of the item (struct or enum)
    fn process(
        &mut self,
        attrs: BrickeAttributes,
        supported_type: SupportedType,
    ) -> proc_macro2::TokenStream;
}
