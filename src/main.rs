use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List(List),
    Extract(Extract),
}

/// List the files presents in the input archive
#[derive(Args)]
struct List {
    // TODO: Make sure this is a path to a BSA
    #[clap(parse(from_os_str), value_name = "ARCHIVE FILE")]
    path: std::path::PathBuf,
}

/// Extract a file from the input archive
#[derive(Args)]
struct Extract {
    // TODO: Make sure this is a path to a BSA
    #[clap(parse(from_os_str), value_name = "ARCHIVE FILE")]
    path: std::path::PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List(path) => {
            println!("Listing {:?}", path.path);
        }
        Commands::Extract(path) => {
            println!("Extracting {:?}", path.path);
        }
    }
}
