use anyhow::Result;
use cosm_orc::{
    config::{
        cfg::Config,
        key::{Key, SigningKey},
    },
    orchestrator::cosm_orc::CosmOrc,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Account {
    name: String,
    address: String,
    mnemonic: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let config = env::var("CONFIG").expect("missing yaml CONFIG env var");
    let mut cfg = Config::from_yaml(&config)?;
    let mut orc = CosmOrc::new(cfg.clone(), false)?;

    // use first test user as DAO admin, and only DAO member:
    let accounts: Vec<Account> =
        serde_json::from_slice(&fs::read("ci/configs/test_accounts.json")?)?;
    let account = accounts[0].clone();

    let key = SigningKey {
        name: account.name,
        key: Key::Mnemonic(account.mnemonic),
    };
    let addr = account.address;

    orc.poll_for_n_blocks(1, Duration::from_millis(20_000), true)?;

    orc.store_contracts("artifacts", &key, None)?;

    let msg = cw721_base::msg::InstantiateMsg {
        name: "token".to_string(),
        symbol: "nonfungible".to_string(),
        minter: addr.clone(),
    };

    let res = orc.instantiate(
        "cw721_base",
        "cw721_base_init",
        &msg,
        &key,
        Some(addr.clone()),
        vec![],
    )?;

    println!(" ------------------------ ");

    println!("admin / minter address: {}", addr);
    println!("cw721_base address: {}", res.address);

    // Persist contract code_ids in local.yaml so we can use
    // SKIP_CONTRACT_STORE locally to avoid having to re-store them
    // again
    cfg.contract_deploy_info = orc.contract_map.deploy_info().clone();
    fs::write(
        "ci/configs/cosm-orc/local.yaml",
        serde_yaml::to_string(&cfg)?,
    )?;

    Ok(())
}
