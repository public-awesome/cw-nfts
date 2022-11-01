const chainConfig = require('./config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
const { GasPrice } = require('@cosmjs/stargate');
const amino = require('@cosmjs/amino');
const {AminoMsg, StdFee} = require('@cosmjs/amino');

// create function to unequip a NFT
async function unequip(_contract, _tokenId) {
    // init wallet of tester from mnemonic
    const testerWallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.tester_mnemonic,
        {prefix: chainConfig.prefix},
    );

    // get tester account
    const testerAccount = (await testerWallet.getAccounts())[0];

    // gas price
    const gasPrice = GasPrice.fromString(`0.025${chainConfig.denom}`);

    // connect tester wallet to chain
    const testerClient = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, testerWallet, {gasPrice});

    // define the token info message using id of nft
    const QueryTokenInfoMsg = {
        nft_info: {
            token_id: _tokenId,
        },
    };

    // check the ownership of NFT
    const infor1 = await testerClient.queryContractSmart(_contract, QueryTokenInfoMsg);
    console.log("infor1: ", infor1);

    // define the owner of NFT message using id of nft
    const QueryOwnerMsg = {
        owner_of: {
            token_id: _tokenId,
        },
    };

    // check the ownership of NFT
    const owner = await testerClient.queryContractSmart(_contract, QueryOwnerMsg);
    console.log("owner: ", owner);

    // define the unequip message using id of nft
    const ExecuteUnequipMsg = {
        unequip: {
            token_id: _tokenId,
        },
    };

    // unequip a NFT
    const unequipResponse = await testerClient.execute(testerAccount.address, _contract, ExecuteUnequipMsg, "auto", "unequip nft");

    // log response
    console.log(unequipResponse);

    // recheck the ownership of NFT
    const infor2 = await testerClient.queryContractSmart(_contract, QueryTokenInfoMsg);
    console.log("infor2: ", infor2);
    
}

const myArgs = process.argv.slice(2);
unequip(myArgs[0], myArgs[1]);
