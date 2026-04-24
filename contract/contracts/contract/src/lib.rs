#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Env, Symbol, String, Map,
};

#[contract]
pub struct PrenupContract;

#[contractimpl]
impl PrenupContract {
    /// Create a new prenup record
    /// This will fail if the ID already exists (immutability enforced)
    pub fn create_prenup(
        env: Env,
        id: Symbol,
        party_a: String,
        party_b: String,
        terms: String,
    ) {
        // Prevent overwriting existing records
        if env.storage().instance().has(&id) {
            panic!("Prenup with this ID already exists");
        }

        let mut record = Map::new(&env);
        record.set(symbol_short!("party_a"), party_a);
        record.set(symbol_short!("party_b"), party_b);
        record.set(symbol_short!("terms"), terms);

        env.storage().instance().set(&id, &record);
    }

    /// Retrieve a prenup record by ID
    pub fn get_prenup(env: Env, id: Symbol) -> Map<Symbol, String> {
        env.storage()
            .instance()
            .get(&id)
            .unwrap_or_else(|| panic!("Prenup not found"))
    }
}
   