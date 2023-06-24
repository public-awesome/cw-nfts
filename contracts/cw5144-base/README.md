# CW5144 Base

This is a basic implementation of a Soul Bound Token CW5144 contract,
created from the (ERC-5144 specification)[https://eips.ethereum.org/EIPS/eip-5114].

It is used to create a non-fungible soul bound token that cannot be transferred or altered with and
is bound to another NFT (which is called here a `soul`).

Therefor, in this contract, the owner of an SBT is an NFT rather than a address/contract, which
is specified using the contract address and the token_id of a certain NFT. The reason is that if a
`soul` moves from one address to another, all its SBTs are still attached to it, while it is not the case
if the owner of an SBT was an address.






