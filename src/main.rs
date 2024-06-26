mod overlay;
mod p2p;
mod types;

use eyre;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    p2p::run().await
}
