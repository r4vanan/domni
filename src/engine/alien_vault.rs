use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    hostname: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    passive_dns: Vec<Record>,
}

#[tokio::main]
pub async fn alien(domain: &str) -> Result<(), Error> {
    let url = format!("https://otx.alienvault.com/api/v1/indicators/domain/{}/passive_dns", domain);
    let response = reqwest::get(url).await?.json::<Response>().await?;

    let mut hostnames: Vec<String> = response.passive_dns.iter().map(|record| record.hostname.clone()).collect();

    let search_string = "google".to_lowercase();
    hostnames.retain(|hostname| hostname.to_lowercase().contains(&search_string));

    hostnames.sort();
    hostnames.dedup();

    for hostname in hostnames {
        println!("{}", hostname);
    }

    Ok(())
}

