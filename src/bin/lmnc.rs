//! The short name, so `uvx lmnc` works. On npm the wrapper carries the
//! scope, so the same run reads `npx @cjohnhanson/lmnc`.
//!
//! maturin ties an installed command name to the Cargo bin name, and
//! refuses a `[project.scripts]` entry beside a binary. A wheel
//! published as `lmnc` therefore needs a `lmnc` command, or
//! `uvx lmnc` errors and tells the reader to type
//! `uvx --from lmnc almanac` forever.
//!
//! This execs `almanac` beside it rather than carrying a second copy.
//! Both names then work from one install, which is what the naming
//! scheme asks for.
use std::os::unix::process::CommandExt;

fn main() -> std::process::ExitCode {
    let Ok(me) = std::env::current_exe() else {
        eprintln!("lmnc: cannot resolve its own path");
        return std::process::ExitCode::FAILURE;
    };
    let real = me.with_file_name("almanac");
    let err = std::process::Command::new(&real)
        .args(std::env::args_os().skip(1))
        .exec();
    eprintln!("lmnc: cannot run {}: {err}", real.display());
    std::process::ExitCode::FAILURE
}
