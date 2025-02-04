use std::error::Error;

mod consensus;
mod staking;
mod governance;
mod api;
mod types;
mod crypto;
mod nervous;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   Ok(())
}
