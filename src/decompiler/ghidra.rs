use std::{env, io::Write as _, process::Command, str};

use anyhow::Result;
use tempfile;

pub fn list_functions(contents: &Vec<u8>) -> Result<Vec<String>> {
    let tdir = tempfile::tempdir()?;
    let mut tfile = tempfile::NamedTempFile::new()?;
    tfile.write_all(contents)?;

    let ghidra_dir = env::var("GHIDRA_DIR")?;
    let command_path = ghidra_dir.clone() + "/ghidra_11.2.1_PUBLIC/support/analyzeHeadless";
    let script_path = ghidra_dir.clone() + "/ghidra_11.2.1_PUBLIC/Ghidra/Features/Base/ghidra_scripts";
    let local_script_path = ghidra_dir.clone() + "/scripts";

    let res = Command::new(command_path)
        .arg(tdir.path())
        .arg("project")
        .arg("-import")
        .arg(tfile.path())
        .arg("-scriptPath")
        .arg(script_path)
        .arg("-scriptPath")
        .arg(local_script_path)
        .arg("-postScript")
        .arg("ListFunctionsH.java")
        .arg("deleteProject")
        .output()?;

    if let Some(0) = res.status.code() {
        let output = str::from_utf8(&res.stdout)?;
        let lines = output.split('\n');

        let functions = lines
            .filter(|s| s.starts_with("INFO  ListFunctionsH.java>"))
            .map(|s| s.trim_start_matches("INFO  ListFunctionsH.java> "))
            .map(|s| s.trim_end_matches(" (GhidraScript)  "))
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        Ok(functions)
    } else {
        Ok(vec![])
    }
    
}

pub fn decompile_function(contents: &Vec<u8>, function_name: &str) -> Result<Vec<u8>> {
    let tdir = tempfile::tempdir()?;
    let mut tfile = tempfile::NamedTempFile::new()?;
    tfile.write_all(contents)?;

    let ghidra_dir = env::var("GHIDRA_DIR")?;
    let command_path = ghidra_dir.clone() + "/ghidra_11.2.1_PUBLIC/support/analyzeHeadless";
    let script_path = ghidra_dir.clone() + "/ghidra_11.2.1_PUBLIC/Ghidra/Features/Base/ghidra_scripts";
    let local_script_path = ghidra_dir.clone() + "/scripts";

    let res = Command::new(command_path)
        .arg(tdir.path())
        .arg("project")
        .arg("-import")
        .arg(tfile.path())
        .arg("-scriptPath")
        .arg(script_path)
        .arg("-scriptPath")
        .arg(local_script_path)
        .arg("-postScript")
        .arg("DecompileFunctionH.java")
        .arg(function_name)
        .arg("deleteProject")
        .output()?;

        let mut func = Vec::new();
        if let Some(0) = res.status.code() {
            let output = str::from_utf8(&res.stdout)?;
            let lines = output.split('\n');
            
            let mut in_decomp = false;
            for line in lines {
                if line.starts_with("INFO  DecompileFunctionH.java>") {
                    in_decomp = true;
                } else if line.contains("(GhidraScript)") {
                    break;
                } else if in_decomp {
                    func.extend(line.as_bytes());
                    func.push(b'\n');
                }
                
            }

        } 
        Ok(func)

}