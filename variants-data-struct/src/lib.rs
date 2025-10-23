#[doc = include_str!("../README.md")]
use proc_macro::TokenStream;

mod variants_data_struct_attr_meta;
mod variants_data_struct_defs;
mod variants_data_struct_field_attr_meta;
mod variants_data_struct_field_meta;
mod variants_data_struct_meta;

use crate::variants_data_struct_attr_meta::VariantsDataStructAttrMeta;
use crate::variants_data_struct_defs::{VariantsDataStructDefs, variants_data_struct_defs};
use crate::variants_data_struct_meta::VariantsDataStructMeta;

/// Derive macro to generate a data struct containing fields for each variant of the enum.
///
/// ```rust
/// use variants_data_struct::VariantsDataStruct;
///
/// #[derive(VariantsDataStruct)]
/// pub enum MyEnum {
///     UnitEnum,
///     TupleEnum(i32, String),
///     StructEnum { id: u32, name: String },
/// }
///
/// // Equivalent to:
/// // pub struct MyEnumVariantsData {
/// //     pub unit_enum: (),
/// //     pub tuple_enum: TupleEnumVariantType,
/// //     pub struct_enum: StructEnumVariantType,
/// // }
/// //
/// // pub struct TupleEnumVariantType(pub i32, pub String);
/// //
/// // pub struct StructEnumVariantType {
/// //     pub id: u32,
/// //     pub name: String,
/// // }
/// ```
///
/// ## Helper attributes
///
/// ### `#[variants_data_struct(<meta>)]` customizes the behavior of the derive macro.
///
/// The `<meta>` (see [`VariantsDataStructAttrMeta`](crate::variants_data_struct_attr_meta::VariantsDataStructAttrMeta))
/// is a comma-separated list that can contain the following items:
///
// - `attrs(#[derive(...)] ...)`: Adds the specified attributes to the generated data struct. Notably, you
/// can use it to add derives like `Debug`, `Clone` to the generated struct.
/// - `vis = <visibility>`: Specifies a custom visibility for the generated data struct. If not provided,
/// the visibility of the original enum is used.
/// - `name = <CustomName>`: Specifies a custom name for the generated data struct.
///  If not provided, the default name is `<EnumName>VariantsData`.
/// - `variants_tys_attrs(#[derive(...)] ...)`: Adds the specified attributes to each of the generated variant type structs.
/// Notably, you can use it to add derives like `Debug`, `Clone` to the generated variant type structs.
///
/// ### `#[variants_data_struct_field(<meta>)]` customizes the behavior of individual fields in the generated data struct
/// and their corresponding variant types.
///
/// The `<meta>` (see [`VariantsDataStructFieldAttrMeta`](crate::variants_data_struct_field_attr_meta::VariantsDataStructFieldAttrMeta))
/// is a comma-separated list that can contain the following items:
///
/// - `field_attrs(#[derive(...)] ...)`: Adds the specified attributes to the generated field in the data struct.
/// Notably, you can use it to add derives like `Debug`, `Clone` to
/// the generated field.
/// - `field_vis = <visibility>`: Specifies a custom visibility for the generated field in the data struct. If not provided,
/// the visibility of the generated data struct is used.
/// - `field_name = <custom_field_name>`: Specifies a custom name for the generated field in the data struct. If not provided,
/// the name is derived from the original variant's name (converted to `snake_case`).
/// - `field_ty_override`: Overrides the type of the generated field in the data struct. If not provided,
/// the type is derived from the original variant's fields. For variants without fields (a unit variant or a struct or tuple variant with no fields),
/// the type is `()`. For tuple and struct variants, a separate "variant type" struct is generated to encapsulate the fields.
/// - `gen_variant_ty`: Overrides the decision whether to generate a separate "variant type" struct for the variant.
/// If not provided, a "variant type" struct is generated for tuple and struct variants, and not for unit variants.
#[proc_macro_derive(
    VariantsDataStruct,
    attributes(variants_data_struct, variants_data_struct_field)
)]
pub fn derive_variants_data_struct(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let syn::DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = input;
    let syn::Data::Enum(enum_data) = data else {
        return syn::Error::new_spanned(
            ident,
            concat!(
                stringify!(VariantsDataStruct),
                " can only be derived for enums"
            ),
        )
        .to_compile_error()
        .into();
    };

    // Parse the `variants_data_struct` attribute meta
    let variants_data_struct_attr_meta: VariantsDataStructAttrMeta =
        match VariantsDataStructAttrMeta::from_attrs(attrs).map(Option::unwrap_or_default) {
            Ok(meta) => meta,
            Err(err) => return err.to_compile_error().into(),
        };

    // Resolve the final metadata for the derived variants data struct
    let VariantsDataStructMeta {
        attrs: variants_data_struct_attrs,
        vis: variants_data_struct_vis,
        name: variants_data_struct_name,
        variants_tys_attrs,
    } = VariantsDataStructMeta::resolve(variants_data_struct_attr_meta, &ident, &vis);

    // Generate the variants data struct definitions
    let VariantsDataStructDefs {
        derived_struct,
        variant_type_structs,
    } = match variants_data_struct_defs(
        variants_data_struct_attrs,
        variants_tys_attrs,
        variants_data_struct_vis,
        variants_data_struct_name,
        generics,
        enum_data.variants,
    ) {
        Ok(defs) => defs,
        Err(err) => return err.to_compile_error().into(),
    };

    quote::quote! {
        #derived_struct

        #(#variant_type_structs)*
    }
    .into()
}
