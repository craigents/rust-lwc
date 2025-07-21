use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Validate)]
pub struct Patient {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub dob: String,
    #[validate(length(min = 1))]
    pub address: String,
}
