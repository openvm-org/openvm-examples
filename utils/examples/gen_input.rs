use std::path::PathBuf;

use openvm_example_utils::{
    hex_str_encode_bytes, hex_str_encode_field_elements, Error, OutputDest,
};
use openvm_stark_sdk::{
    openvm_stark_backend::p3_field::{Field, PrimeField32},
    p3_baby_bear::BabyBear,
};

fn generate_input_from_u64() -> Result<(), Error> {
    let input: u64 = 10;

    hex_str_encode_bytes(&input, OutputDest::Stdout)?;
    hex_str_encode_bytes(
        &input,
        OutputDest::JsonFile(PathBuf::from("input_u64.json")),
    )?;

    Ok(())
}

fn generate_input_from_struct() -> Result<(), Error> {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct ProgramInput {
        bytes: Vec<u8>,
        string: String,
        int: u32,
    }

    let input = ProgramInput {
        bytes: vec![1, 2, 3],
        string: "hello".to_string(),
        int: 123,
    };

    hex_str_encode_bytes(&input, OutputDest::Stdout)?;
    hex_str_encode_bytes(
        &input,
        OutputDest::JsonFile(PathBuf::from("input_struct.json")),
    )?;

    Ok(())
}

fn generate_input_from_field_elements<F: Field + PrimeField32>() -> Result<(), Error> {
    let input: Vec<F> = vec![
        F::from_canonical_u32(12),
        F::from_canonical_u32(13),
        F::from_canonical_u32(14),
    ];

    hex_str_encode_field_elements(&input, OutputDest::Stdout)?;
    hex_str_encode_field_elements(
        &input,
        OutputDest::JsonFile(PathBuf::from("input_field_elements.json")),
    )?;

    Ok(())
}

fn main() {
    generate_input_from_struct().unwrap();
    generate_input_from_u64().unwrap();
    generate_input_from_field_elements::<BabyBear>().unwrap();
}
