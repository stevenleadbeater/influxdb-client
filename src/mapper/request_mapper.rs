use reqwest::{RequestBuilder, Client};
use std::time::Duration;

pub (crate) fn get_request(influxdb_token: String, url: String, body: String) -> RequestBuilder {
    let client = Client::new();
    client.post(&url)
        .header("Authorization", format!("Token {}", influxdb_token))
        .body(body)
        .timeout(Duration::from_secs(5))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_request_correct() {
        let result = get_request(
            "token".to_string(),
            "http://example.com".to_string(),
            "body".to_string(),
        ).build();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!("http://example.com/", result.url().to_string());
        let token = result.headers().get("Authorization");
        assert!(token.is_some());
        let token = token.unwrap();
        assert_eq!("Token token".as_bytes(), token.as_bytes());
        assert!(result.body().is_some());
        let body = result.body().unwrap();
        assert!(body.as_bytes().is_some());
        let body = body.as_bytes().unwrap();
        assert_eq!("body".as_bytes(), body);
    }
}