use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opts {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// Include Paths
    #[structopt(short = "I", long)]
    pub includes: Vec<String>,
}

pub fn parse_cmd_line_args() -> Opts {
    Opts::from_args()
}
