use miniz_oxide::deflate::compress_to_vec;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result, Write};
use std::path::Path;

use crate::Flag;

pub struct JoinBuilder {
    buf: Vec<u8>,
}

impl JoinBuilder {
    pub fn new<P: AsRef<Path>>(root_path: P) -> Result<Self> {
        std::fs::read(root_path).map(|buf| Self { buf })
    }
    pub fn add_file<P: AsRef<Path>>(
        mut self,
        f: Flag,
        path: P,
    ) -> Result<Self> {
        let current_pos = self.buf.len();
        let file_name = path
            .as_ref()
            .file_name()
            .ok_or(Error::new(ErrorKind::Other, "Parse File name failed"))?
            .to_str()
            .ok_or(Error::new(ErrorKind::Other, "Convert to str failed"))?;
        let file = compress_to_vec(&std::fs::read(&path)?, 6);
        self.buf.extend(f);
        self.buf.extend((current_pos as u64).to_le_bytes());
        self.buf.extend((file_name.len() as u32).to_le_bytes());
        self.buf.extend((file.len() as u64).to_le_bytes());
        self.buf.extend(file_name.as_bytes());
        self.buf.extend(file);
        Ok(self)
    }
    pub fn add_buf<T: Sized>(mut self, f: Flag, buf: &T) -> Result<Self> {
        let buf = unsafe {
            core::slice::from_raw_parts(
                (buf as *const T) as *const u8,
                core::mem::size_of::<T>(),
            )
        };
        let current_pos = self.buf.len();
        self.buf.extend(f);
        self.buf.extend((current_pos as u64).to_le_bytes());
        self.buf.extend(buf);
        Ok(self)
    }
    pub fn get_buf(self) -> Vec<u8> {
        self.buf
    }
    pub fn build<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        output_file.write_all(&self.buf)
    }
}
