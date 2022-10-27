'use strict';

const local = {
    rpcEndpoint: 'http://localhost:26657',
    prefix: 'aura',
    denom: 'uaura',
    chainId: 'local-aura',
    broadcastTimeoutMs: 2000,
    broadcastPollIntervalMs: 500
};

const localDocker = {
    rpcEndpoint: 'http://dev-aurad:26657',
    prefix: 'aura',
    denom: 'uaura',
    chainId: 'local-aura',
    broadcastTimeoutMs: 2000,
    broadcastPollIntervalMs: 500
};

const serenity = {
    rpcEndpoint: 'https://rpc.serenity.aura.network',
    prefix: 'aura',
    denom: 'uaura',
    chainId: 'serenity-testnet-001',
    broadcastTimeoutMs: 5000,
    broadcastPollIntervalMs: 1000
};

const auraTestnet = {
    rpcEndpoint: 'https://rpc.dev.aura.network',
    prefix: 'aura',
    denom: 'utaura',
    chainId: 'aura-testnet',
    broadcastTimeoutMs: 5000,
    broadcastPollIntervalMs: 1000
};

const euphoria = {
    rpcEndpoint: 'https://rpc.euphoria.aura.network',
    prefix: 'aura',
    denom: 'ueaura',
    chainId: 'euphoria-1',
    broadcastTimeoutMs: 5000,
    broadcastPollIntervalMs: 1000
};

let defaultChain = serenity;
// switch (process.env.CHAIN_ID) {
//   case 'euphoria':
//     defaultChain = euphoria;
//     break;
//   case 'serenity':
//     defaultChain = serenity;
//     break;
//   case 'local-docker':
//     defaultChain = localDocker;
//     break;
//   case 'aura-testnet':
//     defaultChain = auraTestnet;
//     break;
//   default:
//     defaultChain = local;
//     break;
// }

defaultChain.deployer_mnemonic = process.env.MNEMONIC
    || 'grief assault labor select faint leader impulse broken help garlic carry practice cricket cannon draw resist clump jar debris sentence notice poem drip benefit';

defaultChain.tester_mnemonic = 'forward picnic antenna marble various tilt problem foil arrow animal oil salon catch artist tube dry noise door cliff grain fox left loan reopen';


module.exports = {
    local,
    serenity,
    euphoria,
    auraTestnet,
    defaultChain
};