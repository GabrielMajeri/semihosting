use super::{File, OpenMode};
use crate::error::*;

/// Support for optional extensions of the semihosting interface.
pub struct Extensions([u8; MAX_FEATURES_LEN]);

impl Extensions {
    /// Opens the special file which contains the list of available extensions.
    pub fn open() -> Result<Extensions> {
        let mut f = File::open(":semihosting-features\0", OpenMode::ReadBinary)?;

        // Read and check for the correct magic bytes
        let mut magic = [0; 4];
        f.read(&mut magic);

        if magic != MAGIC {
            return Err(());
        }

        // Read the rest of the feature bytes
        let mut feature_bytes = [0; MAX_FEATURES_LEN];
        f.read(&mut feature_bytes);

        Ok(Extensions(feature_bytes))
    }

    /// Checks if an extension, defined by the a bit in a byte, is supported.
    ///
    /// If the supported extensions file is not long enough to even contain a
    /// byte, it means all the features in that byte are not supported.
    pub fn is_supported(&self, byte: usize, bit: usize) -> bool {
        let exts = self.0;
        let byte = exts.get(byte).unwrap_or(&0);
        let bit = byte & (1 << bit);
        bit != 0
    }
}

/// Magic number at the beginning of the file.
const MAGIC: [u8; 4] = [0x53, 0x48, 0x47, 0x42];

/// Only two extensions are defined for now, and they sit in the first byte.
const MAX_FEATURES_LEN: usize = 1;
