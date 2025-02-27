use std::{fs, io::Write as _, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Args;

use crate::Workspace;

#[derive(Args, Debug)]
pub struct SaveArgs {
    /// Filepath to which contents should be saved
    path: PathBuf,

    /// Overwrite existing file, if present
    force: Option<bool>
}

impl SaveArgs {
    pub fn execute(&self, workspace: &mut Workspace) -> Result<()>{
        match &mut workspace.active_file {
            Some(ref mut workfile) => {
                let path = &self.path;
                match self.force {
                    Some(true) => {
                        if let Ok(mut f) = fs::File::create(path) {
                            f.write_all(&workfile.contents)?;
                            workfile.last_saved = self.path.clone();
                            workfile.modified_since_last_save = false;
                            Ok(())
                        } else {
                            Err(anyhow!("There was a problem saving the file"))
                        }
                    }
                    _ => {
                        match fs::File::create_new(path) {
                            Ok(mut f) => {
                                f.write_all(&workfile.contents)?;
                                workfile.last_saved = self.path.clone();
                                workfile.modified_since_last_save = false;
                                Ok(())
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                                Err(anyhow!("File already exists! Use force=true to overwrite existing file"))
                            }
                            Err(_) => {
                                Err(anyhow!("There was a problem saving the file"))
                            }
                        }
                    }
                }
            },
            None => Err(anyhow!("No current working file")),
        }
    }
}