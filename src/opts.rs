use std::path::PathBuf;

pub struct Options {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    pub debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    pub verbose: u8,

    /// Input file
    pub input: PathBuf,

    /// Include Paths
    pub includes: Vec<String>,
}
