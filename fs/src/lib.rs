#![allow(private_bounds)]
#![allow(private_interfaces)]
#![allow(dead_code)]
mod builder;
pub use builder::*;

use std::fs::File;
use std::io::{self, BufWriter, Write};

pub struct FileStream
{
    io:BufWriter<File>
}

impl FileStream
{
    pub fn new<T:ToFileObject>(t:T) -> io::Result<Self>
    {
        let file = t.into_file()?;
        let io = BufWriter::new(file);
        Ok
            (
                Self
                {
                    io
                }
            )
    }
}

impl From<BufWriter<File>> for FileStream
{
    #[inline]
    fn from(value: BufWriter<File>) -> Self
    {
        Self
        {
            io:value
        }
    }
}

impl From<File> for FileStream
{
    #[inline]
    fn from(value: File) -> Self
    {
        Self::new(value).unwrap()
    }
}

impl Write for FileStream
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        self.io.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()>
    {
        self.io.flush()
    }
}

impl FileStream
{
    #[inline]
    fn write_str<S>(&mut self,slice:S) -> io::Result<()>
    where
        S:AsRef<str>
    {
        let buf = slice.as_ref().as_bytes();
        self.write_all(buf)
    }

    #[inline]
    fn write_lines<S,L>(&mut self,lines:L) -> Result<(),usize>
    where
        S:AsRef<str>,
        L:Into<Vec<S>>
    {
        for (counter,s) in lines.into().iter().enumerate()
        {
            let s = s.as_ref();
            if self.write_str(s).is_err()
            {
                return Err(counter)
            }
        }

        Ok(())
    }
}

impl Drop for FileStream
{
    fn drop(&mut self)
    {
        self.flush().unwrap();
    }
}
