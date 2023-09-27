# Constellation Protocol: An Indexing and Asset Management Platform
Proof of Intent for Stellar Community Fund award.

### Abstract
The [Stellar Asset Contract](https://soroban.stellar.org/docs/advanced-tutorials/stellar-asset-contract) makes it possible to wrap any native Stellar asset into an ERC20-like token. By tokenizing these assets, we can use them as collateral in Soroban smart contracts. A Constellation Token holds a basket of Soroban tokens as collateral, and exposes an arbitrage mechanism through issuance and redemption. 

### Motivation
[Index funds](https://en.wikipedia.org/wiki/Index_fund) play a large role in traditional finance, with over $10 trillion in assets benchmarked to the S&P 500 alone. Exchange Traded Funds (ETFs) can be conveniently bought or sold through a brokerage like Robinhood and provide  diversification at a low cost.

Constellation Protocol aims to bring the ease of bundling together multiple [Stellar assets](https://developers.stellar.org/docs/fundamentals-and-concepts/stellar-data-structures/assets) into a single Constellation Token, which behaves like an on-chain ETF. 

### Project Goals
1. Deploy the Constellation Protocol v1 smart contracts to Stellar Testnet (after Protocol 20 upgrade).
2. Deploy a proof of concept index using Constellation Protocol.

<img src="https://docs.google.com/drawings/d/e/2PACX-1vTSjf1oe_7FxRLUpM6dA_30YaaJHuYXg--01U3lNM0-AzGJizoYVG2maK6avNFqUJLhJIpFZ89hQRkV/pub?w=1181&amp;h=563">

---

## Protocol Requirements (v1)

### Constellation Token Creation
Users must be able to to permissionlessly configure & deploy Constellation Tokens to the Stellar blockchain by interacting with a factory contract. 

All Constellation Tokens must be compatible with the [Soroban Token Interface](https://soroban.stellar.org/docs/reference/interfaces/token-interface).

### Issuance and Redemption
ETFs have an [arbitrage mechanism](https://en.wikipedia.org/wiki/Exchange-traded_fund#Arbitrage_mechanism) whereby market makers can create and redeem ETF shares by exchanging them for the underlying investments. 

Users must be able to interact with the `ConstellationToken` contract to mint new Constellation Tokens in exchange for precise amounts of each component token.

Constellation token holders must always be able to redeem their tokens in exchange for their share of the underlying components. Freezing issuance and redemption can be considered in a future release.


### Internal accounting

The Constellation Token contract must always be fully colleteralized. Each Constellation Token stores a `ComponentPositions` array which tracks a precise amount `units` for each Soroban Token it holds. Initially these amounts will be fixed at deployment time. In a future version of the protocol, they will be changeable to enable rebalances.


---

## Contract Specifications

### `ConstellationTokenCreator`

#### Inheritance
- None

#### Authentication
- None

#### Key functions
```rust!
fn createToken(env: Env, components: Address[], units: i128[], manager: Address, name: String, symbol: &str);
```

```rust!
fn getConstellationTokens(env: Env) -> Vec<Address>;
```

### `ConstellationToken`
#### Inheritance
- [Token](https://github.com/stellar/rs-soroban-sdk/blob/v20.0.0-rc2/soroban-sdk/src/token.rs)
    - *Inherited functions: name, symbol, mint, burn, transfer, etc.*

#### Authentication
- None 

#### Key functions
```rust!
fn issue(env: Env, constellationToken: Address, quantity: i128, to: Address);
```
Issuance involves sending the ConstellationToken contract the underlying tokenized assets and minting new units of the index. The issuing account must have approved all the component tokens for spending by the contract, or the transaction will revert. The issuing account must also hold enough of each component token to fully collateralize the `quantity` of index, or the transaction will revert. 
```rust!
fn redeem(env: Env, constellationToken: Address, quantity: i128, to: Address);
```
Redemption works in much the same way as issuance, except in reverse.
```rust!
fn getComponents(env: Env) -> Vec<Address>;
```

```rust!
fn getComponentPositions(env: Env) -> Vec<ComponentPosition>;
```
These view functions can be used to determine the amount of underlying tokens needed to perform an issuance transaction.

#### Key State Variables
```rust!
struct ComponentPosition {
    address: Address,
    units: i128
}
```

---
## Future Work
### Rebalancing by authorized Token Manager

The creator of the Constellation Token has a privileged role called the Token Manager.

Since Token Managers are expected to play a similar role as fund managers in traditional finance, they should have a way to change the Constellation Token's underlying investments, either by adding/removing component tokens, or by adjusting relative weightings. 

The ability to rebalance depends on having available liquidity to trade against. Since Soroban is so new it's unlikely that the component tokens will be available to buy/sell *in their token form*. There may already be AMM or order book liquidity for the native assets *outside* of the Stellar Asset Contract wrapper, but that liquidity can't be accessed from within the Soroban framework.

### Front-end UI for issuing & redeeming Constellation Tokens
Many users are not sophisticated enough to interact with the Constellation Token smart contracts directly. A web-based user interface would make it easy for users to perform Issuance and Redemption transactions using Freighter (or another Stellar wallet).
