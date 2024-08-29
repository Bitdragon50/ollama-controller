use qdrant_client::qdrant::Value;

enum Field {
    Space,
    Page,
    Date
}

enum Operator {
    Equal,
    NotEqual,
    Greater,
    Lesser,
    And,
    Or
}

enum ValueType {
    ValueType,
    Function
}

struct Clause {
    field: Field,
    operator: Operator,
    value: Value
}

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use std::env;
use base64::encode;

#[derive(Deserialize, Debug)]
struct SearchResult {
    title: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

#[tokio::main]
async fn search_confluence() -> Result<(), Box<dyn std::error::Error>> {
    // Your Atlassian email and API token
    let email = "your_email@atlassian.net";
    let api_token = "your_api_token";

    // Encode credentials
    let credentials = encode(format!("{}:{}", email, api_token));

    // API endpoint with CQL query
    let url = "https://your_site_name.atlassian.net/wiki/rest/api/search?cql=text~\"project plan\"";

    // Make the request
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Basic {}", credentials))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    // Parse the response
    let search_response: SearchResponse = response.json().await?;
    for result in search_response.results {
        println!("Title: {}", result.title);
        println!("URL: {}", result.url);
    }

    Ok(())
}
