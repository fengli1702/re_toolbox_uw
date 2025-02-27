use std::fmt::Display;

use anyhow::anyhow;
use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    /// Compiled
    Bytecode,

    /// Not compiled
    Source
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Bytecode => "Bytecode",
            Self::Source => "Source",
        })?;
        Ok(())
    }
}

impl TryFrom<&str> for Format {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lowercase = value.to_lowercase();
        match lowercase.as_str() {
            "source" => Ok(Self::Source),
            "sourcecode" => Ok(Self::Source),
            "byte" => Ok(Self::Bytecode),
            "bytecode" => Ok(Self::Bytecode),
            "bytes" => Ok(Self::Bytecode),
            _ => Err(anyhow!("Invalid format {}", value))
        }
    }
}