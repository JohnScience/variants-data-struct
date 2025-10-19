use convert_case::Casing as _;

use convert_case::Case;
use proc_macro::TokenStream;
use syn::{Token, punctuated::Punctuated};

struct VariantsDataStructAttrMeta {
    struct_name: Option<syn::Ident>,
    attrs: Vec<syn::Attribute>,
    variants_tys_attrs: Vec<syn::Attribute>,
}

impl VariantsDataStructAttrMeta {
    fn from_attrs(attrs: Vec<syn::Attribute>) -> syn::Result<Option<Self>> {
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
        let mut struct_name: Option<syn::Ident> = None;
        let mut attrs: Vec<syn::Attribute> = vec![];
        let mut variants_tys_attrs: Vec<syn::Attribute> = vec![];

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if !lookahead.peek(syn::Ident) {
                return Err(lookahead.error());
            }
            let ident: syn::Ident = input.parse()?;

            if ident == "name" {
                let _: syn::Token![=] = input.parse()?;
                let name: syn::Ident = input.parse()?;
                struct_name = Some(name);
            } else if ident == "attrs" {
                let content;
                let _paren_token = syn::parenthesized!(content in input);
                attrs = content.call(syn::Attribute::parse_outer)?;
            } else if ident == "variants_tys_attrs" {
                let content;
                let _paren_token = syn::parenthesized!(content in input);
                variants_tys_attrs = content.call(syn::Attribute::parse_outer)?;
            } else {
                return Err(syn::Error::new_spanned(
                    ident,
                    "unexpected identifier in variants_data_struct attribute",
                ));
            }
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }

        Ok(VariantsDataStructAttrMeta {
            variants_tys_attrs,
            struct_name,
            attrs,
        })
    }
}

struct VariantsDataStructDefs {
    derived_struct: syn::ItemStruct,
    variant_type_structs: Vec<syn::ItemStruct>,
}

enum FieldType {
    Unit(syn::TypeTuple),
    TupleStructType {
        def: syn::ItemStruct,
        ty: syn::TypePath,
    },
    ProperStructType {
        def: syn::ItemStruct,
        ty: syn::TypePath,
    },
}

fn unit_type() -> syn::TypeTuple {
    let span: proc_macro2::extra::DelimSpan = {
        let group = proc_macro2::Group::new(
            proc_macro2::Delimiter::Brace,
            proc_macro2::TokenStream::new(),
        );
        group.delim_span()
    };
    syn::TypeTuple {
        paren_token: syn::token::Paren { span },
        elems: Punctuated::new(),
    }
}

fn field_type(
    variant: syn::Variant,
    _enum_generics: &syn::Generics,
    // The visibility for the generated variant type struct and its fields, if they are going to be defined
    vis: syn::Visibility,
    variants_tys_attrs: &[syn::Attribute],
) -> FieldType {
    let syn::Variant {
        attrs: _,
        ident,
        fields,
        discriminant: _,
    } = variant;
    match fields {
        syn::Fields::Unit => FieldType::Unit(unit_type()),
        syn::Fields::Unnamed(fields_unnamed) => {
            let syn::FieldsUnnamed {
                paren_token,
                mut unnamed,
            } = fields_unnamed;

            if unnamed.len() == 0 {
                return FieldType::Unit(unit_type());
            }

            unnamed.pairs_mut().for_each(|pair| {
                let field = pair.into_tuple().0;
                field.vis = vis.clone();
            });

            let variant_struct_ident =
                syn::Ident::new(format!("{ident}VariantType").as_str(), ident.span());

            // TODO: handle generics properly
            let variant_type_generics = syn::Generics::default();

            let item_struct = syn::ItemStruct {
                attrs: Vec::from(variants_tys_attrs),
                vis,
                struct_token: syn::token::Struct { span: ident.span() },
                ident: variant_struct_ident.clone(),
                generics: variant_type_generics,
                fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                    paren_token,
                    unnamed,
                }),
                semi_token: None,
            };

            let type_path = syn::TypePath {
                qself: None,
                path: syn::Path::from(variant_struct_ident),
            };

            FieldType::TupleStructType {
                def: item_struct,
                ty: type_path,
            }
        }
        syn::Fields::Named(field_named) => {
            let syn::FieldsNamed {
                brace_token,
                mut named,
            } = field_named;

            if named.len() == 0 {
                return FieldType::Unit(unit_type());
            }

            named.pairs_mut().for_each(|pair| {
                let field = pair.into_tuple().0;
                field.vis = vis.clone();
            });

            let variant_struct_ident =
                syn::Ident::new(format!("{ident}VariantType").as_str(), ident.span());

            // TODO: handle generics properly
            let variant_type_generics = syn::Generics::default();

            let item_struct = syn::ItemStruct {
                attrs: Vec::from(variants_tys_attrs),
                vis,
                struct_token: syn::token::Struct { span: ident.span() },
                ident: variant_struct_ident.clone(),
                generics: variant_type_generics,
                fields: syn::Fields::Named(syn::FieldsNamed { brace_token, named }),
                semi_token: None,
            };

            let type_path = syn::TypePath {
                qself: None,
                path: syn::Path::from(variant_struct_ident),
            };

            FieldType::ProperStructType {
                def: item_struct,
                ty: type_path,
            }
        }
    }
}

fn variants_data_struct_defs(
    attrs: Vec<syn::Attribute>,
    variants_tys_attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    struct_name: syn::Ident,
    generics: syn::Generics,
    variants: Punctuated<syn::Variant, Token![,]>,
) -> VariantsDataStructDefs {
    let fields = variants.into_iter().map(|variant| {
        let field_ident = syn::Ident::new(
            variant
                .ident
                .to_string()
                .from_case(Case::Pascal)
                .to_case(Case::Snake)
                .as_str(),
            variant.ident.span(),
        );
        let field_type: FieldType =
            field_type(variant, &generics, vis.clone(), &variants_tys_attrs);
        (field_ident, field_type)
    });

    let mut variant_type_structs: Vec<syn::ItemStruct> = vec![];
    let mut struct_fields: Vec<syn::Field> = vec![];

    for (field_ident, field_type) in fields {
        match field_type {
            FieldType::Unit(unit_type) => {
                struct_fields.push(syn::Field {
                    attrs: vec![],
                    mutability: syn::FieldMutability::None,
                    vis: vis.clone(),
                    ident: Some(field_ident),
                    colon_token: Some(syn::token::Colon {
                        spans: [proc_macro2::Span::call_site()],
                    }),
                    ty: syn::Type::Tuple(unit_type),
                });
            }
            FieldType::TupleStructType { def, ty } => {
                variant_type_structs.push(def);
                struct_fields.push(syn::Field {
                    attrs: vec![],
                    mutability: syn::FieldMutability::None,
                    vis: vis.clone(),
                    ident: Some(field_ident),
                    colon_token: Some(syn::token::Colon {
                        spans: [proc_macro2::Span::call_site()],
                    }),
                    ty: syn::Type::Path(ty),
                });
            }
            FieldType::ProperStructType { def, ty } => {
                variant_type_structs.push(def);
                struct_fields.push(syn::Field {
                    attrs: vec![],
                    mutability: syn::FieldMutability::None,
                    vis: vis.clone(),
                    ident: Some(field_ident),
                    colon_token: Some(syn::token::Colon {
                        spans: [proc_macro2::Span::call_site()],
                    }),
                    ty: syn::Type::Path(ty),
                });
            }
        }
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
        vis,
        struct_token: syn::token::Struct {
            span: struct_name.span(),
        },
        ident: struct_name,
        generics,
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace { span: delim_span },
            named: Punctuated::from_iter(struct_fields),
        }),
        semi_token: None,
    };

    VariantsDataStructDefs {
        derived_struct,
        variant_type_structs,
    }
}

#[proc_macro_derive(VariantsDataStruct, attributes(variants_data_struct))]
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

    let variants_data_struct_attr_meta: VariantsDataStructAttrMeta =
        match VariantsDataStructAttrMeta::from_attrs(attrs) {
            Ok(Some(meta)) => meta,
            Ok(None) => VariantsDataStructAttrMeta {
                struct_name: None,
                variants_tys_attrs: vec![],
                attrs: vec![],
            },
            Err(err) => return err.to_compile_error().into(),
        };

    let struct_name: syn::Ident = match variants_data_struct_attr_meta.struct_name {
        Some(name) => name,
        None => proc_macro2::Ident::new(format!("{}VariantsData", ident).as_str(), ident.span()),
    };

    let VariantsDataStructDefs {
        derived_struct,
        variant_type_structs,
    } = variants_data_struct_defs(
        variants_data_struct_attr_meta.attrs,
        variants_data_struct_attr_meta.variants_tys_attrs,
        vis,
        struct_name,
        generics,
        enum_data.variants,
    );

    quote::quote! {
        #derived_struct

        #(#variant_type_structs)*
    }
    .into()
}
