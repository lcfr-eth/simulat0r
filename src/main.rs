mod opt;

use std::sync::Arc;
use eyre::Result;

use foundry_evm::{ fork::database::ForkedDatabase };
use foundry_fork_db::{cache::BlockchainDbMeta, BlockchainDb, SharedBackend};

use revm_primitives::TransactTo;
use revm::Evm;
use std::collections::BTreeSet;

use alloy_provider::{network::AnyNetwork, Provider, ProviderBuilder, RootProvider};
use alloy_rpc_client::ClientBuilder;
use alloy_transport_http::{Client, Http};
use alloy_rpc_types::{Block, BlockId, BlockNumberOrTag};
use alloy_primitives::{U256};

use clap::Parser;
use crate::opt::Cli;

pub fn new_evm(fork_db: ForkedDatabase, block: Block) -> Evm<'static, (), ForkedDatabase> {
    let mut evm = Evm::builder().with_db(fork_db).build();

    evm.block_mut().number = U256::from(block.header.number.unwrap());
    evm.block_mut().timestamp = U256::from(block.header.timestamp);
    evm.block_mut().coinbase = block.header.miner;

    evm.cfg_mut().disable_block_gas_limit = true;
    evm.cfg_mut().disable_base_fee = true;
    evm
}

pub fn get_http_provider(endpoint: &str) -> RootProvider<Http<Client>, AnyNetwork> {
    ProviderBuilder::new()
        .network::<AnyNetwork>()
        .on_client(ClientBuilder::default().http(endpoint.parse().unwrap()))
}

#[tokio::main]
async fn main() -> Result<()> {

    let cli = Cli::parse();
    let provider = get_http_provider(&cli.rpc_url);

    let block_number: Option<u64> = if let Some(block) = cli.block_number {
        match block.parse::<u64>() {
            Ok(num) => Some(num),
            Err(_) => {
                eprintln!("Invalid block number provided.");
                None
            }
        }
    } else {
        None
    };

    let meta = BlockchainDbMeta {
        cfg_env: Default::default(),
        block_env: Default::default(),
        hosts: BTreeSet::from([cli.rpc_url.to_string()]), 
    };

    let db = BlockchainDb::new(meta, None);

    let backend = SharedBackend::spawn_backend(
        Arc::new(provider.clone()), 
        db.clone(), 
        block_number.map(Into::into)
    ).await;

    let db = ForkedDatabase::new(backend, db);

    let latest_block = provider.get_block_number().await?;
    let block_id = BlockId::Number(BlockNumberOrTag::Number(
        block_number.unwrap_or(latest_block)
    ));
    let block = provider.get_block(block_id.clone(), true.into()).await?;

    let mut evm = new_evm(db, block.unwrap());

    evm.tx_mut().transact_to = TransactTo::Call(cli.to.parse().unwrap());
    evm.tx_mut().caller = "0x0000000000000000000000000000000000000000".parse().unwrap();
    if let Some(from) = &cli.from {
        evm.tx_mut().caller = from.parse().unwrap();
    }
    if let Some(value) = &cli.value {
        evm.tx_mut().value = value.parse().unwrap();
    }
    if let Some(data) = &cli.data {
        evm.tx_mut().data = data.parse().unwrap();
    }

    let result = evm.transact().unwrap().result;
    println!("{:#?}", result);

    Ok(())
}