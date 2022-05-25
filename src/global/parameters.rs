// this file contains most of the enum's, structs and associated functions used throughout dexios
// it includes all of the parameters passed to cryptographic functions
// it also contains enums/structs relating to headers
// this file is long, but necessary

use anyhow::{Context, Result};
use clap::ArgMatches;
use std::fs::File;
use std::io::Write;

use super::SALT_LEN;

pub struct CryptoParams {
    pub hash_mode: HashMode,
    pub skip: SkipMode,
    pub bench: BenchMode,
    pub password: PasswordMode,
    pub erase: EraseMode,
    pub keyfile: KeyFile,
}

// the information needed to easily serialise a header
pub struct HeaderType {
    pub header_version: HeaderVersion,
    pub cipher_mode: CipherMode,
    pub algorithm: Algorithm,
}

pub enum HeaderVersion {
    V1,
}

// the data used returned after reading/deserialising a header
pub struct HeaderData {
    pub header_type: HeaderType,
    pub nonce: Vec<u8>,
    pub salt: [u8; SALT_LEN],
}

pub struct PackMode {
    pub dir_mode: DirectoryMode,
    pub hidden: HiddenFilesMode,
    pub exclude: Vec<String>,
    pub compression_level: i32,
    pub print_mode: PrintMode,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DirectoryMode {
    Singular,
    Recursive,
}

#[derive(PartialEq)]
pub enum HiddenFilesMode {
    Include,
    Exclude,
}

#[derive(PartialEq)]
pub enum PrintMode {
    Verbose,
    Quiet,
}

#[derive(PartialEq, Clone, Copy)]
pub enum EraseMode {
    EraseFile(i32),
    IgnoreFile(i32),
}

#[derive(PartialEq, Clone, Copy)]
pub enum HashMode {
    CalculateHash,
    NoHash,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SkipMode {
    ShowPrompts,
    HidePrompts,
}

#[derive(PartialEq, Copy, Clone)]
pub enum BenchMode {
    WriteToFilesystem,
    BenchmarkInMemory,
}

#[derive(PartialEq, Copy, Clone)]
pub enum PasswordMode {
    ForceUserProvidedPassword,
    NormalKeySourcePriority,
}

pub enum OutputFile {
    Some(File),
    None,
}

#[derive(PartialEq)]
pub enum KeyFile {
    Some(String),
    None,
}

pub enum HeaderFile {
    Some(String),
    None,
}

#[derive(Copy, Clone)]
pub enum Algorithm {
    AesGcm,
    XChaCha20Poly1305,
}

impl EraseMode {
    pub fn get_passes(self) -> i32 {
        match self {
            EraseMode::EraseFile(passes) => passes,
            EraseMode::IgnoreFile(_) => 0,
        }
    }
}

impl KeyFile {
    pub fn get_contents(&self) -> Result<String> {
        match self {
            KeyFile::Some(data) => Ok(data.to_string()),
            KeyFile::None => Err(anyhow::anyhow!(
                "Tried using a keyfile when one wasn't provided"
            )), // should never happen
        }
    }
}

impl OutputFile {
    pub fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match self {
            OutputFile::Some(file) => file.write_all(buf),
            OutputFile::None => Ok(()),
        }
    }
    pub fn flush(&mut self) -> std::io::Result<()> {
        match self {
            OutputFile::Some(file) => file.flush(),
            OutputFile::None => Ok(()),
        }
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Algorithm::AesGcm => write!(f, "AES-256-GCM"),
            Algorithm::XChaCha20Poly1305 => write!(f, "XChaCha20-Poly1305"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum CipherMode {
    // could do with a better name
    MemoryMode,
    StreamMode,
}

impl std::fmt::Display for CipherMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CipherMode::MemoryMode => write!(f, "memory mode"),
            CipherMode::StreamMode => write!(f, "stream mode"),
        }
    }
}

pub fn parameter_handler(sub_matches: &ArgMatches) -> Result<CryptoParams> {
    let keyfile = if sub_matches.is_present("keyfile") {
        KeyFile::Some(
            sub_matches
                .value_of("keyfile")
                .context("No keyfile/invalid text provided")?
                .to_string(),
        )
    } else {
        KeyFile::None
    };

    let hash_mode = if sub_matches.is_present("hash") {
        //specify to emit hash after operation
        HashMode::CalculateHash
    } else {
        // default
        HashMode::NoHash
    };

    let skip = if sub_matches.is_present("skip") {
        //specify to hide promps during operation
        SkipMode::HidePrompts
    } else {
        // default
        SkipMode::ShowPrompts
    };

    let erase = if sub_matches.is_present("erase") {
        let result = sub_matches
            .value_of("erase")
            .context("No amount of passes specified")?
            .parse();

        let passes = if let Ok(value) = result {
            value
        } else {
            println!("Unable to read number of passes provided - using the default.");
            16
        };
        EraseMode::EraseFile(passes)
    } else {
        EraseMode::IgnoreFile(0)
    };

    let bench = if sub_matches.is_present("bench") {
        //specify to not write to filesystem, for benchmarking and saving wear on hardware
        BenchMode::BenchmarkInMemory
    } else {
        // default
        BenchMode::WriteToFilesystem
    };

    let password = if sub_matches.is_present("password") {
        //Overwrite, so the user provided password is used and ignore environment supplied one?!
        PasswordMode::ForceUserProvidedPassword
    } else {
        // default
        PasswordMode::NormalKeySourcePriority
    };

    Ok(CryptoParams {
        hash_mode,
        skip,
        bench,
        password,
        erase,
        keyfile,
    })
}