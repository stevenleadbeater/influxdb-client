use std::fmt::{Display, Formatter};
use std::{error, fmt};

pub enum InfluxDbError<T> {
    Failed(T, String),
}

impl<T> Display for InfluxDbError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InfluxDbError::Failed(_, status) => {
                write!(f, "Rest call failed {}", status)
            }
        }
    }
}

impl<T> error::Error for InfluxDbError<T> {}

impl<T> fmt::Debug for InfluxDbError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "InfluxDbErrorError({})", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn map_bad_response_no_body() {
        let result = InfluxDbError::Failed((), "500".to_string());
        assert_eq!("Rest call failed 500", result.to_string());
    }

    #[actix_rt::test]
    async fn map_bad_response_no_body_formatter() {
        let result = InfluxDbError::Failed((), "500".to_string());
        assert_eq!("InfluxDbErrorError(Rest call failed 500)", format!("{:#?}", result));
    }
}
