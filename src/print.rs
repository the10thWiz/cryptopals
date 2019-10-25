#![allow(dead_code)]
use crate::random;
use termion::{color};

pub fn print(i: usize, found: u32, actual: u32) {
    println!(
        "{:03}\t{}",
        i,
        format_u32(found, random::B & (random::B << random::S))
    );
    println!(
        "{}\t{}",
        found == actual,
        format_u32(actual, random::B & (random::B << random::S))
    );
    println!();
}

fn format_u32(num: u32, mask: u32) -> String {
    let mut ret = String::default();
    let mut normal = true;
    for i in (0..32).rev() {
        let tmp_mask = 1 << i;
        if mask & tmp_mask != 0 {
            if normal {
                ret.push_str("\x1B[32m");
                normal = false;
            }
        } else {
            if !normal {
                ret.push_str("\x1B[31m");
                normal = true;
            }
        }
        if num & tmp_mask != 0 {
            ret.push('1');
        } else {
            ret.push('0');
        }
    }
    ret.push_str("\x1B[39m");
    ret
}

// pub fn diff(one: &str, two: &str) -> String {
//     let mut ret = String::default();
//     let mut normal = true;
//     for c in one.chars().zip(two.chars()) {
//         if c.0 == c.1 {
//             if normal {
//                 normal = false;
//                 ret += &color::Fg(color::Red).to_string();
//             }
//         } else {
//             if !normal {
//                 normal = true;
//                 ret += &color::Fg(color::Green).to_string();
//             }
//         }
//         ret.push(c.0);
//     }
//     ret
// }

pub fn diff_hex<T: std::fmt::UpperHex>(a: T, b: T) -> String {
    let one = format!("{:X}", a);
    let two = format!("{:X}", b);
    let mut ret = String::default();
    let mut normal = true;
    for c in one.chars().zip(two.chars()) {
        if c.0 == c.1 {
            if normal {
                normal = false;
                ret += &color::Fg(color::Red).to_string();
            }
        } else {
            if !normal {
                normal = true;
                ret += &color::Fg(color::Green).to_string();
            }
        }
        ret.push(c.0);
    }
    ret
}

pub fn diff_bin<T: std::fmt::Binary>(a: T, b: T) -> String {
    let one = format!("{:032b}", a);
    let two = format!("{:032b}", b);
    let mut ret = String::default();
    let mut normal = true;
    ret += &color::Fg(color::Red).to_string();
    for c in one.chars().zip(two.chars()) {
        if c.0 == c.1 {
            if normal {
                normal = false;
                ret += &color::Fg(color::Red).to_string();
            }
        } else {
            if !normal {
                normal = true;
                ret += &color::Fg(color::Green).to_string();
            }
        }
        ret.push(c.0);
    }
    ret
}

pub fn mask_bin<T: std::fmt::Binary>(a: T, b: T) -> String {
    let one = format!("{:032b}", a);
    let two = format!("{:032b}", b);
    let mut ret = String::default();
    let mut normal = true;
    ret += &color::Fg(color::Red).to_string();
    for c in one.chars().zip(two.chars()) {
        if c.1 == '0' {
            if normal {
                normal = false;
                ret += &color::Fg(color::Red).to_string();
            }
        } else {
            if !normal {
                normal = true;
                ret += &color::Fg(color::Green).to_string();
            }
        }
        ret.push(c.0);
    }
    ret
}

pub fn diff<T>(left: T, right: T) -> PrintFmt<T> {
    PrintFmt {output:FmtType::Diff, left, right}
}

pub fn mask<T>(left: T, right: T) -> PrintFmt<T> {
    PrintFmt {output:FmtType::Mask, left, right}
}

pub enum FmtType {
    Diff, Mask
}

pub struct PrintFmt<T> {
    output: FmtType,
    left: T,
    right: T
}
use std::fmt::*;

impl<T> PrintFmt<T> {
    fn diff(one: String, two: String) -> String {
        let mut ret = String::default();
        let mut normal = true;
        ret += &color::Fg(color::Red).to_string();
        for c in one.chars().zip(two.chars()) {
            if c.0 == c.1 {
                if normal {
                    normal = false;
                    ret += &color::Fg(color::Red).to_string();
                }
            } else {
                if !normal {
                    normal = true;
                    ret += &color::Fg(color::Green).to_string();
                }
            }
            ret.push(c.0);
        }
        ret
    }

    fn mask(one: String, two: String) -> String {
        let mut ret = String::default();
        let mut normal = true;
        ret += &color::Fg(color::Red).to_string();
        for c in one.chars().zip(two.chars()) {
            if c.1 == '0' {
                if normal {
                    normal = false;
                    ret += &color::Fg(color::Red).to_string();
                }
            } else {
                if !normal {
                    normal = true;
                    ret += &color::Fg(color::Green).to_string();
                }
            }
            ret.push(c.0);
        }
        ret
    }
}

impl<T: Display> Display for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let a = String::default();
        let b = format!("{:width$}", self.left, width = f.width().unwrap());
        
        Ok(())
    }
}
impl<T: UpperHex> UpperHex for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(())
    }
}
impl<T: LowerHex> LowerHex for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(())
    }
}
use std::fmt::Write;
impl<T: Binary> Binary for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.output {
            FmtType::Diff => write!(f, "{}", Self::diff(String::default(), String::default())),
            FmtType::Mask => write!(f, "{}", Self::mask(String::default(), String::default())),
        }
    }
}
impl<T: Debug> Debug for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let ret = String::default();
        
        Ok(())
    }
}
impl<T: Octal> Octal for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(())
    }
}
impl<T: UpperExp> UpperExp for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(())
    }
}
impl<T: LowerExp> LowerExp for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(())
    }
}
impl<T: Pointer> Pointer for PrintFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(())
    }
}
