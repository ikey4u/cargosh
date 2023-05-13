use std::{path::{PathBuf, Path}, fs::File, io::Read};

use clap::Parser;
use indoc::indoc;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

/// cargosh - cargo ecosystem based cross platform shell script interpreter
#[derive(Debug, Parser)]
#[clap(version)]
struct Cli {
    #[arg(short = 'c')]
    command_string: Option<String>,
    /// The cargosh script to run
    script_files: Vec<PathBuf>,
}

fn run_cargosh_script<S: AsRef<str>>(script: S) -> Result<()> {
    Ok(())
}

fn app() -> Result<()> {
    let cli = Cli::parse();
    if let Some(command_string) = cli.command_string.as_deref() {
        run_cargosh_script(command_string)?;
    }
    for script_file in cli.script_files {
        let script = if script_file.exists() {
            let mut f = File::open(&script_file)?;
            let mut script = String::new();
            f.read_to_string(&mut script)?;
            script
        } else {
            continue;
        };
        run_cargosh_script(script)?;
    }
    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();
    if let Err(e) = app() {
        log::error!("{e:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let script = indoc! {r#"
        "#};
        assert!(run_cargosh_script(script).is_ok());
    }
}
