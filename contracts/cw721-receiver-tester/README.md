# Cw721 Receiver

This contract can receive a cw721 token sent via the `SendNft` message.
It expects a json message of either `"succeed"` or `"fail"` (mind the quotes).
So an example message would look like this:

```json
{
  "send_nft": {
    "contract": "CW721_CONTRACT_ADDR",
    "token_id": "test",
    "msg": "InN1Y2NlZWQi" // <- base64 encoded "succeed"
  }
}
```

In case of `"succeed"` the contract returns a response with its input data as
attributes and data. In case of `"fail"` the contract returns an error.
