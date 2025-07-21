use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct BillingCode {
    #[validate(length(min = 1, max = 10))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub description: String,
    pub price: f64,
}

impl BillingCode {
    pub fn new(code: String, description: String, price: f64) -> Result<Self, validator::ValidationErrors> {
        let billing_code = Self {
            code,
            description,
            price,
        };
        billing_code.validate()?;
        Ok(billing_code)
    }
}
