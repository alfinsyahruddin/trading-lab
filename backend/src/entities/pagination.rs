use serde::{Deserialize, Serialize};
use validator::Validate;

pub const DEFAULT_PAGE: u32 = 1;
pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 100;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Validate)]
pub struct PaginationQuery {
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,

    #[validate(range(min = 0))]
    pub offset: Option<u32>,
}

impl PaginationQuery {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(DEFAULT_PAGE)
    }

    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT)
    }

    pub fn offset(&self) -> u32 {
        self.offset.unwrap_or_else(|| {
            let p = self.page();
            if p > 0 {
                (p - 1) * self.limit()
            } else {
                0
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

impl<T> PaginatedData<T> {
    pub fn new(items: Vec<T>, total: u64, page: u32, limit: u32) -> Self {
        let total_pages = if limit == 0 {
            0
        } else {
            ((total as f64) / (limit as f64)).ceil() as u32
        };

        Self {
            items,
            total,
            page,
            limit,
            total_pages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::base_response::BaseResponse;

    #[test]
    fn pagination_query_default_values() {
        let query = PaginationQuery::default();
        assert_eq!(query.page(), DEFAULT_PAGE);
        assert_eq!(query.limit(), DEFAULT_LIMIT);
        assert_eq!(query.offset(), 0);
    }

    #[test]
    fn pagination_query_calculated_offset() {
        let query = PaginationQuery {
            page: Some(3),
            limit: Some(25),
            offset: None,
        };
        assert_eq!(query.page(), 3);
        assert_eq!(query.limit(), 25);
        assert_eq!(query.offset(), 50); // (3 - 1) * 25
    }

    #[test]
    fn pagination_query_explicit_offset_precedence() {
        let query = PaginationQuery {
            page: Some(3),
            limit: Some(25),
            offset: Some(10),
        };
        assert_eq!(query.page(), 3);
        assert_eq!(query.limit(), 25);
        assert_eq!(query.offset(), 10);
    }

    #[test]
    fn pagination_query_limit_capped_at_max() {
        let query = PaginationQuery {
            page: Some(1),
            limit: Some(200),
            offset: None,
        };
        assert_eq!(query.limit(), MAX_LIMIT);
    }

    #[test]
    fn pagination_query_validation_boundaries() {
        // Valid query
        let valid = PaginationQuery {
            page: Some(1),
            limit: Some(100),
            offset: Some(0),
        };
        assert!(valid.validate().is_ok());

        // Invalid page (page < 1)
        let invalid_page = PaginationQuery {
            page: Some(0),
            limit: Some(10),
            offset: None,
        };
        assert!(invalid_page.validate().is_err());

        // Invalid limit (limit < 1)
        let zero_limit = PaginationQuery {
            page: Some(1),
            limit: Some(0),
            offset: None,
        };
        assert!(zero_limit.validate().is_err());

        // Invalid limit (limit > 100)
        let high_limit = PaginationQuery {
            page: Some(1),
            limit: Some(101),
            offset: None,
        };
        assert!(high_limit.validate().is_err());
    }

    #[test]
    fn pagination_query_json_deserialization() {
        let json = r#"{"page": 2, "limit": 15}"#;
        let query: PaginationQuery = serde_json::from_str(json).expect("valid JSON query");
        assert_eq!(query.page(), 2);
        assert_eq!(query.limit(), 15);
        assert_eq!(query.offset(), 15);
    }

    #[test]
    fn paginated_data_total_pages_calculation() {
        // 0 items
        let data_empty: PaginatedData<String> = PaginatedData::new(vec![], 0, 1, 10);
        assert_eq!(data_empty.total_pages, 0);

        // Exact multiple
        let data_exact: PaginatedData<i32> = PaginatedData::new(vec![1, 2], 20, 1, 10);
        assert_eq!(data_exact.total_pages, 2);

        // Not an exact multiple (ceil up)
        let data_ceil: PaginatedData<i32> = PaginatedData::new(vec![1], 25, 3, 10);
        assert_eq!(data_ceil.total_pages, 3);
    }

    #[test]
    fn paginated_data_in_unified_response_envelope() {
        let items = vec!["stock_a".to_string(), "stock_b".to_string()];
        let paginated = PaginatedData::new(items, 50, 1, 2);
        let envelope = BaseResponse::success(paginated);

        let json_str = serde_json::to_string(&envelope).expect("serialize envelope");
        let v: serde_json::Value = serde_json::from_str(&json_str).expect("parse JSON");

        assert_eq!(v["status"], 200);
        assert!(v["message"].is_null());
        assert!(v["timestamp"].is_string());
        assert_eq!(v["data"]["total"], 50);
        assert_eq!(v["data"]["page"], 1);
        assert_eq!(v["data"]["limit"], 2);
        assert_eq!(v["data"]["total_pages"], 25);
        assert_eq!(v["data"]["items"].as_array().unwrap().len(), 2);
    }
}
