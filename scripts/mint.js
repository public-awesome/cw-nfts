const chainConfig = require('./config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
// const { calculateFee, GasPrice } = require('@cosmjs/stargate');

function createMessageToSign(chainID, active, passive, uri) {
    const AGREEMENT = 'Agreement(address active,address passive,string tokenURI)';

    // create message to sign based on concating AGREEMENT, signer, receiver, and uri
    const message = AGREEMENT + active + passive + uri;

    const messageToSign = {
        "chain_id": chainID,
        "account_number": "0",
        "sequence": "0",
        "fee": {
            "gas": "0",
            "amount": []
        },
        "msgs": [
            {
                "type": "sign/MsgSignData",
                "value": {
                    "signer": passive,
                    "data": message
                }
            }
        ],
        "memo": ""
    };

    return JSON.stringify(messageToSign);
}

function textToBin(text) {
    var length = text.length,
        output = [];
    for (var i = 0; i < length; i++) {
        var bin = text[i].charCodeAt().toString(2);
        output.push(Array(8 - bin.length + 1).join("0") + bin);
    }
    return output.join(" ");
}

async function getPermitSignature(messageToSign) {
    const deployerWallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.deployer_mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );

    const adminAccount = (await deployerWallet.getAccounts())[0];


    const signature = await deployerWallet.signDirect(adminAccount.address, messageToSign);

    const utf8Encode = new TextEncoder();

    let permitSignature = {
        "hrp": "aura",
        "pub_key": signature.signature.pub_key.value,
        "signature": signature.signature.signature,
    }

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

    // connect tester wallet to chain
    const testerClient = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, testerWallet);

    // get tester account
    const testerAccount = (await testerWallet.getAccounts())[0];

    // create message to sign
    const messageToSign = createMessageToSign(chainConfig.chainId, testerAccount.address, deployerAccount.address, _uri);
    console.log("messageToSign: ", messageToSign);

    // sign message
    const permitSignature = await getPermitSignature(messageToSign);
    console.log("permitSignature: ", permitSignature);

    // config gas price
    const defaultFee = { amount: [{ amount: "250000", denom: chainConfig.denom, },], gas: "250000", };
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
    const takeResponse = await testerClient.execute(testerAccount.address, _contract, ExecuteTakeMsg, defaultFee, memo);

    console.log(takeResponse);
}

const myArgs = process.argv.slice(2);
mint(myArgs[0], myArgs[1]);
