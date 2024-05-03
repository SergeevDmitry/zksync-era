use clap::Subcommand;

use crate::errors::CLIErrors;

pub(crate) mod batch;
pub(crate) mod l1;
mod utils;

#[derive(Subcommand)]
pub enum StatusCommand {
    Batch(batch::Args),
    L1,
}

impl StatusCommand {
    pub(crate) async fn run(self) -> Result<(), CLIErrors> {
        match self {
            StatusCommand::Batch(args) => Ok(batch::run(args).await?),
            StatusCommand::L1 => l1::run().await,
        }
    }
}
