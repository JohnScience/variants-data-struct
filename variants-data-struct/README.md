# Derive data struct for enum

[![Crates.io](https://img.shields.io/crates/v/variants-data-struct)](https://crates.io/crates/variants-data-struct)
[![Downloads](https://img.shields.io/crates/d/variants-data-struct.svg)](https://crates.io/crates/variants-data-struct)
[![Documentation](https://docs.rs/variants-data-struct/badge.svg)](https://docs.rs/variants-data-struct)
[![License](https://img.shields.io/crates/l/variants-data-struct)](https://crates.io/crates/variants-data-struct)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/variants-data-struct/status.svg)](https://deps.rs/repo/github/JohnScience/variants-data-struct)

This crate provides `VariantsDataStruct` derive macro that generates a struct for an enum, where each field corresponds to a variant of the enum. Each field's type is a struct representing the data of that variant or `()` if the variant has no data.

## Example

```rust
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
```
