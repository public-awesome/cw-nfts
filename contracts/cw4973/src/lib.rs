pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg};
pub use crate::state::{ADR36SignDoc, Fee, MsgSignData, MsgSignDataValue, PermitSignature};
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
pub use cw721_base::{
    entry::{execute as _execute, query as _query},
    state::TokenInfo,
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721BaseExecuteMsg,
    Extension, InstantiateMsg as Cw721BaseInstantiateMsg, MintMsg, MinterResponse,
    QueryMsg as Cw721QueryMsg,
};

use bech32::{ToBase32, Variant::Bech32};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::{env, str};

pub mod error;
pub mod msg;
pub mod state;
pub mod test;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw4973";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const AGREEMENT_STRING: &str =
    "Agreement(string chain_id,address active,address passive,string tokenURI)";

pub type Cw4973Contract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;
#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        // set contract's information
        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
        };

        Cw4973Contract::default().instantiate(
            deps.branch(),
            env,
            info,
            cw721_base_instantiate_msg,
        )?;

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Give { to, uri, signature } => {
                execute_give(deps, env, info, to, uri, signature)
            }
            ExecuteMsg::Take {
                from,
                uri,
                signature,
            } => execute_take(deps, env, info, from, uri, signature),
            ExecuteMsg::Unequip { token_id } => execute_unequip(deps, env, info, token_id),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw4973Contract::default().query(deps, env, msg)
    }
}

// execute_give function is used to give a nft to another address
// only the minter can call this function
fn execute_give(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: String,
    uri: String,
    signature: PermitSignature,
) -> Result<Response, ContractError> {
    // cannot give to yourself
    if info.sender == to {
        return Err(ContractError::CannotGiveToSelf);
    }

    // Cannot execute this function if the sender is not the minter get from cw721 contract
    let minter = Cw4973Contract::default().minter.load(deps.storage)?;
    if minter != info.sender {
        return Err(ContractError::Unauthorized);
    }

    // get the nft id using _safe_check_agreement function
    let nft_id = _safe_check_agreement(&deps, &env, &minter.into_string(), &to, &uri, &signature)?;

    // mint the nft to the address 'to' and return the response of mint function
    let mint_msg = MintMsg {
        token_id: nft_id,
        owner: to,
        token_uri: uri.into(),
        extension: None,
    };
    _mint(deps, env, info, mint_msg)
}

// execute_take function is used to take a nft from another address
// the user can only take nft from minter
pub fn execute_take(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    uri: String,
    signature: PermitSignature,
) -> Result<Response, ContractError> {
    // cannot take from yourself
    if info.sender == from {
        return Err(ContractError::CannotTakeFromSelf);
    }

    // check 'from' is a valid human's address
    let from_addr = deps.api.addr_validate(&from)?;

    // Cannot take NFT from a user who is not the minter
    let minter = Cw4973Contract::default().minter.load(deps.storage)?;
    if from_addr != minter {
        return Err(ContractError::Unauthorized);
    }

    // get address of the owner of the nft from info.sender
    let owner = &info.sender;

    // get the nft id using _safe_check_agreement function
    let nft_id = _safe_check_agreement(&deps, &env, owner.as_str(), &from, &uri, &signature)?;

    let mint_msg = MintMsg {
        token_id: nft_id,
        owner: owner.to_string(),
        token_uri: uri.into(),
        extension: None,
    };
    _mint(deps, env, info, mint_msg)
}

// execute_unequip is a function that allows the owner of a nft to unequip it by set the equiped field to false
pub fn execute_unequip(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    // create execute message to burn the nft using the nft_id and ExecuteMsg::Burn of cw721-base
    let burn_msg = Cw721BaseExecuteMsg::Burn { token_id };
    // burn the nft with the given id
    match Cw4973Contract::default().execute(deps, _env, info, burn_msg) {
        Ok(r) => Ok(r),
        Err(e) => Err(ContractError::Cw721ContractError(e)),
    }
}

// _get_bech32_address function is used to get the bech32 address from the public key and hrp
fn _get_bech32_address(hrp: &str, pubkey: &[u8]) -> Result<String, ContractError> {
    let pk_hash = Sha256::digest(pubkey);
    let rip_result = Ripemd160::digest(pk_hash);

    let address_bytes = rip_result.to_base32();
    let bech32_address = bech32::encode(hrp, address_bytes, Bech32)
        .map_err(|err| ContractError::Hrp(err.to_string()))?;

    Ok(bech32_address)
}

// _safeCheckAgreement function is used to check if the agreement is valid or not
// if the agreement is valid then it returns the id of the nft
fn _safe_check_agreement(
    deps: &DepsMut,
    env: &Env,
    active: &str,
    passive: &str,
    uri: &str,
    signature: &PermitSignature,
) -> Result<String, ContractError> {
    // get chain id from blockinfo of env
    let chain_id = env.block.chain_id.clone();

    // get hash for the agreement
    let hash = _get_hash(active, passive, uri, &chain_id);

    // get the signature value from the signature
    let sig = base64::decode(signature.signature.clone()).unwrap();

    // get the public key from the signature
    let pubkey = signature.pub_key.clone();

    // decode the public key
    let pubkey_bytes = base64::decode(pubkey).unwrap();

    // verify the signature using the hash and the public key
    let is_verified = deps.api.secp256k1_verify(&hash, &sig, &pubkey_bytes);

    match is_verified {
        Ok(true) => {
            // If the signature is verified then we must check the address of public key is equal to passive address
            // get hrp from the signature
            let hrp = signature.hrp.clone();

            // get the address of signer from the public key using get_bech32_address function
            let signer_address = _get_bech32_address(&hrp, pubkey_bytes.as_slice()).unwrap();

            if signer_address != *passive {
                Err(ContractError::InvalidSigner)
            } else {
                // return hex encoded hash
                Ok(hex::encode(hash))
            }
        }
        Ok(false) => Err(ContractError::InvalidSignature),
        Err(_) => Err(ContractError::CannotVerifySignature),
    }
}

// the get_hash funtion will concat the address of the sender, the address of the 'to', the uri of the nft and the hash of the string
fn _get_hash(active: &str, passive: &str, uri: &str, chain_id: &str) -> Vec<u8> {
    // hash the constant string and data
    let big_string = base64::encode(format!(
        "{}{}{}{}{}",
        AGREEMENT_STRING, chain_id, active, passive, uri
    ));

    // get the signing document
    let sign_doc_json = _get_sign_doc(passive, &big_string);

    let hash = Sha256::digest(sign_doc_json.as_bytes());

    // return the hash
    return hash.as_slice().to_vec();
}

// rewrite mint function of cw721 base to ignore minter checking
fn _mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    // create the token
    let token = TokenInfo {
        owner: deps.api.addr_validate(&msg.owner)?,
        approvals: vec![],
        token_uri: msg.token_uri,
        extension: msg.extension,
    };

    // update tokens list of contract
    Cw4973Contract::default()
        .tokens
        .update(deps.storage, &msg.token_id, |old| match old {
            Some(_) => Err(ContractError::Claimed),
            None => Ok(token),
        })?;
    Cw4973Contract::default().increment_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("owner", msg.owner)
        .add_attribute("token_id", msg.token_id))
}
// OR: create new contract to handle the signing document is better
// create signable structure from message and chain ID
// @param signer: the address of the signer
// @param message: the message to sign
// @param chain_id: the chain id of the chain
// @return: the signable structure
// TODO: modify this function to specify the others fields of the signing document
fn _get_sign_doc(signer: &str, message: &str) -> String {
    // create signable structure
    let doc = ADR36SignDoc {
        account_number: "0".to_string(),
        chain_id: "".to_string(),
        fee: Fee {
            amount: [].to_vec(),
            gas: "0".to_string(),
        },
        memo: "".to_string(),
        msgs: [MsgSignData {
            r#type: "sign/MsgSignData".to_string(),
            value: MsgSignDataValue {
                data: message.to_string(),
                signer: signer.to_string(),
            },
        }]
        .to_vec(),
        sequence: "0".to_string(),
    };

    // convert the signable structure to string
    serde_json::to_string(&doc).unwrap()
}
