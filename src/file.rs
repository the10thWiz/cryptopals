use crate::data::Bytes;
use std::fs;
use std::io::ErrorKind;
use std::io::Read;

enum DataType {
    HEX,
    B64,
    UTF8,
}

/**
 * Convience struct for reading from data files
 *
 * Use `.read_data()` to read the file as one buffer
 *
 * Use as iterator to read lines as sperate buffers
 */
pub struct File {
    data: DataType,
    file: fs::File,
}

impl File {
    /**
     * Create a `File` for Hex data
     */
    pub fn read_hex_file(s: &str) -> File {
        File {
            data: DataType::HEX,
            file: fs::File::open(s).unwrap(),
        }
    }
    /**
     * Create a `File` for Base 64 data
     */
    pub fn read_64_file(s: &str) -> File {
        File {
            data: DataType::B64,
            file: fs::File::open(s).unwrap(),
        }
    }
    /**
     * Create a `File` for UTF-8 data
     */
    #[allow(dead_code)]
    pub fn read_utf8_file(s: &str) -> File {
        File {
            data: DataType::UTF8,
            file: fs::File::open(s).unwrap(),
        }
    }
}

impl File {
    /**
     * Reads data from file to buffer
     */
    pub fn read_bytes(&mut self) -> Bytes {
        match &self.data {
            DataType::HEX => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => {
                            if n <= 0 {
                                return Bytes::read_hex(&ret[..]);
                            }
                        }
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return Bytes::read_hex(&ret[..]),
                            _ => panic!("Something happened: {:?}", e),
                        },
                    }
                    if buf[0] as char != '\n' {
                        ret.push(buf[0] as char);
                    }
                }
            }
            DataType::B64 => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => {
                            if n <= 0 {
                                return Bytes::read_64(&ret[..]);
                            }
                        }
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return Bytes::read_64(&ret[..]),
                            _ => panic!("Something happened: {:?}", e),
                        },
                    }
                    if buf[0] as char != '\n' {
                        ret.push(buf[0] as char);
                    }
                }
            }
            DataType::UTF8 => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => {
                            if n <= 0 {
                                return Bytes::read_utf8(&ret[..]);
                            }
                        }
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return Bytes::read_utf8(&ret[..]),
                            _ => panic!("Something happened: {:?}", e),
                        },
                    }
                    if buf[0] as char != '\n' {
                        ret.push(buf[0] as char);
                    }
                }
            }
        }
    }
}

impl Iterator for File {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        match &self.data {
            DataType::HEX => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => {
                            if n <= 0 {
                                return None;
                            }
                        }
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return None,
                            _ => panic!("Something happened: {:?}", e),
                        },
                    }
                    if buf[0] as char == '\n' {
                        return Some(Bytes::read_hex(&ret[..]));
                    } else {
                        ret.push(buf[0] as char);
                    }
                }
            }
            DataType::B64 => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => {
                            if n <= 0 {
                                return None;
                            }
                        }
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return None,
                            _ => panic!("Something happened: {:?}", e),
                        },
                    }
                    if buf[0] as char == '\n' {
                        return Some(Bytes::read_64(&ret[..]));
                    } else {
                        ret.push(buf[0] as char);
                    }
                }
            }
            DataType::UTF8 => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => {
                            if n <= 0 {
                                return None;
                            }
                        }
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return None,
                            _ => panic!("Something happened: {:?}", e),
                        },
                    }
                    if buf[0] as char == '\n' {
                        return Some(Bytes::read_utf8(&ret[..]));
                    } else {
                        ret.push(buf[0] as char);
                    }
                }
            }
        }
    }
}
