use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, Args, Subcommand, ValueEnum};

use crate::Workspace;

use super::{Tool, TOOLS};

#[derive(Subcommand, Debug)]
pub enum ToolSubcommand {
    /// List available tools
    List,

    /// Print details about tool,
    Details(ToolArgs),

    /// Execute tool
    Run(ToolArgs)
}


impl ToolSubcommand {
    pub fn execute(&self, workspace: &mut Workspace) -> Result<()>{
        match self {
            Self::List => {
                for tool in TOOLS.get_or_init(|| Vec::new()) {
                    println!("\t{}", tool.name);
                }
                Ok(())
            }
            Self::Details(args) => {
                println!("{}", args.tool.details());
                Ok(())
            }
            Self::Run(ref args) => {
                args.tool.run(workspace)
            }
        }
    }
}

#[derive(Debug, Args)]
pub struct ToolArgs {
    tool: Tool
}

impl FromStr for Tool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for tool in TOOLS.get_or_init(|| Vec::new()) {
            if tool.name == s {
                return Ok(tool.clone())
            }
        }

        return Err(anyhow!("No such tool {}", s))
    }
}

impl ValueEnum for Tool {
    fn value_variants<'a>() -> &'a [Self] {
        &TOOLS.get_or_init(|| Vec::new())

    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(self.name.clone()))
    }
}