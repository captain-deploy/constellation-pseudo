// This contract will resemble the "Deployer" Soroban example.
// https://github.com/stellar/soroban-examples/blob/v20.0.0-rc2/deployer/deployer/src/lib.rs
#![no_std]

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol, Val, Vec};

#[contract]
pub struct ConstellationTokenCreator;

#[contractimpl]
impl ConstellationTokenCreator {
    /// Deploy the ConstellationToken Wasm and after deployment invoke the initialize function
    /// Returns the contract ID and result of the initialize function.
    pub fn deploy(
        env: Env,
        decimal: u32,
        components: Vec<Address>,
        amounts: Vec<u32>,
        admin: Address, // ConstellationMinterBurner contract ID
        manager: Address, // For future use; manager can rebalance and charge fees
        name: String,
        symbol: Symbol
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
    ) -> (Address) {

        // Deploy the contract using the uploaded Wasm with given hash.
        let deployed_address = env
            .deployer()
            .with_current_contract(salt) // Only this contract can deploy Constellation tokens
            .deploy(wasm_hash);

        // Invoke the initialize function with the given arguments.
        env.invoke_contract(&deployed_address, "initialize", (components, units, admin, manager, minter_burner, name, symbol));
        // Return the contract ID of the deployed ConstellationToken contract
        deployed_address
    }
}
