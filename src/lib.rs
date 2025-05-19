#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, unwrap::UnwrapOptimized, Address, Env, Vec,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer,
    TradeHistory,
    LastPrice,
}

#[derive(Clone)]
#[contracttype]
pub struct Offer {
    pub seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_price: u32,
    pub buy_price: u32,
    pub min_buy_amount: i128,
    pub total_bought: i128,
    pub total_sold: i128,
    pub is_active: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct TradeRecord {
    pub buyer: Address,
    pub buy_amount: i128,
    pub sell_amount: i128,
    pub timestamp: u64,
}

#[contract]
pub struct SingleOfferWork;

#[contractimpl]
impl SingleOfferWork {
    pub fn create(
        e: Env,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
        min_buy_amount: i128,
    ) {
        if e.storage().instance().has(&DataKey::Offer) {
            panic!("offer already exists");
        }
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price not allowed");
        }
        if min_buy_amount <= 0 {
            panic!("min_buy_amount must be positive");
        }
        seller.require_auth();
        let offer = Offer {
            seller,
            sell_token,
            buy_token,
            sell_price,
            buy_price,
            min_buy_amount,
            total_bought: 0,
            total_sold: 0,
            is_active: true,
        };
        e.storage().instance().set(&DataKey::Offer, &offer);
    }

    pub fn trade(e: Env, buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128) {
        buyer.require_auth();
        let mut offer = load_offer(&e);

        if !offer.is_active {
            panic!("offer not active");
        }
        if buy_token_amount < offer.min_buy_amount {
            panic!("buy_token_amount too low");
        }

        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let buy_token_client = token::Client::new(&e, &offer.buy_token);

        let sell_token_amount = buy_token_amount
            .checked_mul(offer.sell_price as i128)
            .unwrap_optimized()
            / offer.buy_price as i128;

        if sell_token_amount < min_sell_token_amount {
            panic!("sell_token_amount less than minimum required");
        }

        let contract = e.current_contract_address();
        let contract_balance = sell_token_client.balance(&contract);
        if contract_balance < sell_token_amount {
            panic!("insufficient contract sell_token balance");
        }

        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        buy_token_client.transfer(&contract, &offer.seller, &buy_token_amount);

        offer.total_bought += buy_token_amount;
        offer.total_sold += sell_token_amount;
        e.storage().instance().set(&DataKey::Offer, &offer);

        e.storage().instance().set(&DataKey::LastPrice, &(buy_token_amount, sell_token_amount));
        let trade_record = TradeRecord {
            buyer,
            buy_amount: buy_token_amount,
            sell_amount: sell_token_amount,
            timestamp: e.ledger().timestamp(),
        };
        let mut history: Vec<TradeRecord> = e
            .storage()
            .instance()
            .get(&DataKey::TradeHistory)
            .unwrap_or(Vec::new(&e));
        history.push_back(trade_record);
        if history.len() > 10 {
            history.remove(0);
        }
        e.storage().instance().set(&DataKey::TradeHistory, &history);
    }

    pub fn withdraw(e: Env, token: Address, amount: i128) {
        let offer = load_offer(&e);
        offer.seller.require_auth();
        token::Client::new(&e, &token).transfer(
            &e.current_contract_address(),
            &offer.seller,
            &amount,
        );
    }

    pub fn update_price(e: Env, sell_price: u32, buy_price: u32) {
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price not allowed");
        }
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        offer.sell_price = sell_price;
        offer.buy_price = buy_price;
        e.storage().instance().set(&DataKey::Offer, &offer);
    }

    pub fn update_min_buy_amount(e: Env, min_buy_amount: i128) {
        if min_buy_amount <= 0 {
            panic!("min_buy_amount must be positive");
        }
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        offer.min_buy_amount = min_buy_amount;
        e.storage().instance().set(&DataKey::Offer, &offer);
    }

    pub fn set_active(e: Env, active: bool) {
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        offer.is_active = active;
        e.storage().instance().set(&DataKey::Offer, &offer);
    }

    pub fn get_offer(e: Env) -> Offer {
        load_offer(&e)
    }

    pub fn get_trade_history(e: Env) -> Vec<TradeRecord> {
        e.storage()
            .instance()
            .get(&DataKey::TradeHistory)
            .unwrap_or(Vec::new(&e))
    }

    pub fn get_last_price(e: Env) -> (i128, i128) {
        e.storage()
            .instance()
            .get(&DataKey::LastPrice)
            .unwrap_or((0, 0))
    }
}

fn load_offer(e: &Env) -> Offer {
    e.storage().instance().get(&DataKey::Offer).unwrap()
}