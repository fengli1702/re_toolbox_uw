use std::{env, io::Write as _, process::Command, str};

use anyhow::Result;
use tempfile;

pub fn list_functions(contents: &Vec<u8>) -> Result<Vec<String>> {
    let mut tfile = tempfile::NamedTempFile::new()?;
    tfile.write_all(contents)?;

    let angr_dir = env::var("ANGR_DIR")?;
    let command_path = angr_dir + "/list_functions.py";

    let res = Command::new(command_path)
        .arg(tfile.path())
        .output()?;

    if let Some(0) = res.status.code() {
        let output = str::from_utf8(&res.stdout)?;
        let lines = output.split('\n');

        let functions = lines
            .filter(|s| s.starts_with("!!> "))
            .map(|s| s.trim_start_matches("!!> "))
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        Ok(functions)
    } else {
        Ok(vec![])
    }
    
}

pub fn decompile_function(contents: &Vec<u8>, function_name: &str) -> Result<Vec<u8>> {
    todo!();
    // let tdir = tempfile::tempdir()?;
    // let mut tfile = tempfile::NamedTempFile::new()?;
    // tfile.write_all(contents)?;

    // let command_path = GHIDRA_DIR.to_owned() + "/ghidra_11.2.1_PUBLIC/support/analyzeHeadless";
    // let script_path = GHIDRA_DIR.to_owned() + "/ghidra_11.2.1_PUBLIC/Ghidra/Features/Base/ghidra_scripts";
    // let local_script_path = GHIDRA_DIR.to_owned() + "/scripts";

    // let res = Command::new(command_path)
    //     .arg(tdir.path())
    //     .arg("project")
    //     .arg("-import")
    //     .arg(tfile.path())
    //     .arg("-scriptPath")
    //     .arg(script_path)
    //     .arg("-scriptPath")
    //     .arg(local_script_path)
    //     .arg("-postScript")
    //     .arg("DecompileFunctionH.java")
    //     .arg(function_name)
    //     .arg("deleteProject")
    //     .output()?;

    //     let mut func = Vec::new();
    //     if let Some(0) = res.status.code() {
    //         let output = str::from_utf8(&res.stdout)?;
    //         let lines = output.split('\n');
            
    //         let mut in_decomp = false;
    //         for line in lines {
    //             if line.starts_with("INFO  DecompileFunctionH.java>") {
    //                 in_decomp = true;
    //             } else if line.contains("(GhidraScript)") {
    //                 break;
    //             } else if in_decomp {
    //                 func.extend(line.as_bytes());
    //                 func.push(b'\n');
    //             }
                
    //         }

    //     } 
    //     Ok(func)

}