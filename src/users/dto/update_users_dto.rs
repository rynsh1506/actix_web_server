use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(length(min = 3, max = 255))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 8))]
    pub password: Option<String>,
}
