use crate::model::influxdb_config::InfluxdbConfig;

pub (crate) fn to_influxdb_write_url(influxdb_config: &InfluxdbConfig) -> String {
    format!(
        "{}/api/v2/write?org={}&bucket={}&precision=s",
        influxdb_config.address.to_owned(),
        influxdb_config.organisation.to_owned(),
        influxdb_config.bucket.to_owned()
    )
}

pub (crate) fn to_influxdb_read_url(influxdb_config: &InfluxdbConfig) -> String {
    format!(
        "{}/api/v2/query?org={}",
        influxdb_config.address.to_owned(),
        influxdb_config.organisation.to_owned()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_influxdb_write_url_correct() {
        let result = to_influxdb_write_url(&InfluxdbConfig {
            address: "address".to_string(),
            organisation: "organisation".to_string(),
            bucket: "bucket".to_string(),
            influxdb_token_path: "influxdb_token_path".to_string(),
        });
        assert_eq!("address/api/v2/write?org=organisation&bucket=bucket&precision=s", result);
    }

    #[test]
    fn to_influxdb_read_url_correct() {
        let result = to_influxdb_read_url(&InfluxdbConfig {
            address: "address".to_string(),
            organisation: "organisation".to_string(),
            bucket: "bucket".to_string(),
            influxdb_token_path: "influxdb_token_path".to_string(),
        });
        assert_eq!("address/api/v2/query?org=organisation", result);
    }
}