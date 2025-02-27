use std::{fs, io::{Read as _, Write as _}, path::{Path, PathBuf}, process::Command, str::FromStr};

use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, Args, Subcommand, ValueEnum};
use once_cell::sync::OnceCell;
use walkdir::{IntoIter, WalkDir};
use yaml_rust2::yaml;

use crate::{Format, Workfile, Workspace};

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
                println!("\tName: {}", args.tool.name);
                println!("\tDescription: {}", args.tool.description);
                println!("\tInput Format: {}", args.tool.input);
                println!("\tOutput Format: {:?}", args.tool.output);
                println!("\tCommand: '{}'", args.tool.command);
                Ok(())
            }
            Self::Run(ref args) => {
                match &mut workspace.active_file {
                    Some(workfile) if workfile.format == args.tool.input => {
                        let mut tfile = tempfile::NamedTempFile::new()?;
                        tfile.write_all(&workfile.contents)?;

                        let mut command = Command::new(&args.tool.command);
                        command.arg(tfile.path());
                        match command.output() {
                            Ok(resp) => {
                                println!("Tool completed with status: {:?}", resp.status);
                                let mut return_filename = String::from_utf8(resp.stdout)?;
                                return_filename.remove(return_filename.len()-1);

                                let mut f = fs::File::open(&return_filename)?;
                                let mut new_contents = Vec::new();
                                f.read_to_end(&mut new_contents)?;

                                match args.tool.output {
                                    Some(format) => {
                                        workfile.contents = new_contents;
                                        workfile.format = format;
                                        workfile.modified_since_load = true;
                                        workfile.modified_since_last_save = true;

                                    }
                                    None => {
                                        let output = String::from_utf8(new_contents)?;
                                        println!("{}", output);
                                    }
                                }
                                Ok(())
                            }
                            Err(e) => Err(anyhow!("Problem executing tool: {:?}", e))
                        }
                    }
                    Some(Workfile{ format, ..}) if format != &args.tool.input => {
                        Err(anyhow!("File format does not match required tool input"))
                    }
                    None => Err(anyhow!("No file to operate on")),
                    _ => Err(anyhow!{"Problem with the file format"})
                }
            }
        }
    }
}

#[derive(Debug, Args)]
pub struct ToolArgs {
    tool: Tool
}

static TOOLS: OnceCell<Vec::<Tool>> = OnceCell::new();

#[derive(Debug, Clone)]
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

impl FromStr for Tool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for tool in TOOLS.get_or_init(|| Vec::new()) {
            if tool.name == s{
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

pub fn load_tools(directory: &impl AsRef<Path>) {
    let mut tools = Vec::new();
    for file in yaml_iter(directory) {
        match load_tool(&file) {
            Ok(tool) => tools.push(tool),
            Err(e) => println!("WARN: tool failed to load {}", e)
        }
    }

    let _ = TOOLS.set(tools);
}

fn load_tool(path: &impl AsRef<Path>) -> Result<Tool>{
    let mut file = fs::File::open(path)?;
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

fn yaml_iter(directory: &impl AsRef<Path>) -> std::iter::Filter<std::iter::Map<std::iter::FilterMap<IntoIter, impl FnMut(Result<walkdir::DirEntry, walkdir::Error>) -> Option<walkdir::DirEntry>>, impl FnMut(walkdir::DirEntry) -> PathBuf>, impl FnMut(&PathBuf) -> bool> {
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