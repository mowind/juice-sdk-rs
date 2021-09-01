use std::str::FromStr;

use juice_sdk_rs::{client, transports, types::H256, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let transport = transports::http::Http::new("http://10.1.1.40:7009")?;
    let client = client::Client::new(transport, true);

    let receipt = client
        .transaction_receipt(
            String::from("sys"),
            H256::from_str("0x0166fcae120e3d7e34324dcbb93335677dcf39076c6de9c03ead1d7f72ddde31")
                .unwrap(),
        )
        .await?;
    println!("{}", serde_json::to_string_pretty(&receipt).unwrap());

    Ok(())
}
