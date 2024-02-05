use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum LogLevel
{
    INFO,
    WARN,
    ERROR,
    DEBUG,
    Custom(String)
}

impl Display for LogLevel
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        { 
            LogLevel::INFO => 
                f.pad("INFO"),
            LogLevel::WARN =>
                f.pad("WARN"),
            LogLevel::ERROR =>
                f.pad("ERROR"),
            LogLevel::DEBUG =>
                f.pad("DEBUG"),
            LogLevel::Custom(s) =>
                f.pad(s.to_uppercase().as_str())
        }
    }
}

pub fn new_level<S: AsRef<str>>(level:S) -> LogLevel
{
    match level.as_ref() 
    { 
        "INFO" => 
            LogLevel::INFO,
        "WARN" => 
            LogLevel::WARN,
        "ERROR" => 
            LogLevel::ERROR,
        "DEBUG" =>
            LogLevel::DEBUG,
        _ =>
            LogLevel::Custom(level.as_ref().to_string())
    }
}