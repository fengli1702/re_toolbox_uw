use std::fmt::Display;

use clap::{Args, Subcommand, ValueEnum};


#[derive(Subcommand, Debug)]
pub enum CompilerSubcommand {
    Get,
    Set(CompilerSetArgs),
    List,
    Compile(CompileArgs),
}

#[derive(Args, Debug)]
pub struct CompilerSetArgs {
    compiler: Compiler
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Compiler {
    Gcc,
    Clang
}

impl Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Gcc => "GCC",
            Self::Clang => "Clang",
        })?;
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct CompileArgs {
    flags: Option<String>
}