use rawncc::{Callback, CastContext, ComplexContext, FnContext, VarContext};
use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug, Clone)]
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

impl Into<rawncc::Options> for Opts {
    fn into(self) -> rawncc::Options {
        rawncc::Options {
            debug: self.debug,
            verbose: self.verbose,
            input: self.input,
            includes: self.includes,
        }
    }
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let options = Opts::from_args();

    let mut var_handler = {
        let opts = options.clone();
        move |context: VarContext| {
            if opts.debug {
                log::debug!("Found variable: {:?}", context);
            }
            match rawncc::check_ra_nc_var(&context) {
                Ok(()) => (),
                Err(regex) => log::debug!(
                    "Invalid name for variable {:?} (regex = {})",
                    &context,
                    &regex
                ),
            }
        }
    };

    let mut fn_handler = |context: FnContext| log::debug!("Found function: {:?}", &context);

    let mut cast_handler = |context: CastContext| {
        log::error!(
            "C style cast found at {:?}. Remove immediatelly!",
            &context.location
        )
    };

    let mut complex_handler = |context: ComplexContext| {
        log::debug!("Found complext type: {:?}", &context);
    };

    rawncc::parse_file(
        options.into(),
        Callback {
            var: Some(&mut var_handler),
            fun: Some(&mut fn_handler),
            cast: Some(&mut cast_handler),
            complex: Some(&mut complex_handler),
        },
    );
}
