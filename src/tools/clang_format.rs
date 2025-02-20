use std::{io::Write as _, process::Command, str};

use anyhow::Result;
use tempfile;

pub fn run(contents: &Vec<u8>) -> Result<Vec<u8>> {
    let tdir = tempfile::tempdir()?;
    let mut tfile = tempfile::NamedTempFile::new()?;
    tfile.write_all(contents)?;

    let res = Command::new("clang-format").arg("--style").arg("Microsoft").arg(tfile.path()).output()?;
    if let Some(0) = res.status.code() {
        Ok(res.stdout.clone())
    } else {
        Ok(vec![])
    }
}