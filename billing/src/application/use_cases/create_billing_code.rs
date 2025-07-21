use crate::{
    application::ports::BillingRepositoryPort,
    domain::billing_code::BillingCode,
};

pub struct CreateBillingCodeUseCase<'a> {
    billing_repository: &'a mut dyn BillingRepositoryPort,
}

impl<'a> CreateBillingCodeUseCase<'a> {
    pub fn new(billing_repository: &'a mut dyn BillingRepositoryPort) -> Self {
        Self {
            billing_repository,
        }
    }

    pub fn execute(&mut self, code: String, description: String, price: f64) -> Result<(), validator::ValidationErrors> {
        let billing_code = BillingCode::new(code, description, price)?;
        self.billing_repository.save(billing_code);
        Ok(())
    }
}
