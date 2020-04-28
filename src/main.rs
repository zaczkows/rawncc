use rawncc::{VarContext, VarContextType};
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

    let var_handler = |context: VarContext| {
        log::debug!("Found variable: {:?}", context);
        let mut regex_str = String::from("^");
        let static_const = context.is_static && context.is_const;
        if static_const {
            regex_str += "([A-Z]+_)+[A-Z]+";
        } else {
            if context.is_member {
                regex_str += "m_";
            }
            if context.var_type == VarContextType::Ptr {
                regex_str += "p";
            }
            if context.var_type == VarContextType::Ref {
                regex_str += "r";
            }

            if context.is_member {
                regex_str += "([A-Z][a-z0-9]+)+";
            }
            else{
                regex_str += "[a-z0-9]+([A-Z][a-z0-9]+)*";
            }
        }
        regex_str += "$";

        let r = regex::Regex::new(regex_str.as_str()).unwrap();
        if !r.is_match(context.name.as_str()) {
            log::debug!("Invalid naming (re={})", &regex_str);
        }
    };

    rawncc::parse_file(options.into(), var_handler);
}
