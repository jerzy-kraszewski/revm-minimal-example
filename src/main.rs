use std::sync::Arc;

use alloy::{eips::BlockId, providers::ProviderBuilder, sol_types::SolCall};
use anyhow::Result;
use log::info;
use revm::{
    db::{AlloyDB, CacheDB},
    primitives::{address, AccountInfo, Bytecode, TransactTo},
    Evm,
};
use revm_minimal_example::{
    models::{parse_result, Counter},
    utils::setup_logger,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    setup_logger()?;

    let sender = address!("0xaaaaaaa0B01a61e6B8609482608493a6748d1781");
    let rpc_url = std::env::var("RPC_URL").unwrap();
    info!("RPC URL: {}", rpc_url);

    let provider = ProviderBuilder::new().on_builtin(&rpc_url).await?;
    let provider = Arc::new(provider);
    let mut db = CacheDB::new(AlloyDB::new(provider.clone(), BlockId::default()).unwrap());

    let counter_bytecode = Bytecode::new_raw(Counter::DEPLOYED_BYTECODE.clone());
    let counter_address = address!("0x1234567890abcdef2ec9065E61B39BB9E4d82513");
    let counter_account = AccountInfo::from_bytecode(counter_bytecode);
    db.insert_account_info(counter_address, counter_account);

    let evm = Evm::builder().with_ref_db(db).build();

    let number_call = Counter::numberCall {};
    let calldata = Counter::numberCall::abi_encode(&number_call);

    let mut evm = evm
        .modify()
        .modify_tx_env(|tx| {
            tx.caller = sender;
            tx.transact_to = TransactTo::Call(counter_address);
            tx.data = calldata.into();
        })
        .build();

    let result = evm.transact();
    let tx_result = parse_result(result.unwrap().result)?;

    let number = match Counter::numberCall::abi_decode_returns(&tx_result.output, true) {
        Ok(n) => n,
        Err(e) => {
            panic!("Error decoding number: {}", e);
        }
    };
    let number = number._0;

    info!("Counter number: {}", number);

    Ok(())
}
