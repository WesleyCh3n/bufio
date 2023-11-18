use miniz_oxide::inflate::decompress_to_vec;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result, Write};
use std::path::Path;

use crate::Flag;

pub struct DisJoinBuilder<P: AsRef<Path>> {
    buf: Vec<u8>,
    extract_path: P,
}

impl<P: AsRef<Path>> DisJoinBuilder<P> {
    pub fn new(root_path: P, extract_path: P) -> Result<Self> {
        std::fs::create_dir_all(&extract_path)?;
        Ok(Self {
            buf: std::fs::read(root_path)?,
            extract_path,
        })
    }
    fn find_flag(&self, f: &Flag) -> Option<usize> {
        let range = |i| (i + f.len())..(i + f.len() + 8);
        self.buf
            .windows(f.len())
            .enumerate()
            .position(|(i, window)| {
                window == f && (i as u64).to_le_bytes() == self.buf[range(i)]
            })
            .map(|pos| pos + f.len() + 8)
    }
    pub fn extract_file(self, f: Flag) -> Result<Self> {
        let found_index = self
            .find_flag(&f)
            .ok_or(Error::new(ErrorKind::NotFound, "Flag not found"))?;
        let file_name_len =
            u32::from_le_bytes(slice_as_u32_bytes(&self.buf[found_index..])?);
        let buf_len = u64::from_le_bytes(slice_as_u64_bytes(
            &self.buf[found_index + 4..][..8],
        )?);
        let file_name = std::str::from_utf8(
            &self.buf[found_index + 12..][..file_name_len as usize],
        )
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        let _ = decompress_to_vec(
            &self.buf[found_index + 12 + file_name_len as usize..]
                [..buf_len as usize],
        )
        .map(|buf| {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(self.extract_path.as_ref().join(file_name))?;
            file.write_all(&buf)
        })
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        Ok(self)
    }

    pub fn extract_buf<T: Sized + Copy>(
        self,
        f: Flag,
        buf: &mut T,
    ) -> Result<Self> {
        let found_index = self
            .find_flag(&f)
            .ok_or(Error::new(ErrorKind::NotFound, "Flag not found"))?;
        *buf = unsafe {
            *(self.buf[found_index..][..core::mem::size_of::<T>()].as_ptr()
                as *mut T)
        };
        Ok(self)
    }

    pub fn try_extract_buf<T: Sized + Copy>(
        &self,
        f: Flag,
        buf: &mut T,
    ) -> Option<()> {
        let found_index = self.find_flag(&f)?;
        *buf = unsafe {
            *(self.buf[found_index..][..core::mem::size_of::<T>()].as_ptr()
                as *mut T)
        };
        Some(())
    }
    pub fn finish(&self, cleanup: bool) -> Result<()> {
        if cleanup {
            std::fs::remove_dir_all(self.extract_path.as_ref())?;
        }
        Ok(())
    }
}

fn slice_as_u32_bytes(slice: &[u8]) -> Result<[u8; 4]> {
    slice.try_into().map_err(|_| {
        Error::new(ErrorKind::Other, "Cannot convert slice to u32 bytes")
    })
}

fn slice_as_u64_bytes(slice: &[u8]) -> Result<[u8; 8]> {
    slice.try_into().map_err(|_| {
        Error::new(ErrorKind::Other, "Cannot convert slice to u64 bytes")
    })
}
