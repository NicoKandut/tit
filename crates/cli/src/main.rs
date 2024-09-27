use clap::{Parser, Subcommand};

mod command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Subcommands>,
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
    Version,
    Uninit,
    Commits,
    Download,
    Upload,
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
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cli = Cli::parse();
    let subcommand = cli.command.unwrap_or(Subcommands::Version);

    match subcommand {
        Subcommands::Version => println!("Version {VERSION}"),
        Subcommands::Init { name, server, branch} => command::init(name, server, branch),
        Subcommands::Uninit => command::uninit(),
        Subcommands::Commits => command::commits(),
        Subcommands::Upload => command::upload_all(),
        Subcommands::Download => command::download(),
        Subcommands::Create { resource, id } => match resource.as_str() {
            "branch" => command::create_branch(&id),
            "change" => command::commit(id),
            _ => println!("Unknown resource type: {}", resource),
            
        },
        Subcommands::Add { resource, id } => match resource.as_str() {
            "server" => command::add_server(&id),
            _ => println!("Unknown resource type: {}", resource),
            
        }
    }
}
