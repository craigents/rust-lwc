use std::collections::HashMap;

use crate::application::ports::BillingRepositoryPort;
use crate::domain::billing_code::BillingCode;

pub struct InMemoryBillingRepository {
    billing_codes: HashMap<String, BillingCode>,
}

impl InMemoryBillingRepository {
    pub fn new() -> Self {
        Self {
            billing_codes: HashMap::new(),
        }
    }
}

impl BillingRepositoryPort for InMemoryBillingRepository {
    fn save(&mut self, billing_code: BillingCode) {
        self.billing_codes
            .insert(billing_code.code.clone(), billing_code);
    }

    fn find_by_code(&self, code: &str) -> Option<&BillingCode> {
        self.billing_codes.get(code)
    }
}
