use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub enum LogLevel
{
    Info,
    Warn,
    Error,
    Debug,
    Customize(&'static str)
}

impl Display for LogLevel
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            &LogLevel::Info => f.pad("INFO"),
            &LogLevel::Warn => f.pad("WARN"),
            &LogLevel::Error => f.pad("ERROR"),
            &LogLevel::Debug => f.pad("DEBUG"),
            &LogLevel::Customize(s) => f.pad(&s.to_uppercase())
        }
    }
}

pub struct LogLevelSettingsBuilder
{
    set:HashSet<String>
}

impl LogLevelSettingsBuilder
{
    pub fn new() -> Self
    {
        Self
        {
            set:HashSet::new()
        }
    }

    #[inline]
    pub fn insert(mut self,level:LogLevel) -> Self
    {
        self.set.insert(level.to_string());
        self
    }

    #[inline]
    pub fn build(self) -> LogLevelSetting
    {
        LogLevelSetting
        {
            hash:self.set
        }
    }
}

#[derive(Clone)]
pub struct LogLevelSetting
{
    hash:HashSet<String>
}

impl LogLevelSetting
{
    pub(crate) fn verification(level:)
}