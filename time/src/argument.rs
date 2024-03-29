#![allow(dead_code)]

use crate::LocalTime;
use crate::marker::DateFormatter;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum DateFormattingArgument
{
    Year,
    year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    NanoSecond
}

impl AsRef<str> for DateFormattingArgument
{
    #[inline]
    fn as_ref(&self) -> &'static str
    {
        match self
        {
            DateFormattingArgument::Year =>
                "%Y",
            DateFormattingArgument::year =>
                "%y",
            DateFormattingArgument::Month =>
                "%m",
            DateFormattingArgument::Day =>
                "%d",
            DateFormattingArgument::Hour =>
                "%H",
            DateFormattingArgument::Minute =>
                "%M",
            DateFormattingArgument::Second =>
                "%S",
            DateFormattingArgument::NanoSecond =>
                "%f"
        }
    }
}

impl DateFormatter for DateFormattingArgument
{
    #[inline]
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum DateFormattingArgumentExtends
{
    month,
    day,
    hour,
    minute,
    second
}

impl DateFormattingArgumentExtends
{
    #[inline]
    fn api(&self) -> DateFormattingArgument
    {
        match self
        {
            DateFormattingArgumentExtends::month =>
                DateFormattingArgument::Month,
            DateFormattingArgumentExtends::day =>
                DateFormattingArgument::Day,
            DateFormattingArgumentExtends::hour =>
                DateFormattingArgument::Hour,
            DateFormattingArgumentExtends::minute =>
                DateFormattingArgument::Minute,
            DateFormattingArgumentExtends::second =>
                DateFormattingArgument::Second
        }
    }
}

impl DateFormatter for DateFormattingArgumentExtends
{
    /// 此实现是无效的
    /// 因为它不需要使用as_str来获取参数
    #[inline]
    fn as_str(&self) -> &str {
        ""
    }

    fn date_format(&self, time: &LocalTime) -> String
    {
        let extends = self.api().date_format(time).parse::<u8>().unwrap();
        match self
        {
            DateFormattingArgumentExtends::hour => {
                let n = if extends >= 12
                {
                    extends - 12
                }
                else {
                    extends
                };
                n.to_string()
            },
            _ => extends.to_string()
        }
    }
}

pub type DateFormattingArgumentText = &'static str;

#[derive(Copy, Clone)]
pub enum DateFormattingArguments
{
    CrateAPI(DateFormattingArgument),
    Extends(DateFormattingArgumentExtends),
    Text(DateFormattingArgumentText)
}

impl DateFormatter for DateFormattingArguments
{
    /// 此实现是无效的
    /// 因为它不需要使用as_str来获取参数
    fn as_str(&self) -> &str {
        ""
    }

    #[inline]
    fn date_format(&self, time: &LocalTime) -> String {
        match self
        {
            DateFormattingArguments::CrateAPI(api) =>
                api.date_format(time),
            DateFormattingArguments::Extends(extends) =>
                extends.date_format(time),
            DateFormattingArguments::Text(t) =>
                t.to_string()
        }
    }
}

impl From<DateFormattingArgument> for DateFormattingArguments
{
    #[inline]
    fn from(value: DateFormattingArgument) -> Self
    {
        Self::CrateAPI(value)
    }
}

impl From<DateFormattingArgumentExtends> for DateFormattingArguments
{
    #[inline]
    fn from(value: DateFormattingArgumentExtends) -> Self
    {
        Self::Extends(value)
    }
}

impl From<DateFormattingArgumentText> for DateFormattingArguments
{
    #[inline]
    fn from(value: DateFormattingArgumentText) -> Self
    {
        Self::Text(value)
    }
}