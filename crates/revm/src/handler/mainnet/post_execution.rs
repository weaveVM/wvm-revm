use crate::primitives::{address, alloy_primitives};
use crate::{
    interpreter::{Gas, SuccessOrHalt},
    primitives::{
        db::Database, EVMError, ExecutionResult, ResultAndState, Spec, SpecId, SpecId::LONDON, U256,
    },
    Context, FrameResult,
};

const WVM_TREASURY_ADDRESS: alloy_primitives::Address =
    address!("a2A0D977847805fE224B789D8C4d3D711ab251e7");

/// Mainnet end handle does not change the output.
#[inline]
pub fn end<EXT, DB: Database>(
    _context: &mut Context<EXT, DB>,
    evm_output: Result<ResultAndState, EVMError<DB::Error>>,
) -> Result<ResultAndState, EVMError<DB::Error>> {
    evm_output
}

/// Clear handle clears error and journal state.
#[inline]
pub fn clear<EXT, DB: Database>(context: &mut Context<EXT, DB>) {
    // clear error and journaled state.
    let _ = context.evm.take_error();
    context.evm.inner.journaled_state.clear();
}

/// Reward beneficiary with gas fee.
#[inline]
pub fn reward_beneficiary<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    gas: &Gas,
) -> Result<(), EVMError<DB::Error>> {
    let beneficiary = context.evm.env.block.coinbase;
    let effective_gas_price = context.evm.env.effective_gas_price();

    // transfer fee to coinbase/beneficiary.
    // EIP-1559 discard basefee for coinbase transfer. Basefee amount of gas is discarded.
    let coinbase_gas_price = if SPEC::enabled(LONDON) {
        effective_gas_price.saturating_sub(context.evm.env.block.basefee)
    } else {
        effective_gas_price
    };

    // WVM: if EIP-1559 enabled we send base fee back to treasury
    // instead of burning it
    if SPEC::enabled(LONDON) {
        wvm_add_base_fee_to_treasury(context)?;
    }

    let coinbase_account = context
        .evm
        .inner
        .journaled_state
        .load_account(beneficiary, &mut context.evm.inner.db)?;

    coinbase_account.data.mark_touch();
    coinbase_account.data.info.balance = coinbase_account
        .data
        .info
        .balance
        .saturating_add(coinbase_gas_price * U256::from(gas.spent() - gas.refunded() as u64));

    Ok(())
}

/// WVM: send base fee back to treasury
#[inline]
fn wvm_add_base_fee_to_treasury<EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
) -> Result<(), EVMError<DB::Error>> {
    let base_fee = context.evm.env.block.basefee.clone();
    let treasury_account = context
        .evm
        .inner
        .journaled_state
        .load_account(WVM_TREASURY_ADDRESS, &mut context.evm.inner.db)?;

    treasury_account.data.mark_touch();
    treasury_account.data.info.balance =
        treasury_account.data.info.balance.saturating_add(base_fee);

    Ok(())
}

pub fn refund<SPEC: Spec, EXT, DB: Database>(
    _context: &mut Context<EXT, DB>,
    gas: &mut Gas,
    eip7702_refund: i64,
) {
    gas.record_refund(eip7702_refund);

    // Calculate gas refund for transaction.
    // If spec is set to london, it will decrease the maximum refund amount to 5th part of
    // gas spend. (Before london it was 2th part of gas spend)
    gas.set_final_refund(SPEC::SPEC_ID.is_enabled_in(SpecId::LONDON));
}

#[inline]
pub fn reimburse_caller<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    gas: &Gas,
) -> Result<(), EVMError<DB::Error>> {
    let caller = context.evm.env.tx.caller;
    let effective_gas_price = context.evm.env.effective_gas_price();

    // return balance of not spend gas.
    let caller_account = context
        .evm
        .inner
        .journaled_state
        .load_account(caller, &mut context.evm.inner.db)?;

    caller_account.data.info.balance =
        caller_account.data.info.balance.saturating_add(
            effective_gas_price * U256::from(gas.remaining() + gas.refunded() as u64),
        );

    Ok(())
}

/// Main return handle, returns the output of the transaction.
#[inline]
pub fn output<EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    result: FrameResult,
) -> Result<ResultAndState, EVMError<DB::Error>> {
    context.evm.take_error()?;
    // used gas with refund calculated.
    let gas_refunded = result.gas().refunded() as u64;
    let final_gas_used = result.gas().spent() - gas_refunded;
    let output = result.output();
    let instruction_result = result.into_interpreter_result();

    // reset journal and return present state.
    let (state, logs) = context.evm.journaled_state.finalize();

    let result = match instruction_result.result.into() {
        SuccessOrHalt::Success(reason) => ExecutionResult::Success {
            reason,
            gas_used: final_gas_used,
            gas_refunded,
            logs,
            output,
        },
        SuccessOrHalt::Revert => ExecutionResult::Revert {
            gas_used: final_gas_used,
            output: output.into_data(),
        },
        SuccessOrHalt::Halt(reason) => ExecutionResult::Halt {
            reason,
            gas_used: final_gas_used,
        },
        // Only two internal return flags.
        flag @ (SuccessOrHalt::FatalExternalError | SuccessOrHalt::Internal(_)) => {
            panic!(
                "Encountered unexpected internal return flag: {:?} with instruction result: {:?}",
                flag, instruction_result
            )
        }
    };

    Ok(ResultAndState { result, state })
}
