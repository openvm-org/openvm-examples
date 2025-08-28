use std::{fs, path::PathBuf};

use openvm_stark_sdk::openvm_stark_backend::p3_field::PrimeField32;

pub enum OutputDest {
    JsonFile(PathBuf),
    Stdout,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("openvm error: {0}")]
    OpenvmSerde(#[from] openvm::serde::Error),
}

fn output_hex_bytes(hex_bytes: String, output_dest: OutputDest) -> Result<(), Error> {
    match output_dest {
        OutputDest::JsonFile(path_buf) => {
            let input = serde_json::json!({
                "input": [hex_bytes]
            });
            let input = serde_json::to_string(&input)?;
            fs::write(path_buf, input)?;
        }
        OutputDest::Stdout => println!("{}", hex_bytes),
    }

    Ok(())
}

pub fn hex_str_encode_bytes<T: serde::Serialize + ?Sized>(
    value: &T,
    output_dest: OutputDest,
) -> Result<(), Error> {
    let words: Vec<u32> = openvm::serde::to_vec(value)?;
    let bytes: Vec<u8> = words.into_iter().flat_map(|w| w.to_le_bytes()).collect();
    let hex_bytes = String::from("0x01") + &hex::encode(&bytes);
    output_hex_bytes(hex_bytes, output_dest)
}

pub fn hex_str_encode_field_elements<F: PrimeField32>(
    field_elements: &[F],
    output_dest: OutputDest,
) -> Result<(), Error> {
    let words = field_elements.iter().map(|fe| fe.as_canonical_u32());
    let bytes: Vec<u8> = words.flat_map(|w| w.to_le_bytes()).collect();
    let hex_bytes = String::from("0x02") + &hex::encode(&bytes);
    output_hex_bytes(hex_bytes, output_dest)
}
