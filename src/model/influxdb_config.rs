use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InfluxdbConfig {
    pub address: String,
    pub organisation: String,
    pub bucket: String,
    pub influxdb_token_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let payload = InfluxdbConfig{
            address: "address".to_string(),
            organisation: "organisation".to_string(),
            bucket: "bucket".to_string(),
            influxdb_token_path: "influxdb_token_path".to_string(),
        };
        assert_eq!(
            r#"{"address":"address","organisation":"organisation","bucket":"bucket","influxdb_token_path":"influxdb_token_path"}"#,
            serde_json::to_string(&payload).expect("Cannot serialize").to_string()
        );
    }

    #[test]
    fn deserialize() {
        let payload = r#"{"address":"address","organisation":"organisation","bucket":"bucket","influxdb_token_path":"influxdb_token_path"}"#;
        let result: InfluxdbConfig = serde_json::from_str(&payload).expect("Cannot serialize");
        assert_eq!(
            InfluxdbConfig{
                address: "address".to_string(),
                organisation: "organisation".to_string(),
                bucket: "bucket".to_string(),
                influxdb_token_path: "influxdb_token_path".to_string(),
            },
            result
        );
    }
}