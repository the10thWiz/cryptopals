use super::Bytes;

impl From<u64> for Bytes {
    fn from(n: u64) -> Self {
        Self::from_bytes(&n.to_be_bytes())
    }
}

impl From<u32> for Bytes {
    fn from(n: u32) -> Self {
        Self::from_bytes(&n.to_be_bytes())
    }
}

impl From<u16> for Bytes {
    fn from(n: u16) -> Self {
        Self::from_bytes(&n.to_be_bytes())
    }
}

impl From<u8> for Bytes {
    fn from(n: u8) -> Self {
        Self::from_bytes(&n.to_be_bytes())
    }
}

impl From<&[u8]> for Bytes {
    fn from(n: &[u8]) -> Self {
        Self::from_bytes(n)
    }
}
