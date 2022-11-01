const chainConfig = require('./config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
const { GasPrice } = require('@cosmjs/stargate');
const amino = require('@cosmjs/amino');
const {AminoMsg, StdFee} = require('@cosmjs/amino');

// create function to transfer a NFT
async function transfer(_contract, _tokenId, _to) {
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

    // define the transfer message using the address of tester and id of the nft
    const ExecuteTransferMsg = {
        transfer_nft: {
            recipient: _to,
            token_id: _tokenId,
        },
    };

    // transfer a NFT
    const transferResponse = await testerClient.execute(testerAccount.address, _contract, ExecuteTransferMsg, "auto", "transfer nft");

    // log response
    console.log(transferResponse);

}

const myArgs = process.argv.slice(2);
transfer(myArgs[0], myArgs[1], myArgs[2]);
