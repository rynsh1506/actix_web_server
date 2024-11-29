use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryPagination {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub order: Option<String>,
}

impl QueryPagination {
    pub fn paginate(&self) -> (i64, i64, i64, String) {
        let limit = match self.limit {
            Some(-1) => i64::MAX,
            Some(l) => l,
            None => 10,
        };

        let order = match self.order.as_deref() {
            Some(order) if order.eq_ignore_ascii_case("ASC") => "ASC".to_string(),
            Some(order) if order.eq_ignore_ascii_case("DESC") => "DESC".to_string(),
            _ => "DESC".to_string(),
        };

        let page = self.page.unwrap_or(1);
        let offset = if limit == i64::MAX {
            0
        } else {
            (page - 1) * limit
        };

        (limit, offset, page, order)
    }
}
