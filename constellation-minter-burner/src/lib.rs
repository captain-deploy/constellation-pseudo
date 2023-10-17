// This contract enables issuance and redemption of Constellation tokens.
// It will resemble the "Atomic Swap" Soroban example.
// https://github.com/stellar/soroban-examples/tree/main/atomic_swap

#![no_std]

use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};
use crate::{contract::ConstellationToken, ConstellationTokenClient};

mod constellationToken {
    soroban_sdk::contractimport!(
        file = "../..constellation-token/target/wasm32-unknown-unknown/release/constellation-token-contract.wasm"
    );
}


#[contract]
pub struct ConstellationMinterBurner;

#[contractimpl]
impl ConstellationMinterBurner {
    // Swap component tokens for newly minted Constellation tokens
    pub fn mint(
        env: Env,
        from: Address,
        to: Address,
        ctoken: Address,
        ctoken_amount: i128,
    ) {
        // Verify 'from' has enough of each component token for ctoken_amount
        // Verify 'from' has approved allowances for each component token 
        from.require_auth();
        // Transfer component tokens from 'from' to the ConstellationToken contract
        // Mint ctoken_amount of Constellation tokens to caller
        ctoken.mint(env, to, ctoken_amount);
    }

    // Swap component tokens for newly minted Constellation tokens
    pub fn burn(
        env: Env,
        from: Address,
        ctoken: Address,
        ctoken_amount: i128,
    ) {
        // Verify 'from' user has approved ctoken_amount
        // Transfer component tokens from ConstellationToken contract to 'from' address
        // Burn ctoken_amount of Constellation tokens from caller
        let ctoken = constellationToken::Client::new(&env, &ctoken);
        ctoken.burn(env, from, ctoken_amount);
    }
}
