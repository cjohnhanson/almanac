use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    sigpipe::reset();

    let args = almanac::cli::Args::parse();

    match almanac::cli::run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("almanac: {e}");
            ExitCode::from(1)
        }
    }
}
