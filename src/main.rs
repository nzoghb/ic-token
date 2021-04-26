use ic_token::Token;

use ic_cdk::export::Principal;
use ic_cdk::storage;
use ic_cdk_macros::*;

fn main() {}

static mut INITIALIZED: bool = false;

#[update]
fn init(name: String, decimals: u8, symbol: String, total_supply: u64) -> () {
    unsafe {
        assert!(!INITIALIZED);
    }
    let token = token_mut();
    *token = Token::new(name, decimals, symbol, total_supply);
}

#[query(name = "name")]
fn name() -> String {
    let token = token();
    token.name.clone()
}

#[query(name = "symbol")]
fn symbol() -> String {
    let token = token();
    token.symbol.clone()
}

#[query(name = "decimals")]
fn decimals() -> u8 {
    let token = token();
    token.decimals.clone()
}

#[query(name = "totalSupply")]
fn total_supply() -> u64 {
    let token = token();
    token.total_supply.clone()
}

#[query(name = "balanceOf")]
fn balance_of(account: Principal) -> u64 {
    balance_of_(&account)
}

#[update(name = "transfer")]
fn transfer(recipient: Principal, amount: u64) -> Result<(), ()> {
    let sender = ic_cdk::caller();
    transfer_(sender, recipient, amount);
    Ok(())
}

#[query(name = "allowance")]
fn allowance(owner: Principal, spender: Principal) -> u64 {
    let token = token();
    token.allowance(&owner, &spender)
}

#[update(name = "approve")]
fn approve(spender: Principal, amount: u64) -> Result<(), ()> {
    let sender = ic_cdk::caller();
    approve_(sender, spender, amount);
    Ok(())
}

#[update(name = "transferFrom")]
fn transfer_from(owner: Principal, recipient: Principal, amount: u64) -> Result<(), ()> {
    let sender = ic_cdk::caller();
    let token = token_mut();
    let current_allowance = token.allowance(&owner, &sender);
    assert!(current_allowance >= amount);
    transfer_(owner, recipient, amount);
    Ok(())
}

fn token() -> &'static Token {
    storage::get::<Token>()
}

fn token_mut() -> &'static mut Token {
    storage::get_mut::<Token>()
}

fn balance_of_(account: &Principal) -> u64 {
    token().balance_of(account)
}

fn transfer_(sender: Principal, recipient: Principal, amount: u64) {
    let balance = balance_of_(&sender);
    assert!(balance >= amount);
    let token = token_mut();
    token.balances.insert(sender, balance - amount);
    token.balances.insert(recipient, balance + amount);
}

fn approve_(owner: Principal, spender: Principal, amount: u64) {
    let token = token_mut();
    token.allowances.insert((owner, spender), amount);
}
