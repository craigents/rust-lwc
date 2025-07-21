use crate::domain::billing_code::BillingCode;

pub trait BillingRepositoryPort {
    fn save(&mut self, billing_code: BillingCode);
    fn find_by_code(&self, code: &str) -> Option<&BillingCode>;
}
