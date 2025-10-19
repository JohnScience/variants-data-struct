#![cfg(test)]
#![allow(dead_code)]

use variants_data_struct::VariantsDataStruct;

#[derive(VariantsDataStruct)]

pub enum MyEnum {
    UnitEnum,
    TupleEnum(i32, String),
    StructEnum { id: u32, name: String },
}

// Equivalent to:
//
// pub struct MyEnumVariantsData {
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
pub enum AnotherEnum {
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

#[test]
fn test_variants_data_struct() {
    let _data_struct = MyEnumVariantsData {
        unit_enum: (),
        tuple_enum: TupleEnumVariantType(42, "Hello".to_string()),
        struct_enum: StructEnumVariantType {
            id: 1,
            name: "World".to_string(),
        },
    };
}
