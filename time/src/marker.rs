use crate::LocalTime;

pub trait DateFormatter
{
    fn date_format(&self,time: &LocalTime) -> String;
}