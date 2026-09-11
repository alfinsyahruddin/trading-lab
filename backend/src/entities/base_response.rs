use actix_web::web::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::entities::app_response::AppResponse;

#[derive(Debug, Serialize)]
pub struct BaseResponse<T: Serialize> {
    pub data: Option<T>,
    pub status: u16,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T: Serialize> BaseResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            status: 200,
            message: None,
            timestamp: Utc::now(),
        }
    }
}

impl BaseResponse<()> {
    pub fn message(message: impl Into<String>) -> Self {
        Self {
            data: None,
            status: 200,
            message: Some(message.into()),
            timestamp: Utc::now(),
        }
    }

    pub fn error(status: u16, message: impl Into<String>) -> Self {
        Self {
            data: None,
            status,
            message: Some(message.into()),
            timestamp: Utc::now(),
        }
    }
}

pub trait JsonFromStringTrait {
    fn json(self) -> AppResponse<String>;
    fn json_data(self) -> AppResponse<String>;
}

impl JsonFromStringTrait for String {
    fn json(self) -> AppResponse<String> {
        Ok(Json(BaseResponse {
            data: None,
            status: 200,
            message: Some(self),
            timestamp: Utc::now(),
        }))
    }

    fn json_data(self) -> AppResponse<String> {
        Ok(Json(BaseResponse::success(self)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct DummyItem {
        id: i32,
        name: String,
    }

    #[test]
    fn success_response_structure_and_serialization() {
        let item = DummyItem {
            id: 42,
            name: "Alpha Strategy".into(),
        };
        let res = BaseResponse::success(item);

        assert_eq!(res.status, 200);
        assert!(res.message.is_none());
        assert!(res.data.is_some());

        let json_value: serde_json::Value =
            serde_json::to_value(&res).expect("serializes to json value");
        assert_eq!(json_value["status"], 200);
        assert!(json_value["message"].is_null());
        assert_eq!(json_value["data"]["id"], 42);
        assert_eq!(json_value["data"]["name"], "Alpha Strategy");
        assert!(json_value["timestamp"].is_string());
    }

    #[test]
    fn message_response_structure_and_serialization() {
        let res = BaseResponse::<()>::message("Action completed successfully");

        assert_eq!(res.status, 200);
        assert_eq!(
            res.message.as_deref(),
            Some("Action completed successfully")
        );
        assert!(res.data.is_none());

        let json_value: serde_json::Value =
            serde_json::to_value(&res).expect("serializes to json value");
        assert_eq!(json_value["status"], 200);
        assert_eq!(json_value["message"], "Action completed successfully");
        assert!(json_value["data"].is_null());
        assert!(json_value["timestamp"].is_string());
    }

    #[test]
    fn error_response_structure_and_serialization() {
        let res = BaseResponse::<()>::error(404, "Resource not found");

        assert_eq!(res.status, 404);
        assert_eq!(res.message.as_deref(), Some("Resource not found"));
        assert!(res.data.is_none());

        let json_value: serde_json::Value =
            serde_json::to_value(&res).expect("serializes to json value");
        assert_eq!(json_value["status"], 404);
        assert_eq!(json_value["message"], "Resource not found");
        assert!(json_value["data"].is_null());
        assert!(json_value["timestamp"].is_string());
    }

    #[test]
    fn json_from_string_trait_message() {
        let msg = "Operation succeeded".to_string();
        let app_res = msg.json().expect("ok result");
        let base = app_res.into_inner();

        assert_eq!(base.status, 200);
        assert_eq!(base.message.as_deref(), Some("Operation succeeded"));
        assert!(base.data.is_none());
    }

    #[test]
    fn json_from_string_trait_data() {
        let data = "payload_content".to_string();
        let app_res = data.json_data().expect("ok result");
        let base = app_res.into_inner();

        assert_eq!(base.status, 200);
        assert!(base.message.is_none());
        assert_eq!(base.data.as_deref(), Some("payload_content"));
    }
}
