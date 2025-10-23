/// The [`syn::Attribute::meta`] for the `variants_data_struct` attribute.
///
/// For the resolved values (e.g., with defaults applied), see
/// [`VariantsDataStructMeta`](crate::variants_data_struct_meta::VariantsDataStructMeta).
pub(crate) struct VariantsDataStructAttrMeta {
    /// The attributes to be applied to the derived variants data struct.
    pub(crate) attrs: Vec<syn::Attribute>,
    /// The override for the visibility of the derived variants data struct.
    ///
    /// If not provided, the visibility of the original enum is used.
    pub(crate) vis: Option<syn::Visibility>,
    /// The override for the name of the derived variants data struct.
    ///
    /// If not provided, the default name is `<EnumName>VariantsData`.
    pub(crate) name: Option<syn::Ident>,
    /// The attributes to be applied to the "variant types",
    /// which are the generated types for the respective original enum's variants.
    pub(crate) variants_tys_attrs: Vec<syn::Attribute>,
}

impl Default for VariantsDataStructAttrMeta {
    fn default() -> Self {
        Self {
            attrs: vec![],
            vis: None,
            name: None,
            variants_tys_attrs: vec![],
        }
    }
}

impl VariantsDataStructAttrMeta {
    pub(crate) fn from_attrs(attrs: Vec<syn::Attribute>) -> syn::Result<Option<Self>> {
        let variants_data_struct_attr: syn::Attribute = match attrs
            .into_iter()
            .find(|attr| attr.path().is_ident("variants_data_struct"))
        {
            Some(attr) => attr,
            None => return Ok(None),
        };

        let variants_data_struct_attr_meta = variants_data_struct_attr.parse_args()?;
        Ok(Some(variants_data_struct_attr_meta))
    }
}

impl syn::parse::Parse for VariantsDataStructAttrMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut vis: Option<syn::Visibility> = None;
        let mut name: Option<syn::Ident> = None;
        let mut attrs: Vec<syn::Attribute> = vec![];
        let mut variants_tys_attrs: Vec<syn::Attribute> = vec![];

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if !lookahead.peek(syn::Ident) {
                return Err(lookahead.error());
            }
            let ident: syn::Ident = input.parse()?;
            let ident: String = ident.to_string();
            let ident: &str = ident.as_str();

            match ident {
                "vis" => {
                    let _: syn::Token![=] = input.parse()?;
                    let vis_value: syn::Visibility = input.parse()?;
                    vis = Some(vis_value);
                }
                "name" => {
                    let _: syn::Token![=] = input.parse()?;
                    let name_ident: syn::Ident = input.parse()?;
                    name = Some(name_ident);
                }
                "attrs" => {
                    let content;
                    let _paren_token = syn::parenthesized!(content in input);
                    attrs = content.call(syn::Attribute::parse_outer)?;
                }
                "variants_tys_attrs" => {
                    let content;
                    let _paren_token = syn::parenthesized!(content in input);
                    variants_tys_attrs = content.call(syn::Attribute::parse_outer)?;
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "unexpected identifier in variants_data_struct attribute",
                    ));
                }
            }

            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }

        Ok(VariantsDataStructAttrMeta {
            vis,
            variants_tys_attrs,
            name,
            attrs,
        })
    }
}
