use crate::variants_data_struct_field_attr_meta::VariantsDataStructFieldAttrMeta;

pub(crate) struct VariantTy {
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) vis: syn::Visibility,
    pub(crate) name: syn::Ident,
}

impl VariantTy {
    pub(crate) fn to_struct_def(self, mut fields: syn::Fields) -> syn::ItemStruct {
        let VariantTy { attrs, vis, name } = self;

        match fields {
            syn::Fields::Unit => (),
            syn::Fields::Named(ref mut named_fields) => {
                named_fields.named.iter_mut().for_each(|field| {
                    field.vis = vis.clone();
                });
            }
            syn::Fields::Unnamed(ref mut unnamed_fields) => {
                unnamed_fields.unnamed.iter_mut().for_each(|field| {
                    field.vis = vis.clone();
                });
            }
        }

        syn::ItemStruct {
            attrs,
            vis,
            struct_token: syn::token::Struct { span: name.span() },
            ident: name,
            generics: syn::Generics::default(),
            fields,
            semi_token: None,
        }
    }
}

/// The resolved metadata for a variant field, with defaults applied.
///
/// For the raw attribute meta (i.e., as parsed from the `variant_field` attribute), see
///
/// [`VariantsDataStructFieldAttrMeta`].
pub(crate) struct VariantsDataStructFieldMeta {
    pub(crate) field_attrs: Vec<syn::Attribute>,
    pub(crate) field_vis: syn::Visibility,
    pub(crate) field_name: syn::Ident,
    pub(crate) field_ty: syn::Type,
    pub(crate) variant_ty: Option<VariantTy>,
}

impl VariantsDataStructFieldMeta {
    pub(crate) fn resolve(
        attr_meta: VariantsDataStructFieldAttrMeta,
        variants_tys_attrs: &[syn::Attribute],
        variants_data_struct_vis: syn::Visibility,
        variant: &syn::Variant,
    ) -> VariantsDataStructFieldMeta {
        use convert_case::Casing as _;

        let VariantsDataStructFieldAttrMeta {
            field_attrs,
            field_vis,
            field_name,
            field_ty_override,
            gen_variant_ty,
            mut variant_ty_attrs,
            variant_ty_vis,
            variant_ty_name,
        } = attr_meta;

        variant_ty_attrs.extend_from_slice(variants_tys_attrs);

        let field_vis = match field_vis {
            Some(vis) => vis,
            None => variants_data_struct_vis.clone(),
        };

        let field_name = match field_name {
            Some(name) => name,
            None => syn::Ident::new(
                &variant
                    .ident
                    .to_string()
                    .from_case(convert_case::Case::Pascal)
                    .to_case(convert_case::Case::Snake)
                    .as_str(),
                variant.ident.span(),
            ),
        };

        let gen_variant_ty = match gen_variant_ty {
            Some(val) => val,
            None => match &variant.fields {
                syn::Fields::Unit => false,
                syn::Fields::Named(named_fields) => named_fields.named.len() > 0,
                syn::Fields::Unnamed(unnamed_fields) => unnamed_fields.unnamed.len() > 0,
            },
        };

        let variant_ty = if !gen_variant_ty {
            None
        } else {
            let variant_ty_vis = match variant_ty_vis {
                Some(vis) => vis,
                None => variants_data_struct_vis,
            };

            let variant_ty_name = match variant_ty_name {
                Some(name) => name,
                None => syn::Ident::new(
                    &format!("{}VariantType", variant.ident),
                    variant.ident.span(),
                ),
            };

            Some(VariantTy {
                attrs: variant_ty_attrs,
                vis: variant_ty_vis,
                name: variant_ty_name,
            })
        };

        let field_ty = match field_ty_override {
            Some(ty) => ty,
            None => match &variant_ty {
                Some(variant_ty) => syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path::from(variant_ty.name.clone()),
                }),
                None => match &variant.fields {
                    syn::Fields::Unit => syn::Type::Tuple(syn::TypeTuple {
                        paren_token: syn::token::Paren {
                            span: {
                                let group = proc_macro2::Group::new(
                                    proc_macro2::Delimiter::Parenthesis,
                                    proc_macro2::TokenStream::new(),
                                );
                                group.delim_span()
                            },
                        },
                        elems: syn::punctuated::Punctuated::new(),
                    }),
                    syn::Fields::Named(_named_fields) => {
                        panic!(
                            "field_ty_override is required for named fields if gen_variant_ty is false"
                        )
                    }
                    syn::Fields::Unnamed(unnamed_fields) => syn::Type::Tuple(syn::TypeTuple {
                        paren_token: syn::token::Paren {
                            span: {
                                let group = proc_macro2::Group::new(
                                    proc_macro2::Delimiter::Parenthesis,
                                    proc_macro2::TokenStream::new(),
                                );
                                group.delim_span()
                            },
                        },
                        elems: unnamed_fields
                            .unnamed
                            .iter()
                            .map(|f| f.ty.clone())
                            .collect(),
                    }),
                },
            },
        };

        VariantsDataStructFieldMeta {
            field_attrs,
            field_vis,
            field_name,
            field_ty,
            variant_ty,
        }
    }
}
