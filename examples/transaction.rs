use juice_sdk_rs::{client, transports, types, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let transport = transports::http::Http::new("http://10.1.1.40:7009")?;
    let client = client::Client::new(transport, true);

    let number = client.block_number(String::from("test")).await?;
    let block = client
        .block_by_number(String::from("test"), types::BlockNumber::Number(number))
        .await?;
    let block = block.unwrap();

    if block.transactions.len() > 0 {
        let transaction = client
            .transaction_by_hash(String::from("test"), block.transactions[0].hash)
            .await?;
        println!("{:?}", transaction);
    }

    Ok(())
}
