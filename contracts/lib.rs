#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct RwaGuard;

#[contractimpl]
impl RwaGuard {
    // Check if a wallet is on the approved "Whitelist" for an asset
    pub fn is_compliant(env: Env, user: Address) -> bool {
        let key = symbol_short!("WL_DONE");
        env.storage().persistent().has(&user)
    }

    // Admin function to approve a wallet (simulating KYC completion)
    pub fn approve_user(env: Env, admin: Address, user: Address) {
        admin.require_auth();
        env.storage().persistent().set(&user, &true);
    }
}
