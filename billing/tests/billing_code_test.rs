use billing::{
    application::{ports::BillingRepositoryPort, use_cases::create_billing_code::CreateBillingCodeUseCase},
    infrastructure::in_memory_billing_repository::InMemoryBillingRepository,
};

#[test]
fn test_create_billing_code() {
    let mut billing_repository = InMemoryBillingRepository::new();
    let mut create_billing_code_use_case = CreateBillingCodeUseCase::new(&mut billing_repository);

    let code = "12345".to_string();
    let description = "Test billing code".to_string();
    let price = 100.0;

    create_billing_code_use_case
        .execute(code.clone(), description.clone(), price)
        .unwrap();

    let billing_code = billing_repository.find_by_code(&code).unwrap();

    assert_eq!(billing_code.code, code);
    assert_eq!(billing_code.description, description);
    assert_eq!(billing_code.price, price);
}

#[test]
fn test_create_billing_code_with_invalid_code() {
    let mut billing_repository = InMemoryBillingRepository::new();
    let mut create_billing_code_use_case = CreateBillingCodeUseCase::new(&mut billing_repository);

    let code = "".to_string();
    let description = "Test billing code".to_string();
    let price = 100.0;

    let result = create_billing_code_use_case.execute(code, description, price);

    assert!(result.is_err());
}
