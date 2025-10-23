use crate::VariantsDataStructAttrMeta;

/// Metadata for deriving a variants data struct from an enum.
///
/// For the raw attribute meta (i.e., as parsed from the `variant_field` attribute), see
///
/// [`VariantsDataStructAttrMeta`].

pub(crate) struct VariantsDataStructMeta {
    /// The attributes to be applied to the derived variants data struct.
    pub(crate) attrs: Vec<syn::Attribute>,
    /// The visibility of the derived variants data struct.
    pub(crate) vis: syn::Visibility,
    /// The name of the derived variants data struct.
    pub(crate) name: syn::Ident,
    /// The attributes to be applied to the "variant types",
    /// which are the generated types for the respective original enum's variants.
    pub(crate) variants_tys_attrs: Vec<syn::Attribute>,
}

impl VariantsDataStructMeta {
    pub(crate) fn resolve(
        attr_meta: VariantsDataStructAttrMeta,
        enum_ident: &syn::Ident,
        enum_vis: &syn::Visibility,
    ) -> VariantsDataStructMeta {
        let VariantsDataStructAttrMeta {
            attrs,
            vis,
            name,
            variants_tys_attrs,
        } = attr_meta;

        let vis = match vis {
            Some(vis) => vis,
            None => enum_vis.clone(),
        };

        let name = match name {
            Some(name) => name,
            None => syn::Ident::new(&format!("{enum_ident}VariantsData"), enum_ident.span()),
        };

        VariantsDataStructMeta {
            attrs,
            vis,
            name,
            variants_tys_attrs,
        }
    }
}
