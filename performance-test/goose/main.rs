use goose::prelude::*;

async fn loadtest_products(user: &mut GooseUser) -> TransactionResult {
    let _goose_metrics = user.get("products").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("products").register_transaction(transaction!(loadtest_products))
        )
        .execute()
        .await?;

    Ok(())
}