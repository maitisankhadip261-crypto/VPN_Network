#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

#[contract]
pub struct VpnNetwork;

#[contractimpl]
impl VpnNetwork {

    // Register a VPN node
    pub fn register_node(env: Env, node: Address) {
        let key = Symbol::new(&env, "nodes");
        let mut nodes: Map<Address, bool> = env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        nodes.set(node.clone(), true);
        env.storage().instance().set(&key, &nodes);
    }

    // Pay for VPN usage
    pub fn pay_for_vpn(env: Env, user: Address, amount: i128) {
        let key = Symbol::new(&env, "balances");
        let mut balances: Map<Address, i128> = env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let current = balances.get(user.clone()).unwrap_or(0);
        balances.set(user, current + amount);

        env.storage().instance().set(&key, &balances);
    }

    // Withdraw earnings (for nodes)
    pub fn withdraw(env: Env, node: Address) -> i128 {
        let key = Symbol::new(&env, "balances");
        let mut balances: Map<Address, i128> = env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let amount = balances.get(node.clone()).unwrap_or(0);
        balances.set(node, 0);

        env.storage().instance().set(&key, &balances);

        amount
    }

    // Check balance
    pub fn get_balance(env: Env, addr: Address) -> i128 {
        let key = Symbol::new(&env, "balances");
        let balances: Map<Address, i128> = env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        balances.get(addr).unwrap_or(0)
    }
}