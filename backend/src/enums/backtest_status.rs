use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Type)]
#[sqlx(type_name = "backtest_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BacktestStatus {
    Pending,
    Processing,
    Done,
    Failed,
}

impl BacktestStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "PENDING",
            Self::Processing => "PROCESSING",
            Self::Done => "DONE",
            Self::Failed => "FAILED",
        }
    }
}

impl fmt::Display for BacktestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::BacktestStatus;

    #[test]
    fn deserialize_should_accept_uppercase_status() {
        let status: BacktestStatus =
            serde_json::from_str("\"PROCESSING\"").expect("valid status JSON");
        assert_eq!(status, BacktestStatus::Processing);
        assert_eq!(status.as_str(), "PROCESSING");
        assert_eq!(status.to_string(), "PROCESSING");
    }
}
