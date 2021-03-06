use clap::{ArgEnum, Args, Parser, Subcommand};

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
    Create(Create),
}

/// List the files presents in the input archive
#[derive(Args)]
struct List {
    // TODO: Make sure this is a path to a BSA
    /// Path to archive file
    #[clap(parse(from_os_str))]
    archive: std::path::PathBuf,
    /// Include extra information in archive listing
    #[clap(short, long)]
    long: bool,
}

/// Extract a file from the input archive
#[derive(Args)]
struct Extract {
    // TODO: Make sure this is a path to a BSA
    /// Path to archive file
    #[clap(parse(from_os_str))]
    archive: std::path::PathBuf,
    /// Directory to extract to
    #[clap(parse(from_os_str))]
    output_directory: std::path::PathBuf,
    /// File to extract from archive
    file: Option<String>,
}

/// Create an archive with files from folder
#[derive(Args)]
struct Create {
    /// Path to archive file
    #[clap(parse(from_os_str))]
    archive: std::path::PathBuf,
    /// Directory to create archive from
    #[clap(parse(from_os_str))]
    input_directory: std::path::PathBuf,
    /// Archive type
    #[clap(arg_enum, value_name = "TYPE")]
    archive_type: ArchiveType,
    #[clap(short, long)]
    /// Compress archive
    compress: Option<bool>,
    /// Deduplicate files in the archive
    #[clap(short, long)]
    share: Option<bool>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum ArchiveType {
    Tes3,
    Tes4,
    Tes5,
    Fo3,
    Fnv,
    Sse,
    Fo4,
    Fo4dds,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List(result) => {
            println!("Listing {:?}", result.archive);
        }
        Commands::Extract(result) => {
            println!(
                "Extracting {:?} to {:?}",
                result.archive, result.output_directory
            );
        }
        Commands::Create(result) => {
            println!("Creating {:?}", result.archive);
        }
    }
}
