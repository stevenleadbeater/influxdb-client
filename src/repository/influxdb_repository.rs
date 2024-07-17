use log::debug;
use crate::model::influxdb_config::InfluxdbConfig;
use reqwest::Error;
use crate::mapper::response_mapper::map_response;
use crate::error::influxdb_error::InfluxDbError;
use crate::mapper::url_mapper::{to_influxdb_read_url, to_influxdb_write_url};
use crate::mapper::request_mapper::get_request;

pub async fn write_to_influxdb(
    influxdb_token: String,
    influxdb_config: &InfluxdbConfig,
    body: String
) -> Result<String, InfluxDbError<Option<Error>>> {
    let url = to_influxdb_write_url(influxdb_config);
    debug!("Using token {:#?}", influxdb_token);
    debug!("Using body {:#?}", body);
    let result = get_request(influxdb_token, url, body)
        .send()
        .await;
    debug!("Result {:#?}", result);
    map_response(result).await
}

pub async fn read_from_influxdb(
    influxdb_token: String,
    influxdb_config: &InfluxdbConfig,
    body: String
) -> Result<String, InfluxDbError<Option<Error>>> {
    debug!("Body: {}", body);
    let url = to_influxdb_read_url(influxdb_config);
    let result = get_request(influxdb_token, url, body)
        .header("Content-Type", "application/vnd.flux")
        .send()
        .await;
    map_response(result).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::http_server::setup_test_harness;

    #[actix_rt::test]
    async fn read_from_influxdb_error_no_body() {
        let harness = setup_test_harness();
        let url = harness.url("fails");
        let result = read_from_influxdb(
            "token".to_string(),
            &InfluxdbConfig {
                address: url,
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            "body".to_string()
        ).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed 500 Internal Server Error", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn read_from_influxdb_error_with_body() {
        let harness = setup_test_harness();
        let url = harness.url("fails-body-response");
        let result = read_from_influxdb(
            "token".to_string(),
            &InfluxdbConfig {
                address: url,
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            "body".to_string()
        ).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed Some terrible error", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn read_from_influxdb_failed_request() {
        let harness = setup_test_harness();
        let url = harness.url("some-bad-url");
        let result = read_from_influxdb(
            "token".to_string(),
            &InfluxdbConfig {
                address: url,
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            "body".to_string()
        ).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed 404 Not Found", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn read_from_influxdb_success() {
        let harness = setup_test_harness();
        let url = harness.url("success");
        let result = read_from_influxdb(
            "token".to_string(),
            &InfluxdbConfig {
                address: url,
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            "body".to_string()
        ).await;
        assert!(result.is_ok());
        assert_eq!("test", result.unwrap().to_string());
    }

    #[actix_rt::test]
    async fn write_to_influxdb_error_no_body() {
        let harness = setup_test_harness();
        let url = harness.url("fails");
        let result = write_to_influxdb(
            "token".to_string(),
            &InfluxdbConfig {
                address: url,
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            "body".to_string()
        ).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed 500 Internal Server Error", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn write_to_influxdb_error_with_body() {
        let harness = setup_test_harness();
        let url = harness.url("fails-body-response");
        let result = write_to_influxdb(
            "token".to_string(),
            &InfluxdbConfig {
                address: url,
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            "body".to_string()
        ).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed Some terrible error", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn write_to_influxdb_failed_request() {
        let harness = setup_test_harness();
        let url = harness.url("some-bad-url");
        let result = write_to_influxdb(
            "token".to_string(),
            &InfluxdbConfig {
                address: url,
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            "body".to_string()
        ).await;
        assert!(result.is_err());
        assert_eq!("Rest call failed 404 Not Found", result.unwrap_err().to_string());
    }

    #[actix_rt::test]
    async fn write_to_influxdb_success() {
        let harness = setup_test_harness();
        let url = harness.url("success");
        let result = write_to_influxdb(
            "token".to_string(),
            &InfluxdbConfig {
                address: url,
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            "body".to_string()
        ).await;
        assert!(result.is_ok());
        assert_eq!("test", result.unwrap().to_string());
    }
}