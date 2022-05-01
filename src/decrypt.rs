use std::{fs::File, io::{BufReader, Read, Write}};
use aes_gcm::{Key, Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use anyhow::{Result, Ok, Context};
use std::num::NonZeroU32;
use crate::structs::*;
use crate::misc_functions::*;

pub fn decrypt_file(input: &str, output: &str, keyfile: &str) -> Result<()> {
    let mut use_keyfile = false;
    if !keyfile.is_empty() { use_keyfile = true; }

    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let data_json: DexiosFile = serde_json::from_reader(&mut reader)?; // error = invalid input file

    let raw_key;
    if !use_keyfile { // if we're not using a keyfile, read from stdin
        let mut input = String::new();
        print!("Enter your password: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input).context("Error reading from stdin")?;
        raw_key = strip_newline(&input).as_bytes().to_vec();
    } else {
        let file = File::open(input).context("Error opening keyfile")?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new(); // our file bytes
        reader.read_to_end(&mut buffer).context("Error reading keyfile")?;
        raw_key = buffer.clone();
    }

    let mut key = [0u8; 32];
    let salt = base64::decode(data_json.salt)?; // error = error decoding salt b64
    ring::pbkdf2::derive(ring::pbkdf2::PBKDF2_HMAC_SHA512, NonZeroU32::new(256).unwrap(), &salt, &raw_key, &mut key);

    let nonce_bytes = base64::decode(data_json.nonce)?; // error = error decoding nonce b64
    let nonce = Nonce::from_slice(nonce_bytes.as_slice());
    let cipher_key = Key::from_slice(key.as_slice());
    let cipher = Aes256Gcm::new(cipher_key);
    let encrypted_bytes = base64::decode(data_json.data)?; // error = error decoding data b64
    let decrypted_bytes = cipher.decrypt(nonce, encrypted_bytes.as_slice()).unwrap();
    
    let mut writer = File::create(output)?; // add error handling (e.g. can't create file)
    writer.write_all(&decrypted_bytes)?; // error = unable to write to output file

    Ok(())
}