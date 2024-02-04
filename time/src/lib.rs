mod argument;
mod marker;
mod collection;

use chrono::{DateTime, Local};

pub use marker::*;
pub use argument::*;
pub use collection::*;

#[derive(Copy, Clone)]
pub struct LocalTime
{
    time:DateTime<Local>
}

impl Default for LocalTime
{
    #[inline]
    fn default() -> Self
    {
        Self
        {
            time : Local::now()
        }
    }
}

impl LocalTime
{
    #[inline]
    pub fn new() -> Self
    {
        Self::default()
    }

    fn ref_local(&self) -> &DateTime<Local>
    {
        &self.time
    }

    #[inline]
    pub fn format<F:DateFormatter>(&self,f:F) -> String
    {
        f.date_format(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_test()
    {
        let mut format_arguments = DateFormattingVec::new();
        format_arguments.push(DateFormattingArgument::Year);
        format_arguments.push(":");
        format_arguments.push(DateFormattingArgument::Minute);
        let local = LocalTime::new();
        println!("{}",local.format(format_arguments));
    }
}