use log::{debug, info};
use reqwest::Error;
use serde_json::json;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";

pub struct Request {
    client: reqwest::Client,
}

impl Request {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build client");
        Self { client }
    }
    pub async fn get(&self, url: String) -> Result<serde_json::Value, Error> {
        debug!("Requesting get url: {}", url);
        let response = self.client.get(url).send().await?;
        info!("Status code: {}", response.status());

        let body = response.json::<serde_json::Value>().await?;
        // println!("Response body: {:#?}", body);
        Ok(body)
    }
    pub async fn post(
        &self,
        url: String,
        token: String,
        ref_name: String,
        inputs: String,
    ) -> Result<(), Error> {
        debug!("Requesting post url: {}", url);
        let body = if inputs.is_empty() {
            json!({ "ref": ref_name }).to_string()
        } else {
            json!({ "ref": ref_name, "inputs": inputs }).to_string()
        };
        let response = self
            .client
            .post(url)
            .bearer_auth(token)
            .body(body)
            .send()
            .await?;
        info!("Status code: {}", response.status());
        Ok(())
    }
}

pub async fn put_request(url: String, json_data: String) -> Result<(), Error> {
    //let url = "http://localhost:4000/tasks/7";
    //let json_data = r#"{"title":"Problems during installation","status":"todo","priority":"low","label":"bug"}"#;

    let client = reqwest::Client::new();

    let response = client
        .put(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status code: {}", response.status());

    let response_body = response.text().await?;

    println!("Response body: \n{}", response_body);

    Ok(())
}
