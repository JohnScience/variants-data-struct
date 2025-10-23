/// The [`syn::Attribute::meta`] for the `variants_data_struct_field` attribute.
///
/// For the resolved values (e.g., with defaults applied), see
/// [`VariantsDataStructFieldMeta`](crate::variants_data_struct_field_meta::VariantsDataStructFieldMeta).
pub(crate) struct VariantsDataStructFieldAttrMeta {
    pub(crate) field_attrs: Vec<syn::Attribute>,
    pub(crate) field_vis: Option<syn::Visibility>,
    pub(crate) field_name: Option<syn::Ident>,
    pub(crate) field_ty_override: Option<syn::Type>,
    pub(crate) gen_variant_ty: Option<bool>,
    pub(crate) variant_ty_attrs: Vec<syn::Attribute>,
    pub(crate) variant_ty_vis: Option<syn::Visibility>,
    pub(crate) variant_ty_name: Option<syn::Ident>,
}

impl Default for VariantsDataStructFieldAttrMeta {
    fn default() -> Self {
        Self {
            field_attrs: vec![],
            field_vis: None,
            field_name: None,
            field_ty_override: None,
            gen_variant_ty: None,
            variant_ty_attrs: vec![],
            variant_ty_vis: None,
            variant_ty_name: None,
        }
    }
}

impl VariantsDataStructFieldAttrMeta {
    pub(crate) fn from_attrs(attrs: &[syn::Attribute]) -> syn::Result<Option<Self>> {
        let variants_data_struct_field_attr: &syn::Attribute = match attrs
            .iter()
            .find(|attr| attr.path().is_ident("variants_data_struct_field"))
        {
            Some(attr) => attr,
            None => return Ok(None),
        };

        let variants_data_struct_field_meta = variants_data_struct_field_attr.parse_args()?;
        Ok(Some(variants_data_struct_field_meta))
    }
}

impl syn::parse::Parse for VariantsDataStructFieldAttrMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut field_attrs: Vec<syn::Attribute> = vec![];
        let mut field_vis: Option<syn::Visibility> = None;
        let mut field_name: Option<syn::Ident> = None;
        let mut field_ty_override: Option<syn::Type> = None;
        let mut gen_variant_ty: Option<bool> = None;
        let mut variant_ty_attrs: Vec<syn::Attribute> = vec![];
        let mut variant_ty_vis: Option<syn::Visibility> = None;
        let mut variant_ty_name: Option<syn::Ident> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if !lookahead.peek(syn::Ident) {
                return Err(lookahead.error());
            }
            let ident: syn::Ident = input.parse()?;
            let ident: String = ident.to_string();
            let ident: &str = ident.as_str();

            match ident {
                "field_attrs" => {
                    let content;
                    let _paren_token = syn::parenthesized!(content in input);
                    field_attrs = content.call(syn::Attribute::parse_outer)?;
                }
                "field_vis" => {
                    let _: syn::Token![=] = input.parse()?;
                    let vis_value: syn::Visibility = input.parse()?;
                    field_vis = Some(vis_value);
                }
                "field_name" => {
                    let _: syn::Token![=] = input.parse()?;
                    let name: syn::Ident = input.parse()?;
                    field_name = Some(name);
                }
                "field_ty_override" => {
                    let _: syn::Token![=] = input.parse()?;
                    let ty: syn::Type = input.parse()?;
                    field_ty_override = Some(ty);
                }
                "gen_variant_ty" => {
                    let _: syn::Token![=] = input.parse()?;
                    let gen_variant_ty_lit: syn::LitBool = input.parse()?;
                    gen_variant_ty = Some(gen_variant_ty_lit.value());
                }
                "variant_ty_attrs" => {
                    let content;
                    let _paren_token = syn::parenthesized!(content in input);
                    variant_ty_attrs = content.call(syn::Attribute::parse_outer)?;
                }
                "variant_ty_vis" => {
                    let _: syn::Token![=] = input.parse()?;
                    let val: syn::Visibility = input.parse()?;
                    variant_ty_vis = Some(val);
                }
                "variant_ty_name" => {
                    let _: syn::Token![=] = input.parse()?;
                    let name: syn::Ident = input.parse()?;
                    variant_ty_name = Some(name);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "unexpected identifier in variant_field attribute",
                    ));
                }
            }

            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }

        Ok(VariantsDataStructFieldAttrMeta {
            field_attrs,
            field_vis,
            field_name,
            field_ty_override,
            gen_variant_ty,
            variant_ty_attrs,
            variant_ty_vis,
            variant_ty_name,
        })
    }
}
