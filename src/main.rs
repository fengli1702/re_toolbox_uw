use std::env;
use std::str;

use clap::Parser;
use clap_repl::reedline::{
    DefaultPrompt, DefaultPromptSegment, FileBackedHistory,
};
use clap_repl::ClapEditor;

mod workspace;

mod compiler;
mod decompiler;
mod file;
mod tool;
pub use file::Format;
pub use file::LoadArgs;
pub use file::SaveArgs;
pub use workspace::{Workfile, Workspace};
pub use decompiler::{Decompiler, DecompilerSubcommand};
pub use tool::ToolSubcommand;
pub use compiler::{Compiler, CompilerSubcommand};


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

fn main() {
    dotenv::dotenv().ok();

    if let Ok(plugin_path) = env::var("PLUGIN_PATH") {
        tool::load_tools(&plugin_path);
    } else {
        println!("WARN! No plugin path found");
    }

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
            Command::Load(args) => {
                if let Err(e) = args.execute(&mut workspace) {
                    println!("Error running command: {:?}", e);
                }
            },
            Command::Save(args) => {
                if let Err(e) = args.execute(&mut workspace) {
                    println!("Error running command: {:?}", e);
                }
            },
            Command::Print => handle_print(&workspace),
            Command::Decompiler(subcommand) => subcommand.execute(&mut workspace),
            Command::Compiler(subcommand) => handle_compiler(&mut workspace, &subcommand),
            Command::Tool(subcommand) => {
                if let Err(e) = subcommand.execute(&mut workspace) {
                    println!("Error running command: {:?}", e);
                }
            },
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


fn handle_compiler(workspace: &mut Workspace, subcommand: &CompilerSubcommand) {
    match subcommand {
        CompilerSubcommand::Get => todo!(),
        CompilerSubcommand::Set(compiler_set_args) => todo!(),
        CompilerSubcommand::List => todo!(),
        CompilerSubcommand::Compile(compile_args) => todo!(),
    }
}