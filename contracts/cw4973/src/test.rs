#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, DepsMut, Empty, Binary, CanonicalAddr,};
use sha2::{Digest, Sha256};
use bech32::{ToBase32, Variant::Bech32};
use ripemd::{Ripemd160};
use std::{str, env};
use base64;
use bip39::{Mnemonic, MnemonicType, Language, Seed};
use secp256k1::{Secp256k1, Message, };

pub use crate::state::{ADR36SignDoc, Fee, MsgSignData, MsgSignDataValue, PermitSignature};


use crate::{
    ContractError, Cw4973Contract, ExecuteMsg, InstantiateMsg, QueryMsg, 
};

pub use cw721_base::{
    QueryMsg as Cw721BaseQueryMsg, Extension, InstantiateMsg as Cw721BaseInstantiateMsg,
};

pub use cw721::{ContractInfoResponse, NftInfoResponse,};

const MINTER: &str = "minter";

fn setup_contract(deps: DepsMut<'_>) -> Cw4973Contract {
    let contract = Cw4973Contract::default();
    let msg = Cw721BaseInstantiateMsg {
        name: "Aura 4973".to_string(),
        symbol: "A4973".to_string(),
        minter: String::from(MINTER),
    };

    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    // prepare the mock dependencies
    let mut deps = mock_dependencies();
    
    // setup the contract
    setup_contract(deps.as_mut());

    // get contract by using default Aura4973 contract
    let contract = Cw4973Contract::default();

    // prepare the env
    let env = mock_env();

    // prepare the ContractInfo query
    let query_msg = Cw721BaseQueryMsg::from(QueryMsg::ContractInfo {});

    // it worked, let's query the state
    let res = contract.query(deps.as_ref(), mock_env(), query_msg).unwrap();
    let value: ContractInfoResponse = from_binary(&res).unwrap();
    assert_eq!("Aura 4973", value.name);
    assert_eq!("A4973", value.symbol);
}

// generate keypair and address from mnemonic for testing using Secp256k1
fn generate_keypair(mnemonic: &str) -> (Vec<u8>, String) {
    let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");
    let secp = Secp256k1::new();

    // generate private key and public key
    let private_key = secp256k1::SecretKey::from_slice(&seed.as_bytes()[0..32]).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &private_key);

    // get the hash of the pubkey bytes
    let pk_hash = Sha256::digest(&public_key.serialize()[1..]);

    // Insert the hash result in the ripdemd hash function
    let mut rip_hasher = Ripemd160::default();
    rip_hasher.update(pk_hash);
    let rip_result = rip_hasher.finalize();

    let address_bytes = CanonicalAddr(Binary(rip_result.to_vec()));

    let base32_addr = address_bytes.0.as_slice().to_base32();

    let bech32_address = bech32::encode("aura", base32_addr, Bech32)
        .map_err(|err| ContractError::Hrp(err.to_string()));

    // return the private key, public key, and address
    (public_key.serialize().to_vec(), bech32_address.unwrap())
}

// test the user can take nft from contract after minter provide permit by a signature
#[test]
fn test_user_can_take_nft_after_minter_provide_permit_by_signature() {
    // prepare the mock dependencies
    let mut deps = mock_dependencies();

    // setup the contract
    setup_contract(deps.as_mut());

    // get contract by using default Aura4973 contract
    let contract = Cw4973Contract::default();

    // prepare the env
    let env = mock_env();

    // prepare the ContractInfo query
    let query_msg = Cw721BaseQueryMsg::from(QueryMsg::ContractInfo {});

    // it worked, let's query the state
    let res = contract.query(deps.as_ref(), mock_env(), query_msg).unwrap();
    let value: ContractInfoResponse = from_binary(&res).unwrap();
    assert_eq!("Aura 4973", value.name);
    assert_eq!("A4973", value.symbol);

    // prepare the minter info including the private key, public key and address
    let minter_info = mock_info(MINTER, &[]);

    // prepare Agreement const
    const AGREEMENT: &str = "Agreement(address active,address passive,string tokenURI)";

    // prepare the Agreement message by concatenating the Agreement const, user's address as active address, minter as passive address, and the tokenURI
    let agreement_message = format!("{}{}{}{}", AGREEMENT, "user", MINTER, "tokenURI");

    // prepare the signature by admin signs the agreement message

}

