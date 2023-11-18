pub mod disjoin;
pub mod join;

use std::string::FromUtf8Error;

pub type Flag = [u8; 4];

const STR_MAX: usize = 256;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct U8String {
    len: u32,
    buf: [u8; STR_MAX],
}
impl Default for U8String {
    fn default() -> Self {
        Self {
            len: 0,
            buf: [0; STR_MAX],
        }
    }
}

impl U8String {
    pub fn new(s: &str) -> Self {
        let mut buf = [0; STR_MAX];
        buf[..s.len()].copy_from_slice(s.as_bytes());
        Self {
            len: s.len() as u32,
            buf,
        }
    }
    pub fn to_string(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.buf[..self.len as usize].to_vec())
    }
}


/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_test() {

    }
} */
