mod init;

use std::fs;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    Init,
    Version,
    Delete,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cli = Cli::parse();
    
    let subcommand = cli.command.unwrap_or(Subcommands::Version);

    match subcommand {
        Subcommands::Init => {
            init::run();
        }
        Subcommands::Version => {
            println!("Version {VERSION}");
        }
        Subcommands::Delete => {
            let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
            let tit_dir = working_dir.join(core::TIT_DIR);
            fs::remove_dir_all(tit_dir).expect("Failed to remove tit dir!");
        }
    }
}
