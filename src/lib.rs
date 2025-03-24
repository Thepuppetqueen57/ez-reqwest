use reqwest::Client;
use std::collections::HashMap;

pub async fn get_request(url: String, params: Option<HashMap<String, String>>) -> String {
    let client = Client::builder()
        .build()
        .expect("Failed to build client");

    let mut request = client.get(url);

    if let Some(p) = params {
        request = request.query(&p);
    }

    let res = request
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

pub async fn post_request(url: String, params: Option<HashMap<String, String>>) -> String {
    let client = Client::builder()
        .build()
        .expect("Failed to build client");

    let mut request = client.post(url);

    if let Some(p) = params {
        request = request.form(&p);
    }

    let res = request
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_request_test() {
        let result = get_request(
            "https://puppet57.xyz/random-php/ez-request-test.php".to_string(),
            None
        ).await;
        assert_eq!(result, "working".to_string());
    }

    #[tokio::test]
    async fn post_request_test() {
        let result = get_request(
            "https://puppet57.xyz/random-php/ez-request-test.php".to_string(),
            None
        ).await;
        assert_eq!(result, "working".to_string());
    }
}