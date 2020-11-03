use super::Bytes;
use std::fmt::{Alignment, Debug, Display, Formatter, LowerHex, Result, UpperHex};

impl Bytes {
    fn fmt_tmp(s: String, mult: usize, f: &mut Formatter) -> Result {
        if let Some(width) = f.width() {
            let mut i = 0;
            if s.len() > width * mult {
                for c in s.chars() {
                    write!(f, "{}", c)?;
                    i += 1;
                    if i >= width * mult {
                        i = 0;
                        writeln!(f)?;
                    }
                }
            } else {
                if let Some(align) = f.align() {
                    match align {
                        Alignment::Left => {
                            write!(f, "{}", s)?;
                            for _ in 0..width * mult - s.len() {
                                write!(f, "{}", f.fill())?;
                            }
                        }
                        Alignment::Right => {
                            for _ in 0..width * mult - s.len() {
                                write!(f, "{}", f.fill())?;
                            }
                            write!(f, "{}", s)?;
                        }
                        Alignment::Center => {
                            for _ in 0..(width * mult - s.len()) / 2 {
                                write!(f, "{}", f.fill())?;
                            }
                            write!(f, "{}", s)?;

                            for _ in 0..(width * mult - s.len()) / 2 {
                                write!(f, "{}", f.fill())?;
                            }
                        }
                    }
                } else {
                    for _ in 0..width * 2 - s.len() {
                        write!(f, "{}", f.fill())?;
                    }
                    write!(f, "{}", s)?;
                }
            }
            Ok(())
        } else {
            write!(f, "{}", s)
        }
    }
    pub fn as_one(val: &[Bytes]) -> SliceFmt {
        SliceFmt(val)
    }
    pub fn list<'a>(val: &'a std::slice::Iter<Bytes>) -> List<'a> {
        List(val)
    }
}

impl Display for Bytes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Self::fmt_tmp(String::from(self.to_utf8()), 1, f)
    }
}
impl Debug for Bytes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Bytes [")?;
        Self::fmt_tmp(self.to_hex(), 2, f)?;
        write!(f, "]")
    }
}
impl UpperHex for Bytes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Self::fmt_tmp(self.to_hex(), 2, f)
    }
}
impl LowerHex for Bytes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Self::fmt_tmp(self.to_lower_hex(), 2, f)
    }
}

pub struct SliceFmt<'a>(&'a [Bytes]);
impl<'a> Display for SliceFmt<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for v in self.0 {
            Display::fmt(v, f)?
        }
        Ok(())
    }
}
impl<'a> UpperHex for SliceFmt<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for v in self.0 {
            UpperHex::fmt(v, f)?
        }
        Ok(())
    }
}
impl<'a> LowerHex for SliceFmt<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for v in self.0 {
            LowerHex::fmt(v, f)?
        }
        Ok(())
    }
}
impl<'a> Debug for SliceFmt<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for v in self.0 {
            Debug::fmt(v, f)?
        }
        Ok(())
    }
}

pub struct List<'a>(&'a std::slice::Iter<'a, Bytes>);
impl<'a> Display for List<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[")?;
        for v in self.0.as_ref() {
            Display::fmt(v, f)?;
            write!(f, ", ")?;
        }
        write!(f, "]")
    }
}
impl<'a> UpperHex for List<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[")?;
        for v in self.0.as_ref() {
            UpperHex::fmt(v, f)?;
            write!(f, ", ")?;
        }
        write!(f, "]")
    }
}
impl<'a> LowerHex for List<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[")?;
        for v in self.0.as_ref() {
            LowerHex::fmt(v, f)?;
            write!(f, ", ")?;
        }
        write!(f, "]")
    }
}
impl<'a> Debug for List<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[")?;
        for v in self.0.as_ref() {
            Debug::fmt(v, f)?;
            write!(f, ", ")?;
        }
        write!(f, "]")
    }
}
