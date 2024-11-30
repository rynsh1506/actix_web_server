use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn default_limit() -> i64 {
    10
}

fn default_page() -> i64 {
    1
}

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
    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default = "default_page")]
    pub page: i64,

    #[validate(custom(function = "validate_order"))]
    #[serde(default = "default_order")]
    pub order: HashMap<String, String>,
}

impl QueryPagination {
    pub fn paginate(&self) -> (i64, i64, i64, HashMap<String, String>) {
        let limit = if self.limit == -1 {
            i64::MAX
        } else {
            self.limit
        };

        let offset = if limit == i64::MAX {
            0
        } else {
            (self.page - 1) * limit
        };

        let order = self
            .order
            .iter()
            .map(|(key, value)| (key.clone(), value.to_uppercase()))
            .collect();

        (limit, offset, self.page, order)
    }
}
