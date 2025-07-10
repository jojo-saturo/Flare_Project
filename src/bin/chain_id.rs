use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;
use url::Url;

#[tokio::main]
async fn main()-> Result<()> {
    let provider = 
        ProviderBuilder:: new().connect_http(Url::parse("https://coston2-api.flare.network/ext/C/rpc")?);
    let chain_id = provider.get_chain_id().await?;
    println!("Chain ID: {chain_id}");
        Ok(())
}