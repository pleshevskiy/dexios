use crate::global::states::{HashMode, SkipMode};

use super::states::{Compression, DirectoryMode, EraseMode, EraseSourceDir, Key, PrintMode};

pub struct CryptoParams {
    pub hash_mode: HashMode,
    pub skip: SkipMode,
    pub erase: EraseMode,
    pub key: Key,
}

pub struct PackParams {
    pub dir_mode: DirectoryMode,
    pub print_mode: PrintMode,
    pub erase_source: EraseSourceDir,
    pub compression: Compression,
}
