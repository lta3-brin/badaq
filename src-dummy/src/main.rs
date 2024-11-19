use aerolib::aeronet::net;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    net::try_connect("127.0.0.1:20508").await?;

    Ok(())
}
