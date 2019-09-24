use crate::data::Bytes;

/**
 * Convient iterator to iterate over consecutive `Bytes`
 */
pub struct KeyGen {
    cur: Bytes,
    done: bool,
}

impl Iterator for KeyGen {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        if self.done {
            return None;
        }
        let tmp = self.cur.clone();
        if self.cur.inc() {
            self.done = true;
        }
        Some(tmp)
    }
}

impl KeyGen {
    /**
     * Create new `KeyGen` with `size` bytes
     */
    pub fn new(size: usize) -> KeyGen {
        KeyGen {
            cur: Bytes::zero(size),
            done: false,
        }
    }
}
