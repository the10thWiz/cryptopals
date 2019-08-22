
use crate::data::Bytes;
use std::fs;
use std::io::Read;
use std::io::ErrorKind;

enum DataType {
    HEX,
    B64,
    UTF8
}

pub struct File {
    data: DataType,
    file: fs::File
}
impl File {
    pub fn read_hex_file(s:&str) -> File {
        File {data:DataType::HEX, file:fs::File::open(s).unwrap()}
    }
    pub fn read_64_file(s:&str) -> File {
        File {data:DataType::B64, file:fs::File::open(s).unwrap()}
    }
    pub fn read_utf8_file(s:&str) -> File {
        File {data:DataType::B64, file:fs::File::open(s).unwrap()}
    }
}

impl File {
    pub fn read_bytes(&mut self) -> Bytes {
        match &self.data {
            DataType::HEX => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => if n <= 0 {
                            return Bytes::read_hex(&ret[..]);
                        },
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return Bytes::read_hex(&ret[..]),
                            _ => panic!("Something happened: {:?}", e)
                        }
                    }
                    if buf[0] as char != '\n' {
                        ret.push(buf[0] as char);
                    }
                }
            },
            DataType::B64 => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => if n <= 0 {
                            return Bytes::read_64(&ret[..]);
                        },
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return Bytes::read_64(&ret[..]),
                            _ => panic!("Something happened: {:?}", e)
                        }
                    }
                    if buf[0] as char != '\n' {
                        ret.push(buf[0] as char);
                    }
                }
            },
            DataType::UTF8 => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => if n <= 0 {
                            return Bytes::read_utf8(&ret[..]);
                        },
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return Bytes::read_utf8(&ret[..]),
                            _ => panic!("Something happened: {:?}", e)
                        }
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
                        Ok(n) => if n <= 0 {
                            return None;
                        },
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return None,
                            _ => panic!("Something happened: {:?}", e)
                        }
                    }
                    if buf[0] as char == '\n' {
                        return Some(Bytes::read_hex(&ret[..]));
                    }else {
                        ret.push(buf[0] as char);
                    }
                }
            },
            DataType::B64 => {
                let mut ret = String::default();
                loop {
                    let mut buf = [0u8];
                    match self.file.read(&mut buf) {
                        Ok(n) => if n <= 0 {
                            return None;
                        },
                        Err(e) => match e.kind() {
                            ErrorKind::UnexpectedEof => return None,
                            _ => panic!("Something happened: {:?}", e)
                        }
                    }
                    if buf[0] as char == '\n' {
                        return Some(Bytes::read_64(&ret[..]));
                    }else {
                        ret.push(buf[0] as char);
                    }
                }
            }
        }
    }
}
