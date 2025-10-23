use crate::variants_data_struct_field_attr_meta::VariantsDataStructFieldAttrMeta;
use crate::variants_data_struct_field_meta::VariantsDataStructFieldMeta;

pub(crate) struct VariantsDataStructDefs {
    pub(crate) derived_struct: syn::ItemStruct,
    pub(crate) variant_type_structs: Vec<syn::ItemStruct>,
}

struct VariantData {
    field_attrs: Vec<syn::Attribute>,
    field_vis: syn::Visibility,
    field_name: syn::Ident,
    field_ty: syn::Type,
    variant_ty_def: Option<syn::ItemStruct>,
}

/// Generates a field for the variants data struct.
fn variants_data_struct_field(
    // The attributes to be applied to the field.
    attrs: Vec<syn::Attribute>,
    // The visibility of the field.
    vis: syn::Visibility,
    // The name of the field.
    ident: syn::Ident,
    // The type of the field.
    ty: syn::Type,
) -> syn::Field {
    syn::Field {
        attrs,
        mutability: syn::FieldMutability::None,
        vis,
        ident: Some(ident),
        colon_token: Some(syn::token::Colon {
            spans: [proc_macro2::Span::call_site()],
        }),
        ty,
    }
}

/// Generates the variants data struct definitions, including
///
/// * the data variants struct itself and
/// * (oftentimes) the variant types.
pub(crate) fn variants_data_struct_defs(
    // The attributes to be applied to the derived variants data struct.
    attrs: Vec<syn::Attribute>,
    // The attributes to be applied to the "variant types",
    // which are the generated types for the respective original enum's variants.
    variants_tys_attrs: Vec<syn::Attribute>,
    // The visibility of the derived variants data struct.
    variants_data_struct_vis: syn::Visibility,
    // The name of the derived variants data struct.
    struct_name: syn::Ident,
    // The generics of the original enum,
    // meant to be used for inferring the generics of the "variant types".
    enum_generics: syn::Generics,
    // The variants of the original enum.
    variants: syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> syn::Result<VariantsDataStructDefs> {
    let variant_data_iter = variants.into_iter().map(|variant| {
        // Parse the `variant_field` attribute meta for the variant
        let variants_data_struct_field_attr_meta: VariantsDataStructFieldAttrMeta =
            VariantsDataStructFieldAttrMeta::from_attrs(&variant.attrs)?.unwrap_or_default();

        // Resolve the final metadata for the variant field and the variant type
        let VariantsDataStructFieldMeta {
            field_attrs,
            field_vis,
            field_name,
            field_ty,
            variant_ty,
        } = VariantsDataStructFieldMeta::resolve(
            variants_data_struct_field_attr_meta,
            &variants_tys_attrs,
            variants_data_struct_vis.clone(),
            &variant,
        );

        // Generate the variant type definition, if applicable
        let variant_ty_def = variant_ty.map(|variant_ty| variant_ty.to_struct_def(variant.fields));

        let variant_data = VariantData {
            field_attrs,
            field_vis,
            field_name,
            field_ty,
            variant_ty_def,
        };

        syn::Result::Ok(variant_data)
    });

    let mut variant_ty_defs: Vec<syn::ItemStruct> = vec![];
    let mut struct_fields: Vec<syn::Field> = vec![];

    for variant_data in variant_data_iter {
        let VariantData {
            field_attrs,
            field_name,
            field_vis,
            field_ty,
            variant_ty_def,
        } = variant_data?;

        if let Some(def) = variant_ty_def {
            variant_ty_defs.push(def);
        }

        let field =
            variants_data_struct_field(field_attrs, field_vis, field_name.clone(), field_ty);

        struct_fields.push(field);
    }

    let delim_span: proc_macro2::extra::DelimSpan = {
        let group = proc_macro2::Group::new(
            proc_macro2::Delimiter::Brace,
            proc_macro2::TokenStream::new(),
        );
        group.delim_span()
    };

    let derived_struct = syn::ItemStruct {
        attrs,
        vis: variants_data_struct_vis,
        struct_token: syn::token::Struct {
            span: struct_name.span(),
        },
        ident: struct_name,
        generics: enum_generics,
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace { span: delim_span },
            named: syn::punctuated::Punctuated::from_iter(struct_fields),
        }),
        semi_token: None,
    };

    Ok(VariantsDataStructDefs {
        derived_struct,
        variant_type_structs: variant_ty_defs,
    })
}
