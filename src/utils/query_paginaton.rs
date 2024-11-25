use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryPagination {
    pub limit: Option<String>, // limit as Option<String> to support "ALL" or numeric strings
    pub page: Option<i64>,     // Page number
    pub order: Option<String>, // Order (e.g., "ASC" or "DESC")
}

impl QueryPagination {
    pub fn paginate(&self) -> (i64, i64, i64, String, String) {
        let (limit, limit_str) = match self.limit.as_deref() {
            Some("-1") | Some("ALL") => (i64::MAX, "ALL".to_string()),
            Some(l) => {
                let parsed_limit = l.parse().unwrap_or(10);
                (parsed_limit, l.to_string())
            }
            None => (10, "10".to_string()),
        };

        let page = self.page.unwrap_or(1);
        let order = self.order.clone().unwrap_or("DESC".to_string());
        let offset = if limit == i64::MAX {
            0
        } else {
            (page - 1) * limit
        };

        (limit, offset, page, order, limit_str)
    }
}
