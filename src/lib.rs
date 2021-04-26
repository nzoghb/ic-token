use std::collections::BTreeMap;

use ic_cdk::export::{candid::CandidType, Principal};
use serde::Serialize;

#[derive(Clone, Debug, Default, CandidType, Serialize)]
pub struct Token {
    pub name: String,
    pub decimals: u8,
    pub symbol: String,
    pub total_supply: u64,
    pub balances: BTreeMap<Principal, u64>,
    pub allowances: BTreeMap<(Principal, Principal), u64>,
}

impl Token {
    pub fn new(name: String, decimals: u8, symbol: String, total_supply: u64) -> Self {
        Token {
            name,
            decimals,
            symbol,
            total_supply,
            balances: BTreeMap::new(),
            allowances: BTreeMap::new(),
        }
    }

    pub fn balance_of(&self, account: &Principal) -> u64 {
        match self.balances.get(account) {
            Some(balance) => balance.clone(),
            None => 0,
        }
    }

    pub fn allowance(&self, owner: &Principal, spender: &Principal) -> u64 {
        match self.allowances.get(&(owner.clone(), spender.clone())) {
            Some(allowance) => allowance.clone(),
            None => 0,
        }
    }
}
