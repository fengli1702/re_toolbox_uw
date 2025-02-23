use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::{self, FromStr};

use anyhow::anyhow;
use clap::{Args, Error, FromArgMatches, Parser, Subcommand, ValueEnum};
use clap_repl::reedline::{
    DefaultPrompt, DefaultPromptSegment, FileBackedHistory,
};
use clap_repl::ClapEditor;

mod workspace;
use tools::clang_format;
use workspace::{Workspace, Workfile};

mod decompilers;
mod tools;

#[derive(Debug, Parser)]
#[command(name = "")] // This name will show up in clap's error messages, so it is important to set it to "".
enum Command {
    /// Print info about the working file
    Info,

    /// Load file contents into workspace
    Load(LoadArgs),

    /// Save workspace out to a file
    Save(SaveArgs),

    /// Print the current workspace contents
    Print,

    /// Decompiler related commands
    #[command(subcommand)]
    Decompiler(DecompilerSubcommand),
    
    /// Compiler related commands
    #[command(subcommand)]
    Compiler(CompilerSubcommand),

    /// Commands relating to tools other than compiler/decompiler
    #[command(subcommand)]
    Tool(ToolSubcommand),

    /// Exit the prompt
    Exit,
}

#[derive(Args, Debug)]
struct LoadArgs {
    /// Path to file to load
    filename: PathBuf,

    /// Format of file contents
    format: Format
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
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

#[derive(Args, Debug)]
struct SaveArgs {
    /// Filepath to which contents should be saved
    path: PathBuf,

    /// Overwrite existing file, if present
    force: Option<bool>
}

#[derive(Subcommand, Debug)]
enum DecompilerSubcommand {
    /// Display currently configured decompiler
    Get,

    /// Set current decompiler
    Set(DecompilerSetArgs),

    /// List available decompilers
    List,

    /// List functions present in binary, using current decompiler
    ListFunctions,

    /// Decompile function, using current decompiler
    Decompile(DecompileArgs),
}

#[derive(Args, Debug)]
struct DecompilerSetArgs {
    /// Decompiler to switch to
    decompiler: Decompiler
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Decompiler {
    Ghidra,
    Angr
}

impl Display for Decompiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Angr => "Angr",
            Self::Ghidra => "Ghidra",
        })?;
        Ok(())
    }
}

#[derive(Args, Debug)]
struct DecompileArgs {
    /// Function to decompile
    function: String
}

#[derive(Subcommand, Debug)]
enum CompilerSubcommand {
    Get,
    Set(CompilerSetArgs),
    List,
    Compile(CompileArgs),
}

#[derive(Args, Debug)]
struct CompilerSetArgs {
    compiler: Compiler
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Compiler {
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
struct CompileArgs {
    flags: Option<String>
}

#[derive(Subcommand, Debug)]
enum ToolSubcommand {
    /// List available tools
    List,

    /// Print details about tool,
    Details(ToolArgs),

    /// Execute tool
    Run(ToolArgs)
}

#[derive(Debug, Args)]
struct ToolArgs {
    tool: Tool
}

#[derive(Debug, Clone, Copy)]
struct Tool {

}

impl FromStr for Tool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl ValueEnum for Tool {
    fn value_variants<'a>() -> &'a [Self] {
        todo!()
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        todo!()
    }
}

// impl Display for Tool {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", match self {
//             Self::Bintoven => "Bintoven",
//             Self::ClangFormat => "ClangFormat",
//             Self::Comcat => "Comcat",
//             Self::Demangle => "Demangle"
//         })
//     }
// }

fn main() {
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("RE_toolbox".to_owned()),
        ..DefaultPrompt::default()
    };
    let rl = ClapEditor::<Command>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, "/tmp/clap-repl-simple-example-history".into())
                    .unwrap(),
            ))
        })
        .build();
    let mut workspace = Workspace::new();
    rl.repl(|command| {
        match command {
            Command::Info => handle_info(&workspace),
            Command::Load(args) => handle_load(&mut workspace, &args),
            Command::Save(args) => handle_save(&mut workspace, &args),
            Command::Print => handle_print(&workspace),
            Command::Decompiler(subcommand) => handle_decompiler(&mut workspace, &subcommand),
            Command::Compiler(subcommand) => handle_compiler(&mut workspace, &subcommand),
            Command::Tool(subcommand) => handle_tool(&mut workspace, &subcommand),
            Command::Exit => {
                println!("Bye!");
                return;
            }
            _ => todo!("New command?")
        }
    });
}

fn handle_info(workspace: &Workspace) {
    println!("----------------------------------------");
    println!("--------------Environment---------------");
    println!("Current decompiler: {}", workspace.decompiler);
    println!("Current compiler: {}", workspace.compiler);
    println!("");
    println!("----------------------------------------");
    println!("-------------Working File---------------");
    match &workspace.active_file {
        Some(file) => {
            println!("Filename: {}", file.filename);
            println!("Contents: {} bytes", file.contents.len());
            println!("Format: {}", file.format);
            println!("Loaded From: {}", file.loaded_from.display());
            println!("Last Saved: {}", file.last_saved.display());
            println!("Modified since load: {}", file.modified_since_load);
            println!("Modified since save: {}", file.modified_since_last_save);
        }
        None => println!("No file")
    }
    println!("")
}

fn handle_load(workspace: &mut Workspace, args: &LoadArgs) {
    if !args.filename.is_file() {
        println!("!! No such file !!");
    } else {
        if let Ok(workfile) = Workfile::new(args.filename.clone(), args.format) {
            println!("Changing working file to: {}", workfile.filename);
            workspace.active_file = Some(workfile);
        } else {
            println!("!! Problem reading file !!");
        }

    }
}

fn handle_save(workspace: &mut Workspace, args: &SaveArgs) {
    match &mut workspace.active_file {
        Some(ref mut workfile) => {
            let path = &args.path;
            match args.force {
                Some(true) => {
                    if let Ok(mut f) = File::create(path) {
                        f.write_all(&workfile.contents).unwrap();
                        workfile.last_saved = args.path.clone();
                        workfile.modified_since_last_save = false;
                    } else {
                        println!("!! There was a problem saving the file !!");
                    }
                }
                _ => {
                    match File::create_new(path) {
                        Ok(mut f) => {
                            f.write_all(&workfile.contents).unwrap();
                            workfile.last_saved = args.path.clone();
                            workfile.modified_since_last_save = false;
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                            println!("File already exists! Use force=true to overwrite existing file");
                        }
                        Err(_) => {
                            println!("!! There was a problem saving the file !!");
                        }
                    }
                }
            }
        },
        None => println!("!! No current working file !!"),
    }
}

fn handle_print(workspace: &Workspace) {
    match &workspace.active_file {
        Some(workfile) => {
            match workfile.format {
                Format::Bytecode => println!("Printing binary files is unsupported"),
                Format::Source => {
                    match str::from_utf8(&workfile.contents) {
                        Ok(contents) => println!("{}", contents),
                        Err(_) => println!("!! Source file is not valid UTF-8 !!"),
                    }
                }
            }
        }
        None => println!("!! No current working file !!"),
    }
}

fn handle_decompiler(workspace: &mut Workspace, subcommand: &DecompilerSubcommand) {
    match subcommand {
        DecompilerSubcommand::Get => {
            println!("Current decompiler {}", workspace.decompiler);
        },
        DecompilerSubcommand::Set(decompiler_set_args) => {
            workspace.decompiler = decompiler_set_args.decompiler;
            println!("Decompiler changed to {}", decompiler_set_args.decompiler);
        }
        DecompilerSubcommand::List => {
            println!("Available decompilers: ");
            println!("  {}", Decompiler::Angr);
            println!("  {}", Decompiler::Ghidra);
        }
        DecompilerSubcommand::ListFunctions => {
            if let Some(workfile) = &workspace.active_file {
                match workfile.format {
                    Format::Bytecode => {
                        println!("Running list_function from {}...", workspace.decompiler);

                        let function_names = match workspace.decompiler {
                            Decompiler::Ghidra => decompilers::ghidra::list_functions(&workfile.contents),
                            Decompiler::Angr => decompilers::angr::list_functions(&workfile.contents),
                        };

                        if let Ok(functions) = function_names {
                            for function in functions {
                                println!("  {}", function);
                            }
                        } else {
                            println!("!! Unexpected error occurred !!");
                        }
                    },
                    Format::Source => println!("!! Decompiler cannot be used on Source file !!"),
                }
            } else {
                println!("! No active file to decompile !");
            }

        },
        DecompilerSubcommand::Decompile(decompile_args) => {
            if let Some(workfile) = &mut workspace.active_file {
                match workfile.format {
                    Format::Bytecode => {
                        println!("Decompiling function {} with {}... ", decompile_args.function, workspace.decompiler);

                        let function_names = match workspace.decompiler {
                            Decompiler::Ghidra => decompilers::ghidra::decompile_function(&workfile.contents, &decompile_args.function),
                            Decompiler::Angr => {
                                println!("!! Decompiling with Angr is not currently supported !!");
                                Err(anyhow!("bad"))
                            }
                        };

                        if let Ok(source) = function_names {
                            println!("... Function decompiled");
                            println!("Replacing working file with new content");

                            workfile.format = Format::Source;
                            workfile.contents = source;
                            workfile.modified_since_load = true;
                            workfile.modified_since_last_save = true;
                        } else {
                            println!("!! Unexpected error occurred !!");
                        }
                    },
                    Format::Source => println!("!! Decompiler cannot be used on Source file !!"),
                }
            } else {
                println!("! No active file to decompile !");
            }

        },
    }
}

fn handle_compiler(workspace: &mut Workspace, subcommand: &CompilerSubcommand) {
    match subcommand {
        CompilerSubcommand::Get => todo!(),
        CompilerSubcommand::Set(compiler_set_args) => todo!(),
        CompilerSubcommand::List => todo!(),
        CompilerSubcommand::Compile(compile_args) => todo!(),
    }
}

fn handle_tool(workspace: &mut Workspace, subcommand: &ToolSubcommand) {
    // match subcommand {
    //     ToolSubcommand::List => 
    //         {
    //             println!("Available tools:");
    //             println!("  {}", Tool::Bintoven);
    //             println!("  {}", Tool::ClangFormat);
    //             println!("  {}", Tool::Comcat);
    //             println!("  {}", Tool::Demangle);
    //         }
    //     ToolSubcommand::Run(run_args) => {
    //         if let Some(workfile) = &mut workspace.active_file {
    //             match run_args.tool {
    //                 Tool::Bintoven => todo!(),
    //                 Tool::ClangFormat => {
    //                     match workfile.format {
    //                         Format::Bytecode => println!("! This tool cannot be run on bytecode !"),
    //                         Format::Source => {
    //                             println!("Running tool ...");
    //                             let output = clang_format::run(&workfile.contents);
    //                             if let Ok(source) = output {
    //                                 println!("... Tool completed");
    //                                 println!("Replacing working file with new content");
        
    //                                 workfile.format = Format::Source;
    //                                 workfile.contents = source;
    //                                 workfile.modified_since_load = true;
    //                                 workfile.modified_since_last_save = true;
    //                             } else {
    //                                 println!("!! Unexpected error occurred !!");
    //                             }
    //                         },
    //                     }
    //                 }
    //                 Tool::Comcat => todo!(),
    //                 Tool::Demangle => todo!(),
    //             }
    //         } else {
    //             println!("!! No working file to run tool on !!");
    //         }
    //     },
    // }
}