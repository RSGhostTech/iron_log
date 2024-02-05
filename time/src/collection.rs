use crate::argument::DateFormattingArguments;
use crate::LocalTime;
use crate::marker::DateFormatter;

#[derive(Default, Clone)]
pub struct DateFormattingVec
{
    wrap:Vec<DateFormattingArguments>
}

impl DateFormattingVec
{
    #[inline]
    pub fn new() -> Self
    {
        Self::default()
    }

    #[inline]
    pub fn push<A>(&mut self,argument: A)
    where
        A:Into<DateFormattingArguments>
    {
        self.wrap.push(argument.into())
    }
}

impl DateFormatter for DateFormattingVec
{
    /// 此实现是无效的
    /// 因为它不需要使用as_str来获取参数
    #[inline]
    fn as_str(&self) -> &str {
        ""
    }

    #[inline]
    fn date_format(&self, time: &LocalTime) -> String {
        let mut parts = Vec::new();

        for args in self.wrap.iter()
        {
            parts.push(args.date_format(time))
        }

        parts.join("")
    }
}