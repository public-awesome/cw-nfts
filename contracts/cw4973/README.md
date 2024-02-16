# Cw4973 - Account-bound Tokens (ABT)

Cw4973 is implemented based on the description of [EIP-4973](https://eips.ethereum.org/EIPS/eip-4973) with some details changed to be suitable with Cosmos ecosystem.

## Specification
Cw4973 is extended from Cw721-base with some modifications in execution section:

- `ContractInfo` structures is unchanged.
- All execute functions are removed. There will be no `transfer`, no `approve`, etc.
- All query functions are unchanged.
- We assume the owner of this contract is the only ABT issuer.
- We add 3 execute functions:

    - `Give`
    - `Take`
    - `Unequip`

In order for a new ABT to be minted, both issuer and receiver must give explicit consent by signing the following `signDoc`:
```json
{
  chain_id: "cosmoshub",
  account_number: "0",
  sequence: "0",
  fee: { gas: "0", amount: [] },
  msgs: {
    type: "sign/MsgSignData",
    value: {
      signer: "cosm1address",
      data: "Agreement(address active,address passive,string tokenURI)cosm1address1cosm1address2https://example.com/abt/1.json"
    }
  },
  memo: ""
}
```
`account_number`, `sequence`, `fee` and `memo` are added with default values for compatible with [StdSignDoc](https://cosmos.github.io/cosmjs/latest/amino/interfaces/StdSignDoc.html).
Either issuer or receiver will provide their signature while the other will submit a transaction using function `give` or `take` accordingly.
An ABT token will be minted with `token_id` calculated as hash of the whole `signDoc`.

### `Give`
This must be called by owner of this contract to give an ABT to address `to`:
```rust
Give {
    to: String,
    uri: String,
    signature: PermitSignature,
}
```
In which `PermitSignature` is:
```rust
pub struct PermitSignature {
    pub hrp: String,
    pub pub_key: String,
    pub signature: String,
}
```
The following event will be emitted:
```json
{
    "action": "transfer",
    "token_id": token_id,
    "owner": sender
}
```

### `Take`
This must be called by ABT receivers to get an ABT from this contract's owner:
```rust
Take {
    from: String,
    uri: String,
    signature: PermitSignature,
}
```
`from` should be the same as this contract's owner. A new token will be minted to transaction's sender.

### `Unequip`
This function must be called by an ABT owner to burn an ABT:
```
Unequip { token_id: String }
```
The following event will be emitted:
```json
{
    "action": "unequip",
    "token_id": token_id,
    "owner": sender
}
```

## Example
In this section, we will provide the examples of the signable message, the signature and execute messages being used in the contract. 

### Generate a signDoc
We could use the following code snippet to create a singable message using cosmjs.
```javascript
const amino = require('@cosmjs/amino');

function createMessageToSign(chainID, active, passive, uri) {
    const AGREEMENT = 'Agreement(address active,address passive,string tokenURI)';

    // create message to sign based on concating AGREEMENT, signer, receiver, and uri
    const message = {
        type: "sign/MsgSignData",
        value: {
            signer: passive,
            data: AGREEMENT + active + passive + uri;
        }
    };

    const fee = {
        gas: "0",
        amount: []
    };

    return amino.makeSignDoc(message, fee, chainID, "",  0, 0);
}
```

### Sign the message
In the following example, we sign a message which can be use with `take` to mint an ABT. The account used to sign the message should be the owner (issuer) of the ABT contract.
```javascript
// signed message
const messageToSign = createMessageToSign(chainID, active, passive, uri);
const signedDoc = await wallet.signAmino(issuer, messageToSign);

let permitSignature = {
    "hrp": "cosm",
    "pub_key": Buffer.from(adminAccount.pubkey).toString('base64'),
    "signature": signedDoc.signature.signature,
}
```

`hrp` is "human readable prefix" value which is used to generate account address from public key for each blockchain.

