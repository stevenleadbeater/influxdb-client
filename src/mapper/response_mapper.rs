use reqwest::{Response, Error};
use crate::error::influxdb_error::InfluxDbError;
use log::{debug, error};

pub (crate) async fn map_response(result: Result<Response, Error>) -> Result<String, InfluxDbError<Option<Error>>> {
    return match result {
        Err(error) => {
            error!("Error: {:#?}", error);
            Err(InfluxDbError::Failed(Some(error), "request failed".to_string()))
        }
        Ok(result) => {
            let status = result.status().clone();
            let body = result
                .text()
                .await;
            debug!("Result: {:#?}", body);
            if !status.is_success() {
                return Err(map_bad_response(body, status.to_string()));
            }
            if body.is_err() {
                return Err(InfluxDbError::Failed(body.err(), "".to_string()))
            }
            Ok(body.unwrap())
        }
    };
}

fn map_bad_response(body: reqwest::Result<String>, status: String) -> InfluxDbError<Option<Error>> {
    InfluxDbError::Failed(
        None,
        if !body.is_ok() {
            status
        } else {
            let body = body.unwrap();
            if body.len() > 0 {
                body
            } else {
                status
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::http_server::setup_test_harness;
    use reqwest::Client;

    #[actix_rt::test]
    async fn map_bad_response_no_body() {
        let harness = setup_test_harness();
        let url = harness.url("");
        let client = Client::new();
        let result = client.post(url)
            .send()
            .await;
        let result = map_bad_response(result.unwrap().text().await, "500".to_string());
        assert_eq!("Rest call failed 500", result.to_string());
    }

    #[actix_rt::test]
    async fn map_bad_response_no_body_formatter() {
        let harness = setup_test_harness();
        let url = harness.url("");
        let client = Client::new();
        let result = client.post(url)
            .send()
            .await;
        let result = map_bad_response(result.unwrap().text().await, "500".to_string());
        assert_eq!("InfluxDbErrorError(Rest call failed 500)", format!("{:#?}", result));
    }

    #[actix_rt::test]
    async fn map_bad_response_with_body() {
        let harness = setup_test_harness();
        let url = harness.url("body-response");
        let client = Client::new();
        let result = client.post(url)
            .send()
            .await;
        let result = map_bad_response(result.unwrap().text().await, "500".to_string());
        assert_eq!("Rest call failed Some terrible error", result.to_string());
    }

    #[actix_rt::test]
    async fn map_bad_response_with_body_formatter() {
        let harness = setup_test_harness();
        let url = harness.url("body-response");
        let client = Client::new();
        let result = client.post(url)
            .send()
            .await;
        let result = map_bad_response(result.unwrap().text().await, "500".to_string());
        assert_eq!("InfluxDbErrorError(Rest call failed Some terrible error)", format!("{:#?}", result));
    }

    #[actix_rt::test]
    async fn map_response_error_no_body() {
        let harness = setup_test_harness();
        let url = harness.url("");
        let client = Client::new();
        let result = client.post(url)
            .send()
            .await;
        let result = map_response(result).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed 500 Internal Server Error", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn map_response_error_with_body() {
        let harness = setup_test_harness();
        let url = harness.url("body-response");
        let client = Client::new();
        let result = client.post(url)
            .send()
            .await;
        let result = map_response(result).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed Some terrible error", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn map_response_failed_request() {
        let harness = setup_test_harness();
        let url = harness.url("some-bad-url");
        let client = Client::new();
        let result = client.post(url)
            .send()
            .await;
        let result = map_response(result).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed 404 Not Found", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn map_response_success() {
        let harness = setup_test_harness();
        let url = harness.url("success");
        let client = Client::new();
        let result = client.post(url)
            .send()
            .await;
        let result = map_response(result).await;
        assert!(result.is_ok());
        assert_eq!("test", result.unwrap().to_string());
    }
}