use std::process::exit;

use clap::{Parser, Subcommand};
use exitcode::EXIT_UNKNOWN_RESOURCE;

mod command;
mod exitcode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    Init {
        #[arg(short, long, short = 'n', help = "Name of the repository")]
        name: Option<String>,
        #[arg(short, long, short = 's', help = "Remote server to connect to")]
        server: Option<String>,
        #[arg(short, long, short = 'b', help = "Name of first branch")]
        branch: Option<String>,
    },
    Uninit,
    Sync,
    Add {
        #[arg(index = 1, name = "resource", help = "Type of resource to add")]
        resource: String,
        #[arg(index = 2, name = "id", help = "Type of resource to add")]
        id: String,
    },
    Create {
        #[arg(index = 1, name = "resource", help = "Type of resource to add")]
        resource: String,
        #[arg(index = 2, name = "id", help = "Type of resource to add")]
        id: String,
    },
    List {
        #[arg(index = 1, name = "resource", help = "Type of resource to add")]
        resource: String,
    },
    Switch {
        #[arg(index = 1, name = "resource", help = "Type of resource to switch")]
        resource: String,
        #[arg(index = 2, name = "id", help = "Type of resource to add")]
        id: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let subcommand = cli.command;

    let exit_code = match subcommand {
        Subcommands::Init {
            name,
            server,
            branch,
        } => command::init(name, server, branch),
        Subcommands::Uninit => command::uninit(),
        Subcommands::Sync => command::sync(),
        Subcommands::Create { resource, id } => match resource.as_str() {
            "branch" => command::create_branch(&id),
            "change" => command::commit(id),
            _ => EXIT_UNKNOWN_RESOURCE,
        },
        Subcommands::Add { resource, id } => match resource.as_str() {
            "server" => command::add_server(&id),
            _ => EXIT_UNKNOWN_RESOURCE,
        },
        Subcommands::List { resource } => match resource.as_str() {
            "commits" => command::list_commits(),
            "servers" => command::list_servers(),
            "branches" => command::list_branches(),
            "changes" => command::list_changes(),
            _ => EXIT_UNKNOWN_RESOURCE,
        },
        Subcommands::Switch { resource, id } => match resource.as_str() {
            "branch" => command::set_branch(&id),
            "server" => command::set_server(&id),
            _ => EXIT_UNKNOWN_RESOURCE,
        },
    };

    exit(exit_code);
}
