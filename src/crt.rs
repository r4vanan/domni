use reqwest;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
struct Subdomain {
    name_value: String,
}

#[tokio::main]
pub async fn get_main(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let domain = "google.com";

    // Specify the URL of the JSON file to retrieve
    let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);

    // Send an HTTP GET request to the URL and retrieve the response as text
    let response_text = reqwest::get(url).await?.text().await?;

    // Parse the JSON response text into a vector of Subdomain structs
    let subdomains: Vec<Subdomain> = serde_json::from_str(&response_text)?;

    // Print the list of subdomains
    let mut unique_lines: HashSet<String> = HashSet::new();
    for subdomain in subdomains {
        let filter_line = subdomain.name_value.lines().filter(|line| !line.contains("@") && !line.contains("#") && !line.contains("*") && !line.contains("\\"));
        let mut filtered_text: Vec<String> = filter_line.map(|line| line.to_string()).collect();
        filtered_text.sort();
        for line in filtered_text {
            if !unique_lines.contains(&line) {
                unique_lines.insert(line.clone());
                println!("{}", line);
            }
        }
    }
    Ok(())
}

