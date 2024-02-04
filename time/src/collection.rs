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