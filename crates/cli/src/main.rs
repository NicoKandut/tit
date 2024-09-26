use std::fs;

use clap::{Parser, Subcommand};

mod commit;
mod commits;
mod init;
mod remote;
mod download;
mod upload;

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
    Commit {
        #[arg(long, short = 'm')]
        message: String,
    },
    Commits,
    Remote {
        #[arg(long, short = 's')]
        server: String,
    },
    Download {
        #[arg(long, short = 's')]
        server: String,
        #[arg(long, short = 'p')]
        project: String
    },
    Create {
        #[arg(long, short = 's')]
        server: String,
        #[arg(long, short = 'n')]
        name: String
    },
    Upload {
        #[arg(long, short = 's')]
        server: String,
        #[arg(long, short = 'p')]
        project: String
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cli = Cli::parse();
    let subcommand = cli.command.unwrap_or(Subcommands::Version);

    match subcommand {
        Subcommands::Init => init::run(),
        Subcommands::Commit { message } => commit::run(message),
        Subcommands::Commits => commits::run(),
        Subcommands::Version => println!("Version {VERSION}"),
        Subcommands::Delete => {
            let working_dir =
                std::env::current_dir().expect("Failed to get current working directory!");
            let tit_dir = working_dir.join(kern::TIT_DIR);
            fs::remove_dir_all(tit_dir).expect("Failed to remove tit dir!");
        }
        Subcommands::Remote { server } => remote::add_remote(&server, "test"),
        Subcommands::Download { server, project } => download::download(&server, &project),
        Subcommands::Create { server, name } => upload::create_repo(&server, &name),
        Subcommands::Upload { server, project } => upload::upload_all(&server, &project),
    }
}
