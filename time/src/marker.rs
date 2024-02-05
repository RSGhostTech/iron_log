use crate::LocalTime;

pub trait DateFormatter
{
    fn as_str(&self) -> &str;

    /// 如果格式化失败，
    /// 则会返回空字符串
    fn date_format(&self, time: &LocalTime) -> String
    {
        time.ref_local().format(self.as_str()).to_string()
    }
}

impl DateFormatter for &str
{
    #[inline]
    fn as_str(&self) -> &str
    {
        self
    }
}

impl DateFormatter for String
{
    #[inline]
    fn as_str(&self) -> &str
    {
        self.as_str()
    }
}

impl DateFormatter for &String
{
    #[inline]
    fn as_str(&self) -> &str
    {
        self
    }
}