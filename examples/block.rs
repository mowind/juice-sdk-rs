use juice_sdk_rs::{client, transports, types, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let transport = transports::http::Http::new("http://10.1.1.40:7009")?;
    let client = client::Client::new(transport, true);

    //let number = client.block_number(String::from("sys")).await?;
    let block = client
        .block_by_number(
            String::from("sys"),
            types::BlockNumber::Number(types::U64::from(300)),
        )
        .await?;
    println!("{}", serde_json::to_string_pretty(&block).unwrap());

    Ok(())
}
