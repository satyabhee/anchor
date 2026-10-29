#![cfg(feature = "convert")]

use anchor_lang_idl::{
    convert::{convert_idl, convert_idl_to_legacy},
    types::IdlTypeDefGeneric,
};

/// Legacy IDL with a type-generic struct and a struct with a const generic
/// used as an array length.
const LEGACY: &str = r#"{
  "version": "0.1.0",
  "name": "generics",
  "instructions": [],
  "types": [
    {
      "name": "Pair",
      "generics": ["T", "U"],
      "type": {
        "kind": "struct",
        "fields": [
          { "name": "first", "type": { "generic": "T" } },
          { "name": "second", "type": { "generic": "U" } }
        ]
      }
    },
    {
      "name": "Buffer",
      "generics": ["T", "N"],
      "type": {
        "kind": "struct",
        "fields": [
          { "name": "items", "type": { "option": { "genericLenArray": [{ "generic": "T" }, "N"] } } }
        ]
      }
    }
  ],
  "metadata": { "address": "Generics11111111111111111111111111111111111" }
}"#;

fn generics_of(idl: &anchor_lang_idl::types::Idl, name: &str) -> Vec<IdlTypeDefGeneric> {
    idl.types
        .iter()
        .find(|ty| ty.name == name)
        .unwrap()
        .generics
        .clone()
}

#[test]
fn legacy_type_generics_are_kept() {
    let idl = convert_idl(LEGACY.as_bytes()).unwrap();
    assert_eq!(
        generics_of(&idl, "Pair"),
        vec![
            IdlTypeDefGeneric::Type { name: "T".into() },
            IdlTypeDefGeneric::Type { name: "U".into() },
        ]
    );
}

#[test]
fn legacy_array_len_generics_become_const() {
    let idl = convert_idl(LEGACY.as_bytes()).unwrap();
    assert_eq!(
        generics_of(&idl, "Buffer"),
        vec![
            IdlTypeDefGeneric::Type { name: "T".into() },
            IdlTypeDefGeneric::Const {
                name: "N".into(),
                ty: "usize".into(),
            },
        ]
    );
}

#[test]
fn legacy_generics_round_trip() {
    let idl = convert_idl(LEGACY.as_bytes()).unwrap();
    let legacy = convert_idl_to_legacy(&idl).unwrap();

    let actual: serde_json::Value = serde_json::from_slice(&legacy).unwrap();
    let expected: serde_json::Value = serde_json::from_str(LEGACY).unwrap();
    assert_eq!(actual["types"], expected["types"]);
}
