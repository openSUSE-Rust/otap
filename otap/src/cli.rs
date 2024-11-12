use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Increase verbosity (conflicts with --quiet)
    #[arg(long, short = 'v')]
    pub verbose: bool,

    /// Be quiet, not verbose (conflicts with --verbose)
    #[arg(long, short = 'q')]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Various subcommands about requests
    #[command(subcommand)]
    Request(RequestCommand),
}

#[derive(Subcommand, Debug)]
pub enum RequestCommand {
    /// Show the request itself, and generate a diff for review, if
    /// used with the --diff option. The keyword show can be omitted if the ID is numeric.
    Show {
        /// Generate a diff
        #[arg(long, short)]
        diff: bool,
        /// Print output in list view as list subcommand
        #[arg(long, short)]
        brief: bool,
        /// Request ID
        id: usize,
    },
}
