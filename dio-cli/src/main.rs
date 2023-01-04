//! [`dio-cli`]

pub mod app;
pub mod backend;
pub mod db;
pub mod server;
pub mod ui;
pub mod util;

use argh::FromArgs;

// -------------------------------------------------------------------------------------------------------------------

pub const LEN_PRINCIPLES: usize = 14;
pub const LEN_FACTS: usize = 12;

// -------------------------------------------------------------------------------------------------------------------

/// Demo.
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in `ms` between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,

    /// whether unicode symbols are used to improve the overlook app of the app.
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

// -------------------------------------------------------------------------------------------------------------------

/// .
///
/// # Errors
///
/// This function will return an error if .
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Gloria In Excelsis Deo!");

    let cli: Cli = argh::from_env();
    let tick_rate = std::time::Duration::from_millis(cli.tick_rate);

    backend::run(tick_rate, cli.enhanced_graphics).await?;
    Ok(())
}

// -------------------------------------------------------------------------------------------------------------------
