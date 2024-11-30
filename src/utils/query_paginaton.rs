use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationError};

fn default_order() -> HashMap<String, String> {
    let mut default = HashMap::new();
    default.insert("id".to_string(), "DESC".to_string());
    default
}

fn validate_order(order: &HashMap<String, String>) -> Result<(), ValidationError> {
    for (key, value) in order {
        if !value.eq_ignore_ascii_case("ASC") && !value.eq_ignore_ascii_case("DESC") {
            let mut error = ValidationError::new("invalid_order");
            error.message = Some(
                format!(
                    "Invalid order value for key '{}'. Must be 'ASC' or 'DESC'.",
                    key
                )
                .into(),
            );
            return Err(error);
        }
    }
    Ok(())
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct QueryPagination {
    pub limit: Option<i64>,
    pub page: Option<i64>,

    #[validate(custom(function = "validate_order"))]
    #[serde(default = "default_order")]
    pub order: HashMap<String, String>,
}

impl QueryPagination {
    pub fn paginate(&self) -> (i64, i64, i64, HashMap<String, String>) {
        let mut limit = self.limit.unwrap_or(10);
        let mut page = self.page.unwrap_or(1);

        if limit == -1 {
            page = 1;
            limit = i64::MAX;
        }

        let offset = if limit == i64::MAX {
            0
        } else {
            (page - 1) * limit
        };

        let order = self
            .order
            .iter()
            .map(|(key, value)| (key.clone(), value.to_uppercase()))
            .collect();

        (limit, offset, page, order)
    }
}
