//! Byte order reading extensions for no_std_io::io::Read.
//!
//! This replaces the functionality of byteorder_lite::ReadBytesExt
//! for use with no_std_io's Read trait.

use no_std_io::io::{self, Read};

pub(crate) trait ReadBytesExt: Read {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16_le(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn read_u24_le(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 3];
        self.read_exact(&mut buf)?;
        Ok(u32::from(buf[0]) | (u32::from(buf[1]) << 8) | (u32::from(buf[2]) << 16))
    }

    fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl<R: Read + ?Sized> ReadBytesExt for R {}
