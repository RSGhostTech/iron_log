mod level;

use std::fmt::Display;
#[cfg(feature = "advanced_fs")]
use std::io::{self, Write};
#[cfg(feature = "advanced_fs")]
pub use fs;
#[cfg(feature = "advanced_time")]
pub use time;

pub use level::*;
#[cfg(feature = "advanced_time")]
pub use time::DateFormatter;

pub struct IronLogger
{
    iron:iron::IronLogger,
    #[cfg(feature = "advanced_time")]
    fmt:Option<String>,
    #[cfg(feature = "advanced_fs")]
    file:Option<fs::FileStream>
}

impl IronLogger
{
    pub fn new() -> iron::IronResult<Self>
    {
        let init = iron::init()?;
        Ok
            (
                Self
                {
                    iron : init,
                    #[cfg(feature = "advanced_time")]
                    fmt: None,
                    #[cfg(feature = "advanced_fs")]
                    file : None
                }
            )
    }

    #[cfg(feature = "advanced_time")]
    #[inline]
    pub fn time_fmt<F:DateFormatter>(&mut self,fmt:F) -> Option<String>
    {
        self.fmt.replace(fmt.as_str().to_string())
    }

    #[cfg(feature = "advanced_fs")]
    #[inline]
    pub fn file<F:fs::ToFileObject>(&mut self,f:F) -> io::Result<Option<fs::FileStream>>
    {
        let file = fs::FileStream::new(f)?;
        Ok(self.file.replace(file))
    }

    #[inline]
    pub fn free(self)
    {
        iron::free(self.iron);
        #[cfg(feature = "advanced_fs")]
        if let Some(mut f) = self.file
        {
            f.flush().unwrap();
        }
    }
}

#[allow(dead_code)]
impl IronLogger
{
    #[inline]
    fn display<D: Display>(&mut self,d:D)
    {
        self.iron.log(d)
    }

    #[cfg(feature = "advanced_time")]
    #[inline]
    fn _fmt(&self) -> Option<&str>
    {
        if let Some(fmt) = &self.fmt
        {
            return Some(fmt.as_str())
        }

        None
    }

    #[cfg(feature = "advanced_time")]
    #[inline]
    fn time_format(&self) -> Option<String>
    {
        let fmt = self._fmt()?;
        let local = time::LocalTime::new();
        Some(fmt.date_format(&local))
    }

    #[cfg(feature = "advanced_fs")]
    #[inline]
    fn _file(&mut self) -> Option<&mut fs::FileStream>
    {
        if let Some(file) = &mut self.file
        {
            return Some(file)
        }

        None
    }

    #[cfg(feature = "advanced_fs")]
    #[inline]
    fn file_write<S:AsRef<str>>(&mut self,s:S) -> Option<io::Result<()>>
    {
        let f = self._file()?;
        Some(f.write_str(s))
    }

    #[cfg(feature = "advanced_fs")]
    #[inline]
    fn file_write_lines<S,L>(&mut self,l:L) -> Option<Result<(),usize>>
    where
        S:AsRef<str>,
        L:Into<Vec<S>>
    {
        let f = self._file()?;
        Some(f.write_lines(l))
    }
}

impl IronLogger
{
    fn parameter<D: Display>(&self,data:&[D]) -> String
    {
        if data.is_empty()
        {
            return String::new();
        }

        let s = data.iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        format!(" {}",s)
    }

    #[inline]
    /// parameters : 补充的参数，如 执行者 等其他实现Display的信息，按照顺序写入
    pub fn log<D: Display>(&mut self,level:LogLevel,parameters:&[D],log:D) -> bool
    {
        let parameter = self.parameter(parameters);

        #[cfg(feature = "advanced_time")]
        let out =
            if let Some(time) = self.time_format()
            {
                format!("[{level} {time}{parameter}]{log}")
            }else {
                format!("[{level}{parameter}]{log}")
            };
        #[cfg(not(feature = "advanced_time"))]
        let out = format!("[{level}{parameter}]{log}");

        self.display(&out);

        #[cfg(feature = "advanced_fs")]
        if let Some(f) = self.file_write(out)
        {
            return f.is_ok()
        }
        true
    }

    #[inline]
    /// parameters : 补充的参数，如 执行者 等其他实现Display的信息，按照顺序写入
    pub fn log_lines<D: Display>(&mut self,level:LogLevel,parameters:&[D],mut log:Vec<D>) -> bool
    {
        if log.is_empty()
        {
            return true
        }

        if log.len() == 1
        {
            return self.log(level,parameters,log.into_iter().next().unwrap())
        }

        let parameter = self.parameter(parameters);
        let first = log.remove(0);

        #[cfg(feature = "advanced_time")]
        let first =
            if let Some(time) = self.time_format()
            {
                format!("[{level} {time}{parameter}]{first}")
            }else {
                format!("[{level}{parameter}]{first}")
            };

        #[cfg(not(feature = "advanced_time"))]
        let first = format!("[{level}{parameter}]{first}");

        self.display(&first);

        #[cfg(feature = "advanced_fs")]
        if let Some(result) = self.file_write(first)
        {
            if result.is_err()
            {
                return false
            }
        }

        let log = log.into_iter()
            .map(|d| format!("\t{d}"))
            .collect::<Vec<_>>();

        for i in log.iter()
        {
            self.display(i);
        }

        #[cfg(feature = "advanced_fs")]
        {
            if let Some(s) = self.file_write_lines(log)
            {
                return s.is_ok()
            }
        }

        true
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use super::*;

    #[test]
    fn test1()
    {
        let mut logger = IronLogger::new().unwrap();
        let _ = logger.file("C:\\a.txt");
        logger.log(LogLevel::INFO,&[],"你好");
    }
}