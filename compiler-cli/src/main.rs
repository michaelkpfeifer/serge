pub use serge_core::error::Result;

use clap::{
    builder::{styling, Styles},
    Parser,
};

#[derive(Parser)]
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
    Build {},
}

fn main() {
    let result = match Command::parse() {
        Command::Build {} => command_build(),
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

fn command_build() -> Result<()> {
    Ok(())
}
