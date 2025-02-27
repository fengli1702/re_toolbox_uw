use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Args;

use crate::{Format, Workfile, Workspace};

#[derive(Args, Debug)]
pub struct LoadArgs {
    /// Path to file to load
    filename: PathBuf,

    /// Format of file contents
    format: Format
}

impl LoadArgs {
    pub fn execute(&self, workspace: &mut Workspace) -> Result<()>{
        if !self.filename.is_file() {
            Err(anyhow!("No such file"))
        } else {
            if let Ok(workfile) = Workfile::new(self.filename.clone(), self.format) {
                println!("Changing working file to: {}", workfile.filename);
                workspace.active_file = Some(workfile);
                Ok(())
            } else {
                Err(anyhow!("Error reading file"))
            }

        }
    }
}