#![cfg(test)]
#![allow(dead_code)]

use variants_data_struct::VariantsDataStruct;

#[derive(VariantsDataStruct)]

pub enum EnumA {
    UnitEnum,
    TupleEnum(i32, String),
    StructEnum { id: u32, name: String },
}

// Equivalent to:
//
// pub struct EnumAVariantsData {
//     pub unit_enum: (),
//     pub tuple_enum: TupleEnumVariantType,
//     pub struct_enum: StructEnumVariantType,
// }
// pub struct TupleEnumVariantType(pub i32, pub String);
//
// pub struct StructEnumVariantType {
//     pub id: u32,
//     pub name: String,
// }

#[derive(VariantsDataStruct)]
#[variants_data_struct(
    name = CustomName,
    attrs(
        #[derive(Debug, Clone)]
    ),
    variants_tys_attrs(
        #[derive(Debug, Clone)]
    )
)]
pub enum EnumB {
    A,
    B(f64),
    C { flag: bool },
}

// Equivalent to:
//
// #[derive(Debug, Clone)]
// pub struct CustomName {
//     pub a: (),
//     pub b: BVariantType,
//     pub c: CVariantType,
// }
// #[derive(Debug, Clone)]
// pub struct BVariantType(pub f64);
//
// #[derive(Debug, Clone)]
// pub struct CVariantType {
//     pub flag: bool,
// }

#[derive(VariantsDataStruct)]
#[variants_data_struct(
    vis = pub(crate),
    variants_tys_attrs(
        #[derive(Debug)]
    )
)]
pub enum EnumC {
    A,
    #[variants_data_struct_field(
        field_name = custom_b,
        variant_ty_name = BType,
        variant_ty_vis = pub(self),
        variant_ty_attrs(
            #[derive(Clone)]
        )
    )]
    B(f64),
    #[variants_data_struct_field(
        field_name = custom_c,
        variant_ty_name = CType,
        variant_ty_vis = pub(self),
        variant_ty_attrs(
            #[derive(Clone)]
        )
    )]
    C {
        flag: bool,
    },
}

// Equivalent to:
// pub(crate) struct EnumCVariantsData {
//     pub(crate) a: (),
//     pub(crate) custom_b: BType,
//     pub(crate) custom_c: CType,
// }
// #[derive(Clone, Debug)]
// pub(self) struct BType(pub(self) f64);
//
// #[derive(Clone, Debug)]
// pub(self) struct CType {
//     pub(self) flag: bool,
// }

#[test]
fn test_variants_data_struct() {
    let _data_struct = EnumAVariantsData {
        unit_enum: (),
        tuple_enum: TupleEnumVariantType(42, "Hello".to_string()),
        struct_enum: StructEnumVariantType {
            id: 1,
            name: "World".to_string(),
        },
    };
}
