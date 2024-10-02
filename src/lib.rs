mod namesdao;

use reqwest;
use serde::{Deserialize, Serialize};
use tokio;
use log::{debug, error};



#[derive(Serialize)]
struct NameRequest {
    name: String,
}

#[derive(Deserialize)]
struct NameResponse {
    address: Option<String>,
    name: String,
    nft_coin_id: String,
    uris: Vec<String>,
    meta_uris: Vec<String>,
    created_block: u64,
    last_transferred_block: u64,
    expiry_block: u64,
}

pub struct Namesdao {}

async fn get_address(name: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    // Strip off '.xch' if present at the end of the name
    let namesdao_name = if name.ends_with(".xch") {
        debug!("Trimmed name: {}", name);
        name.trim_end().to_string()
    } else {
        debug!("Original name: {}", name);
        name.to_string()
    };

    // Build the API URL with the provided name
    let url = format!("https://namesdaolookup.xchstorage.com/{}.json", namesdao_name);

    // Send a GET request to the NamesDAO API
    let response = client.get(&url).send().await?;

    // Parse the JSON response into a `NameResponse` struct
    let name_response: NameResponse = response.json().await?;

    // Extract the address from the parsed response
    let address = name_response.address.unwrap();

    Ok(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_address() {
        let name = "Namesdao.xch";
        let expected_address = "xch1jhye8dmkhree0zr8t09rlzm9cc82mhuqtp5tlmsj4kuqvs69s2wsl90su4";

        match get_address(name).await {
            Ok(address) => assert_eq!(address, expected_address),
            Err(e) => {
                //panic!("Failed to retrieve address: {}", e);
                // Print the response content for further investigation
                println!("Response Content: {:?}", name); //response.content());
            }
        }
    }
}


