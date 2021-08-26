use juice_sdk_rs::{client, transports};

#[tokio::main]
async fn main() -> juice_sdk_rs::Result<()> {
    let transport = transports::http::Http::new("http://10.1.1.40:7009")?;
    let client = client::Client::new(transport, true);

    let number = client.block_number(String::from("sys")).await?;
    println!("{}", number);

    Ok(())
}
