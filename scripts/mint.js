const chainConfig = require('./config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
const { GasPrice } = require('@cosmjs/stargate');
const amino = require('@cosmjs/amino');
const {AminoMsg, StdFee} = require('@cosmjs/amino');

function createMessageToSign(chainID, active, passive, uri) {
    const AGREEMENT = 'Agreement(address active,address passive,string tokenURI)';

    // create message to sign based on concating AGREEMENT, signer, receiver, and uri
    const message = AGREEMENT + active + passive + uri;

    // const messageToSign = {
    //     "chain_id": chainID,
    //     "account_number": "0",
    //     "sequence": "0",
    //     "fee": {
    //         "gas": "0",
    //         "amount": []
    //     },
    //     "msgs": [
    //         {
    //             "type": "sign/MsgSignData",
    //             "value": {
    //                 "signer": passive,
    //                 "data": message
    //             }
    //         }
    //     ],
    //     "memo": ""
    // };

    const mess = {
        type: "sign/MsgSignData",
        value: {
            signer: String(passive),
            data: String(message)
        }
    };

    console.log("mess: ", mess);

    const fee = {
        gas: "0",
        amount: []
    };

    console.log("fee: ", fee);

    const messageToSign = amino.makeSignDoc(mess, fee, String(chainID), "",  0, 0);

    return messageToSign;
}

async function getPermitSignatureAmino(messageToSign) {
    const deployerWallet = await amino.Secp256k1HdWallet.fromMnemonic(
        chainConfig.deployer_mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );

    // const adminAccount = deployerWallet.getAccounts()[0];
    const adminAccount = (await deployerWallet.getAccounts())[0];

    // sign message
    const signedDoc = await deployerWallet.signAmino(adminAccount.address, messageToSign);
    console.log(signedDoc);
    
    // convert signature from base64 string to Uint8Array
    const signatureUint8Array = Buffer.from(signedDoc.signature.signature, 'base64');

    // convert signature from base64 string to Uint8Array
    // const signatureUint8Array = Buffer.from(signature.signature.signature, 'base64');

    // console.log(signatureUint8Array);
    // const bytes = new Uint8Array(b);
    // console.log(bytes);

    // const signatureBase64 = amino.encodeSecp256k1Signature(signature.signature.pub_key.value, signatureUint8Array);
    const decodedSignature = amino.decodeSignature(signedDoc.signature);
    console.log(decodedSignature);

    console.log("adminAccount.address: ", adminAccount.address);
    console.log("adminAccount.pubkey: ", Buffer.from(adminAccount.pubkey).toString('base64'));

    // pubkey must be compressed in base64
    let permitSignature = {
        "hrp": "aura",
        "pub_key": Buffer.from(adminAccount.pubkey).toString('base64'),
        "signature": signedDoc.signature.signature,
    }

    // console.log("signature: ", signatureBase64.signature);

    return permitSignature;
}

async function getPermitSignature(messageToSign) {
    const deployerWallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.deployer_mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );

    const adminAccount = (await deployerWallet.getAccounts())[0];

    // sign message
    // const signature = await deployerWallet.signAmino(adminAccount.address, messageToSign);
    const signature = await deployerWallet.signDirect(adminAccount.address, messageToSign);

    const signatureBase64 = amino.encodeSecp256k1Signature(signature.pubkey, signature.signature);

    console.log("adminAccount.address: ", adminAccount.address);
    console.log("adminAccount.pubkey: ", Buffer.from(adminAccount.pubkey).toString('base64'));

    // pubkey must be compressed in base64
    let permitSignature = {
        "hrp": "aura",
        "pub_key": Buffer.from(adminAccount.pubkey).toString('base64'),
        "signature": signatureBase64.signature,
    }

    console.log("signature: ", signatureBase64.signature);

    return permitSignature;
}

async function mint(_contract, _uri) {
    const deployerWallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.deployer_mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );

    const testerWallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.tester_mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );

    // get deployer account
    const deployerAccount = (await deployerWallet.getAccounts())[0];

    // gas price
    const gasPrice = GasPrice.fromString(`0.025${chainConfig.denom}`);

    // connect tester wallet to chain
    const testerClient = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, testerWallet, {gasPrice});

    // get tester account
    const testerAccount = (await testerWallet.getAccounts())[0];

    // create message to sign
    const messageToSign = createMessageToSign(chainConfig.chainId, testerAccount.address, deployerAccount.address, _uri);
    console.log("messageToSign: ", messageToSign);

    // sign message
    const permitSignature = await getPermitSignatureAmino(messageToSign);
    console.log("permitSignature: ", permitSignature);

    const memo = "take nft";
    // define the take message using the address of deployer, uri of the nft and permitSignature
    const ExecuteTakeMsg = {
        "take": {
            "from": deployerAccount.address,
            "uri": _uri,
            "signature": permitSignature,
        }
    }

    console.log("ExecuteTakeMsg: ", ExecuteTakeMsg);

    // take a NFT
    const takeResponse = await testerClient.execute(testerAccount.address, _contract, ExecuteTakeMsg, "auto", memo);

    console.log(takeResponse);
}

const myArgs = process.argv.slice(2);
mint(myArgs[0], myArgs[1]);
