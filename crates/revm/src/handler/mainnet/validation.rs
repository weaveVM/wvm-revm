use crate::primitives::Address;
use crate::{
    primitives::{db::Database, EVMError, Env, InvalidTransaction, Spec},
    Context,
};
use revm_interpreter::gas;
use wvm::BUNDLER_ADDRESSES;

/// Validate environment for the mainnet.
pub fn validate_env<SPEC: Spec, DB: Database>(env: &Env) -> Result<(), EVMError<DB::Error>> {
    // Important: validate block before tx.
    env.validate_block_env::<SPEC>()?;
    env.validate_tx::<SPEC>()?;
    Ok(())
}

/// Validates transaction against the state.
pub fn validate_tx_against_state<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
) -> Result<(), EVMError<DB::Error>> {
    // load acc
    let tx_caller = context.evm.env.tx.caller;
    let caller_account = context
        .evm
        .inner
        .journaled_state
        .load_code(tx_caller, &mut context.evm.inner.db)?;

    context
        .evm
        .inner
        .env
        .validate_tx_against_state::<SPEC>(caller_account.data)
        .map_err(EVMError::Transaction)?;

    Ok(())
}

/// Validate initial transaction gas.
pub fn validate_initial_tx_gas<SPEC: Spec, DB: Database>(
    env: &Env,
) -> Result<u64, EVMError<DB::Error>> {
    let input = &env.tx.data;
    let is_create = env.tx.transact_to.is_create();
    let access_list = &env.tx.access_list;
    let authorization_list_num = env
        .tx
        .authorization_list
        .as_ref()
        .map(|l| l.len() as u64)
        .unwrap_or_default();

    let initial_gas_spend = gas::validate_initial_tx_gas(
        SPEC::SPEC_ID,
        input,
        is_create,
        access_list,
        authorization_list_num,
        env.tx.transact_to.to(),
    );

    // Additional check to see if limit is big enough to cover initial gas.
    if initial_gas_spend > env.tx.gas_limit {
        return Err(InvalidTransaction::CallGasCostMoreThanGasLimit.into());
    }
    Ok(initial_gas_spend)
}

#[cfg(test)]
mod test {
    use crate::primitives::address;
    use revm_interpreter::gas::is_bundler_addr;
    use wvm::BUNDLER_ADDRESSES;

    #[test]
    pub fn test_verify_bundlr_target() {
        let babe1 = is_bundler_addr(&address!("babe1d25501157043c7b4ea7CBC877B9B4D8A057"));
        let babe2 = is_bundler_addr(&address!("babe2dCAf248F2F1214dF2a471D77bC849a2Ce84"));
        let not_babe = is_bundler_addr(&address!("A6dC883ea2A6acb576A933B4d38D13d6069d9fBE"));

        assert!(babe1);
        assert!(babe2);
        assert!(!not_babe);
    }
}
