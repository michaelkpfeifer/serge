mod build;

pub use serge_core::error::Result;

use serge_core::build::Target;

use clap::{
    builder::{styling, Styles},
    Parser,
};

#[derive(Parser, Debug)]
#[command(
    version,
    next_display_order = None,
    help_template = "\
{before-help}{name} {version}

{usage-heading} {usage}

{all-args}{after-help}",
    styles = Styles::styled()
        .header(styling::AnsiColor::Blue.on_default())
        .usage(styling::AnsiColor::Blue.on_default())
        .literal(styling::AnsiColor::Green.on_default())
)]

enum Command {
    /// Build the project
    Build {
        /// Emit compile time warnings as errors
        #[arg(long)]
        warnings_as_errors: bool,

        #[arg(short, long, ignore_case = true)]
        target: Option<Target>,
    },
}

fn main() {
    let result = match Command::parse() {
        Command::Build {
            target,
            warnings_as_errors,
        } => command_build(target, warnings_as_errors),
    };

    match result {
        Ok(_) => {
            println!("Looks like things went well.");
        }
        Err(_) => {
            println!("Looks like something went wrong.");
        }
    }
}

fn command_build(target: Option<Target>, warnings_as_errors: bool) -> Result<()> {
    let _ = build::main();
    Ok(())
}
