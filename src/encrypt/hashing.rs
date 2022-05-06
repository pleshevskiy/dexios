use anyhow::{Ok, Result};

use crate::structs::DexiosFile;
use std::io::Write;

pub fn hash_data_blake3(data: DexiosFile) -> Result<String> {
    let mut hasher = blake3::Hasher::new();
    serde_json::to_writer(hasher.by_ref(), &data)?;
    let hash = hasher.finalize().to_hex().to_string();
    Ok(hash)
}