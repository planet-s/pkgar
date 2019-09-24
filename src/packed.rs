//! The packed structs represent the on-disk format of pkgar

use core::mem;
use plain::Plain;

#[repr(packed)]
pub (crate) struct PackedHeader {
    /// NaCl signature of header data
    pub signature: [u8; 64],
    /// NaCl public key used to generate signature
    pub public_key: [u8; 32],
    /// SHA-256 sum of entry data
    pub sha256: [u8; 32],
    /// Count of Entry structs, which immediately follow
    pub count: u64,
}

unsafe impl Plain for PackedHeader {}

impl PackedHeader {
    /// Retrieve the size of the Header and its entries
    pub fn size(&self) -> Option<u64> {
        self.count
            .checked_mul(mem::size_of::<PackedEntry>() as u64)
            .and_then(|x| x.checked_add(mem::size_of::<PackedHeader>() as u64))
    }
}

#[repr(packed)]
pub (crate) struct PackedEntry {
    /// SHA-256 sum of the file data
    pub sha256: [u8; 32],
    /// Offset of file data in the data portion
    pub offset: u64,
    /// Size in bytes of the file data in the data portion
    pub size: u64,
    /// Unix permissions (user, group, other with read, write, execute)
    pub mode: u16,
    /// NUL-terminated relative path from extract directory
    pub path: [u8; 256],
}

unsafe impl Plain for PackedEntry {}

#[cfg(test)]
mod tests {
    use core::mem;

    use super::{PackedEntry, PackedHeader};

    #[test]
    fn header_size() {
        assert_eq!(mem::size_of::<PackedHeader>(), 136);
    }

    #[test]
    fn entry_size() {
        assert_eq!(mem::size_of::<PackedEntry>(), 306);
    }
}
