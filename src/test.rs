#![cfg(test)]
extern crate std;

use crate::{token, SingleOfferWorkClient};
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, symbol_short,
};

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn create_single_offer_work<'a>(
    e: &Env,
    seller: &Address,
    sell_token: &Address,
    buy_token: &Address,
    sell_price: u32,
    buy_price: u32,
    min_buy_amount: i128,
) -> SingleOfferWorkClient<'a> {
    let contract = SingleOfferWorkClient::new(e, &e.register(crate::SingleOfferWork, ()));
    contract.create(
        seller,
        sell_token,
        buy_token,
        &sell_price,
        &buy_price,
        &min_buy_amount,
    );
    contract
}

#[test]
fn test_happy_path() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let seller = Address::generate(&e);
    let buyer = Address::generate(&e);

    let (sell_token, sell_admin) = create_token_contract(&e, &admin);
    let (buy_token, buy_admin) = create_token_contract(&e, &admin);

    sell_admin.mint(&seller, &500);
    buy_admin.mint(&buyer, &2000);

    let offer = create_single_offer_work(
        &e,
        &seller,
        &sell_token.address,
        &buy_token.address,
        1, // 1 sell_token
        2, // 2 buy_token
        10,
    );

    sell_token.transfer(&seller, &offer.address, &100);

    assert!(offer.try_trade(&buyer, &5, &1).is_err());
    offer.trade(&buyer, &20, &10);

    assert_eq!(sell_token.balance(&seller), 400);
    assert_eq!(sell_token.balance(&buyer), 10);
    assert_eq!(sell_token.balance(&offer.address), 90);
    assert_eq!(buy_token.balance(&seller), 20);
    assert_eq!(buy_token.balance(&buyer), 1980);
    assert_eq!(buy_token.balance(&offer.address), 0);

    offer.withdraw(&sell_token.address, &40);

    assert_eq!(sell_token.balance(&seller), 440);
    assert_eq!(sell_token.balance(&offer.address), 50);

    offer.update_price(&2, &2);
    offer.trade(&buyer, &20, &9);

    let (last_buy, last_sell) = offer.get_last_price();
    assert_eq!(last_buy, 20);
    assert_eq!(last_sell, 20);

    let history = offer.get_trade_history();
    assert_eq!(history.len(), 2);
    assert_eq!(history.get(0).unwrap().buy_amount, 20);
    assert_eq!(history.get(1).unwrap().buy_amount, 20);
}

#[test]
fn test_offer_activation() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let seller = Address::generate(&e);
    let buyer = Address::generate(&e);

    let (sell_token, sell_admin) = create_token_contract(&e, &admin);
    let (buy_token, buy_admin) = create_token_contract(&e, &admin);

    sell_admin.mint(&seller, &100);
    buy_admin.mint(&buyer, &100);

    let offer = create_single_offer_work(
        &e,
        &seller,
        &sell_token.address,
        &buy_token.address,
        1,
        1,
        1,
    );
    sell_token.transfer(&seller, &offer.address, &100);

    offer.set_active(&false);
    assert!(offer.try_trade(&buyer, &2, &2).is_err());
    offer.set_active(&true);
    offer.trade(&buyer, &10, &10);

    let fetched = offer.get_offer();
    assert!(fetched.is_active);
    assert_eq!(fetched.total_bought, 10);
    assert_eq!(fetched.total_sold, 10);
}