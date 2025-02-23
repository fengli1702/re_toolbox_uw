use std::{fs::{File, FileType}, io::Read as _, path::PathBuf};

use walkdir::{IntoIter, WalkDir};
use anyhow::{anyhow, Result};
use yaml_rust2::yaml;

use crate::Format;

pub mod binthoven;
pub mod clang_format;
pub mod demangle;

pub struct Tool {
    name: String,
    description: String,
    input: Format,
    output: Option<Format>,
    command: String,

}

impl Tool {
    fn new(name: &str, description: &str, input: Format, output: Option<Format>, command: &str) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            input,
            output,
            command: command.to_owned()
        }
    }
}

fn load_tools(directory: &PathBuf) {
    for file in yaml_iter(directory) {
        load_tool(&file);
    }
}

fn load_tool(path: &PathBuf) -> Result<Tool>{
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let docs = yaml::YamlLoader::load_from_str(&s)?;
    let doc = &docs[0];

    let name = doc["name"].as_str().ok_or_else(|| anyhow!("ahh"))?;
    let description = doc["description"].as_str().ok_or_else(|| anyhow!("ahh"))?;
    let input_type = doc["input_type"].as_str().ok_or_else(|| anyhow!("ahh"))?;
    let input_type = format_from_str(input_type)?;
    let output_type = doc["output_type"].as_str().ok_or_else(|| anyhow!("ahh"))?;
    let output_type = format_from_str(output_type)?;
    let command = doc["command"].as_str().ok_or_else(|| anyhow!("ahh"))?;

    Ok(Tool::new(name, description, input_type, Some(output_type), command))
}

fn format_from_str(s: &str) -> Result<Format> {
    match s.to_lowercase().as_str() {
        "source" => Ok(Format::Source),
        "byte" => Ok(Format::Bytecode),
        "bytecode" => Ok(Format::Bytecode),
        _ => Err(anyhow!("Invalid format"))
    }
}

fn yaml_iter(directory: &PathBuf) -> std::iter::Filter<std::iter::Map<std::iter::FilterMap<IntoIter, impl FnMut(Result<walkdir::DirEntry, walkdir::Error>) -> Option<walkdir::DirEntry>>, impl FnMut(walkdir::DirEntry) -> PathBuf>, impl FnMut(&PathBuf) -> bool> {
    WalkDir::new(directory).into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| {
            if let Some(ext) = p.extension().and_then(|ext| Some(ext.to_ascii_lowercase())) {
                match ext.to_str() {
                    Some("yml") => true,
                    Some("yaml") => true,
                    _ => false
                }
            } else {
                false
            }
        })
}