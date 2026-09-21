use clap::{Parser, Subcommand};

mod ops;

pub type Result<T> = std::result::Result<T, open_library_client::OpenLibraryError>;

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct UtilCli {
    #[command(subcommand)]
    pub op: UtilOperation,
}

#[derive(Subcommand, Clone, Debug)]
pub enum UtilOperation {
    /// Run a bulk test
    Bulk(ops::bulk::BulkArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = UtilCli::parse();
    println!("{cli:#?}");
    match cli.op.clone() {
        UtilOperation::Bulk(bulk_args) => bulk_args.run().await?,
    }
    Ok(())
}
