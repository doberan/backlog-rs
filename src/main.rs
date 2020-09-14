use dotenv;
use std::collections::HashMap;

pub struct Request {
    key: String,
    domain: String,
    prefix: String
}

impl Request {
    pub fn new(key: String, domain: String, prefix: String) -> Self {
        Self { key, domain, prefix }
    }

    pub fn get_request_url(&mut self) -> String {
        format!("{}{}?apiKey={}", self.domain, self.prefix, self.key)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut request: Request = Request {
        key:    dotenv::var("API_KEY")?,
        domain: dotenv::var("API_URL")?,
        prefix: "/space/activities".to_string()
     };
    let url = request.get_request_url();

    let client = reqwest::Client::new();

    println!("api_key = {}", request.key);
    println!("api_url = {}", request.domain);

    let resp = client.get(url.as_str()).send().await?;
    let body = resp.text().await? ;

    let mut tickets: Vec<HashMap<String, serde_json::Value>> = vec![HashMap::new()];
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let notification_array = json.as_array().unwrap();    

    println!("{:?}", json);

    for ticket in notification_array.iter() {
        let object = ticket.as_object().unwrap();
        let mut map: HashMap<String, serde_json::Value> = HashMap::new();
        for (key, value) in object.iter() {
            map.insert(key.to_owned(), value.to_owned());
        }
        tickets.push(map);
    }
    println!("{:?}", tickets[0]);
    Ok(())
}
