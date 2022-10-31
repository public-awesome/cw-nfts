pub use crate::msg::{InstantiateMsg, QueryMsg, ExecuteMsg};
pub use crate::error::ContractError;
pub use crate::state::{ADR36SignDoc, Fee, MsgSignData, MsgSignDataValue, PermitSignature};
use cosmwasm_std::{Binary, CanonicalAddr, Empty};
pub use cw721_base::{
    entry::{execute as _execute, query as _query},
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721BaseExecuteMsg, Extension, InstantiateMsg as Cw721BaseInstantiateMsg,
    MintMsg, MinterResponse,
    state::TokenInfo,
};

use sha2::{Digest, Sha256};
use bech32::{ToBase32, Variant::Bech32};
use ripemd::{Ripemd160};
use std::{str, env};
use base64;

pub mod error;
pub mod msg;
pub mod state;
pub mod test;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw4973";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const AGREEMENT_STRING: &str = "Agreement(address active,address passive,string tokenURI)";

pub type Cw4973Contract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use cosmwasm_std::{
        entry_point, DepsMut, Env, MessageInfo, Response,
    };

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
            ExecuteMsg::Give { to, uri, signature } => execute_give(deps, env, info, to, uri, signature),
            ExecuteMsg::Take { from, uri, signature } => execute_take(deps, env, info, from, uri, signature),
            ExecuteMsg::UnEquip { nft_id } => execute_unequip(deps, env, info, nft_id),
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
        // Cannot execute this function if the sender is not the minter get from cw721 contract
        let minter = Cw4973Contract::default().minter.load(deps.storage)?;
        if minter != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        // get the nft id using _safeCheckAgreement function
        let nft_id = _safe_check_agreement(&deps,  &env,&minter.into_string(), &to, &uri, &signature);
        
        // check if the nft id is empty, then return error
        if nft_id.is_empty() {
            return Err(ContractError::To {});
        }

        // mint the nft to the address 'to' and return the response of mint function
        let mint_msg = MintMsg {
            token_id: nft_id.to_string(),
            owner: to.to_string(),
            token_uri: uri.into(),
            extension: None,
        };
        
        // execute the mint message
        _min(deps, env, info, mint_msg).ok();

        // return the response of mint function
        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("nft_id", nft_id)
            .add_attribute("owner", to))
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
        // check 'from' is a valid human's address
        let from_addr = deps.api.addr_validate(&from)?;

        // Cannot take NFT from a user who is not the minter
        let minter = Cw4973Contract::default().minter.load(deps.storage)?;
        if from_addr != minter {
            return Err(ContractError::Unauthorized {});
        }

        // get address of the owner of the nft from info.sender
        let owner = info.sender.clone();

        // get the nft id using _safeCheckAgreement function
        let nft_id = _safe_check_agreement(&deps, &env, &owner.to_string(), &from, &uri, &signature);
        
        // check if the nft id is empty, then return error
        if nft_id.is_empty() {
            return Err(ContractError::From {});
        }

        // create ExecuteMsg::Mint with Option<Extension> = None
        let mint_msg = MintMsg {
            token_id: nft_id.to_string(),
            owner: owner.to_string(),
            token_uri: uri.into(),
            extension: None,
        };
        
        // execute the mint message
        _min(deps, env, info, mint_msg).ok();

        // return the response of mint function
        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("nft_id", nft_id)
            .add_attribute("owner", owner))

    }

    // execute_unequip is a function that allows the owner of a nft to unequip it by set the equiped field to false
    pub fn execute_unequip(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        nft_id: String,
    ) -> Result<Response, ContractError> {
        // create execute message to burn the nft using the nft_id and ExecuteMsg::Burn of cw721-base
        let burn_msg = Cw721BaseExecuteMsg::Burn { token_id: nft_id.clone() };
        // burn the nft with the given id
        Cw4973Contract::default().execute(deps, _env, info.clone(), burn_msg).ok();

        // return response
        Ok(Response::new()
            .add_attribute("action", "unequip")
            .add_attribute("nft_id", nft_id)
            .add_attribute("owner", info.sender))
    }

    // _get_bech32_address function is used to get the bech32 address from the public key and hrp
    fn _get_bech32_address(hrp: &str, pubkey: &[u8]) -> Result<String, ContractError> {
        // get the hash of the pubkey bytes
        let pk_hash = Sha256::digest(&pubkey);

        // Insert the hash result in the ripdemd hash function
        let mut rip_hasher = Ripemd160::default();
        rip_hasher.update(pk_hash);
        let rip_result = rip_hasher.finalize();

        let address_bytes = CanonicalAddr(Binary(rip_result.to_vec()));

        let base32_addr = address_bytes.0.as_slice().to_base32();

        let bech32_address = bech32::encode(hrp, base32_addr, Bech32)
            .map_err(|err| ContractError::Hrp(err.to_string()))?;

        Ok(bech32_address)
    }

    // _safeCheckAgreement function is used to check if the agreement is valid or not
    // if the agreement is valid then it returns the id of the nft
    fn _safe_check_agreement(
        deps: &DepsMut,
        env: &Env,
        active: &String,
        passive: &String,
        uri: &String,
        signature: &PermitSignature,
    ) -> String {
        // get chain id from blockinfo of env
        let chain_id = env.block.chain_id.clone();

        // get hash for the agreement
        let hash = _get_hash(&active, &passive, &uri, &chain_id);

        // get the signature value from the signature
        let sig = signature.signature.clone();
        
        // get the public key from the signature
        let pubkey = signature.pub_key.clone();

        // decode the public key
        let pubkey_bytes = base64::decode(pubkey).unwrap();

        // verify the signature using the hash and the public key
        let is_verified = deps.api.secp256k1_verify(&hash, sig.as_bytes(), pubkey_bytes.as_slice());
        match is_verified {
            Ok(_) => {
                // If the signature is verified then we must check the address of public key is equal to passive address
                // get hrp from the signature
                let hrp = signature.hrp.clone();

                // get the address of signer from the public key using get_bech32_address function
                let signer_address = _get_bech32_address(&hrp, pubkey_bytes.as_slice()).unwrap();

                // check if the recovered address is same as the 'to' address, then return empty string
                if signer_address != *passive {
                    return "".to_string();
                } else {
                    // the id of the nft is the hash of hash value using sha256
                    let nft_id = Sha256::digest(&hash);

                    // return the nft id as string
                    return str::from_utf8(nft_id.as_slice()).unwrap().to_string();
                }
            }
            Err(_) => {
                // if the signature is not verified then return empty string
                return "".to_string();
            }
        }
    }

    // the get_hash funtion will concat the address of the sender, the address of the 'to', the uri of the nft and the hash of the string
    fn _get_hash(
        active: &String,
        passive: &String,
        uri: &String,
        chain_id: &String,
    ) -> Vec<u8> {
        // hash the constant string and data
        let big_string = format!("{}{}{}{}", AGREEMENT_STRING, active, passive, uri);

        // get the signing document
        let sign_doc = _get_sign_doc(passive, &big_string, chain_id);

        // convert the signDoc to json
        let sign_doc_json = serde_json::to_string(&sign_doc).unwrap();

        let hash = Sha256::digest(sign_doc_json.as_bytes());
        
        // return the hash
        return hash.as_slice().to_vec();
    }

    // rewrite mint function of cw721 base to ignore minter checking
    fn _min(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: MintMsg<Extension>,
    ) -> Result<Response, Cw721ContractError> {
        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&msg.owner)?,
            approvals: vec![],
            token_uri: msg.token_uri,
            extension: msg.extension,
        };

        // update tokens list of contract
        Cw4973Contract::default().tokens
            .update(deps.storage, &msg.token_id, |old| match old {
                Some(_) => Err(Cw721ContractError::Claimed {}),
                None => Ok(token),
            })?;

        Cw4973Contract::default().increment_tokens(deps.storage)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("owner", msg.owner)
            .add_attribute("token_id", msg.token_id))
    }

    // TODO: modify this function to specify the others fields of the signing document
    // OR: create new contract to handle the signing document is better
    // create signable structure from message and chain ID
    // @param signer: the address of the signer
    // @param message: the message to sign
    // @param chain_id: the chain id of the chain
    // @return: the signable structure
    fn _get_sign_doc(
        signer: &String,
        message: &String,
        chain_id: &String,
    ) -> ADR36SignDoc {
        // create signable structure
        let doc = ADR36SignDoc {
            chain_id: chain_id.to_string(),
            account_number: "0".to_string(),
            sequence: "0".to_string(),
            fee: Fee {
                gas: "0".to_string(),
                amount: [].to_vec(),
            },
            msgs: [MsgSignData {
                r#type: "sign/MsgSignData".to_string(),
                value: MsgSignDataValue {
                    signer: signer.to_string(),
                    data: message.to_string().into_bytes(),
                },
            }].to_vec(),
            memo: "".to_string()
        };
    
        doc
    }
}
