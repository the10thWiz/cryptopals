use crate::data::Bytes;

pub trait StreamCipher {
    /// Gets the next set of bytes from the stream cipher
    ///
    /// No particular size is necessary, but at least one
    /// byte should be returned each time. The Stream will
    /// call get_next until it has enough bytes to encrypt
    /// whatever data has been provided
    fn get_next(&mut self) -> Bytes;
}

pub struct Stream<C: StreamCipher>(C, Bytes);

impl<C: StreamCipher> Stream<C> {
    /// Creates a new stream cipher using the
    /// provided StreamCipher
    pub fn new(cipher: C) -> Self {
        Self(cipher, Bytes::new())
    }
    /// Encrypts (and decrypts) data using the cipher
    pub fn encrypt(&mut self, data: &Bytes) -> Bytes {
        while self.1.len() < data.len() {
            self.1 += self.0.get_next();
        }
        let mut data = data.clone();
        data ^= &self.1;
        self.1.truncate_start(data.len());
        data
    }
}

pub trait SeekableStreamCipher {
    /// Gets some bytes from the stream. They must be stable,
    /// i.e. get(n) must always return the same bytes for a
    /// given value of n.
    ///
    /// Returns (start, key_stream). start must be before
    /// location, and start + key_stream.len() must be
    /// after location
    ///
    /// No particular size is necessary, but at least one
    /// byte should be returned each time. The Stream will
    /// call get_next until it has enough bytes to encrypt
    /// whatever data has been provided
    fn get(&self, location: usize) -> (usize, Bytes);
}

pub struct SeekableStream<C: SeekableStreamCipher>(C);

impl<C: SeekableStreamCipher> SeekableStream<C> {
    /// Creates a new stream cipher using the
    /// provided StreamCipher
    pub fn new(cipher: C) -> Self {
        Self(cipher)
    }
    /// Encrypts (and decrypts) data using the cipher
    pub fn encrypt(&self, data: &Bytes, location: usize) -> Bytes {
        let (start, key_stream) = self.0.get(location);
        let mut key_stream = key_stream.truncate_start(location - start);
        while key_stream.len() < data.len() {
            let (_s, next) = self.0.get(start + key_stream.len());
            key_stream += next;
        }
        let mut data = data.clone();
        data ^= key_stream;
        data
    }
    /// Replaces the bytes in cipher with the bytes from new
    pub fn edit(&self, cipher: &mut Bytes, location: usize, new: &Bytes) {
        *cipher = cipher.replace(&self.encrypt(new, location)[..], location);
    }
}
