use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub trait ToFileObject
{
    fn into_file(self) -> io::Result<File>;
}

impl ToFileObject for &Path
{
    #[inline]
    fn into_file(self) -> io::Result<File>
    {
        OpenOptions::new()
            .append(true)
            .create_new(true)
            .open(self)
    }
}

impl ToFileObject for File
{
    #[inline]
    fn into_file(self) -> io::Result<File>
    {
        Ok(self)
    }
}

impl ToFileObject for &str
{
    #[inline]
    fn into_file(self) -> io::Result<File>
    {
        let path = Path::new(self);
        path.into_file()
    }
}

impl ToFileObject for &String
{
    #[inline]
    fn into_file(self) -> io::Result<File>
    {
        let path = Path::new(self);
        path.into_file()
    }
}