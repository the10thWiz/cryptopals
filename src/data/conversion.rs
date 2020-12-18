use super::Bytes;

impl std::iter::FromIterator<u8> for Bytes {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Self {
            bytes: Vec::from_iter(iter),
        }
    }
}

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

impl Into<String> for Bytes {
    fn into(self) -> String {
        String::from_utf8(self.bytes).unwrap_or(String::new())
    }
}

impl AsRef<str> for Bytes {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(&self.bytes[..]).unwrap_or("|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||")
    }
}
