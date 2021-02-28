use crate::ib_enums;
use rust_decimal::prelude::*;
use crate::utils::ib_message::Encodable;
#[derive(Debug)]
pub struct ComboLeg {
    pub con_id: i32,
    pub ratio: i32,
    pub action: ib_enums::ComboAction,
    pub exchange: String,
    pub open_close: ib_enums::OptionOpenClose,
    pub shortsale_slot: ib_enums::ShortSaleSlot,
    pub designated_location: String,
    pub exempt_code: i32,
}

#[derive(Default,Debug)]
pub struct DeltaNeutralContract {
    pub con_id: i32,
    pub delta: Decimal,
    pub price: Decimal,
}

#[derive(Default,Debug)]
pub struct Contract {
    pub con_id: Option<i32>,
    pub symbol: Option<String>,
    pub sec_type: Option<ib_enums::SecType>,
    pub last_trade_date_or_contract_month: Option<String>,
    pub strike: Option<Decimal>,
    pub right: Option<ib_enums::OptionRight>,
    pub multiplier: Option<String>,
    pub exchange: Option<String>,
    pub currency: Option<String>,
    pub local_symbol: Option<String>,
    pub primary_exchange: Option<String>,
    pub trading_class: Option<String>,
    pub include_expired: Option<bool>,
    pub sec_id_type: Option<ib_enums::SecIdType>,
    pub sec_id: Option<String>,
    pub combo_legs_description: Option<String>,
    pub combo_legs: Option<Vec<ComboLeg>>,
    pub delta_neutral_contract: Option<DeltaNeutralContract>,
}

impl Encodable for Contract {
    fn encode(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.con_id.encode());
        code.push_str(&self.symbol.encode());
        code.push_str(&self.sec_type.encode());
        code.push_str(&self.last_trade_date_or_contract_month.encode());
        code.push_str(&self.strike.encode());
        code.push_str(&self.right.encode());
        code.push_str(&self.multiplier.encode());
        code.push_str(&self.exchange.encode());
        code.push_str(&self.primary_exchange.encode());
        code.push_str(&self.currency.encode());
        code.push_str(&self.local_symbol.encode());
        code.push_str(&self.trading_class.encode());
        code.push_str(&self.include_expired.encode());
        code.push_str(&self.sec_id_type.encode());
        code.push_str(&self.sec_id.encode());
        code
    }
}

impl Contract {
    pub fn encode_for_order(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.con_id.encode());
        code.push_str(&self.symbol.encode());
        code.push_str(&self.sec_type.encode());
        code.push_str(&self.last_trade_date_or_contract_month.encode());
        code.push_str(&self.strike.encode());
        code.push_str(&self.right.encode());
        code.push_str(&self.multiplier.encode());
        code.push_str(&self.exchange.encode());
        code.push_str(&self.primary_exchange.encode());
        code.push_str(&self.currency.encode());
        code.push_str(&self.local_symbol.encode());
        code.push_str(&self.trading_class.encode());
        code.push_str(&self.sec_id_type.encode());
        code.push_str(&self.sec_id.encode());
        code
    }

    pub fn encode_for_ticker(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.con_id.encode());
        code.push_str(&self.symbol.encode());
        code.push_str(&self.sec_type.encode());
        code.push_str(&self.last_trade_date_or_contract_month.encode());
        code.push_str(&self.strike.encode());
        code.push_str(&self.right.encode());
        code.push_str(&self.multiplier.encode());
        code.push_str(&self.exchange.encode());
        code.push_str(&self.primary_exchange.encode());
        code.push_str(&self.currency.encode());
        code.push_str(&self.local_symbol.encode());
        code.push_str(&self.trading_class.encode());
        code
    }
}

#[derive(Default,Debug)]
pub struct ContractDetails {
    pub contract: Option<Contract>,
    pub market_name: Option<String>,
    pub min_tick: Option<Decimal>,
    pub price_magnifier: Option<i32>,
    pub order_types: Option<String>,
    pub valid_exchanges: Option<String>,
    pub under_con_id: Option<i32>,
    pub long_name: Option<String>,
    pub contract_month: Option<String>,
    pub industry: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub timezone_id: Option<String>,
    pub trading_hours: Option<String>,
    pub liquid_hours: Option<String>,
    pub ev_rule: Option<String>,
    pub ev_multiplier: Option<String>,
    pub md_size_multiplier: Option<String>,
    pub agg_group: Option<i32>,
    pub sec_id_list: Option<Vec<(String, String)>>,
    pub under_symbol: Option<String>,
    pub under_sec_type: Option<ib_enums::SecType>,
    pub market_rule_ids: Option<String>,
    pub real_expiration_date: Option<String>,
    pub last_trade_time: Option<String>,
    pub stock_type: Option<String>,
    pub cusip: Option<String>,
    pub ratings: Option<String>,
    pub desc_append: Option<String>,
    pub bond_type: Option<String>,
    pub coupon_type: Option<String>,
    pub callable: Option<bool>,
    pub putable: Option<bool>,
    pub coupon: Option<bool>,
    pub convertible: Option<bool>,
    pub maturity: Option<bool>,
    pub issue_date: Option<bool>,
    pub next_option_date: Option<bool>,
    pub next_option_type: Option<bool>,
    pub notes: Option<String>,
}

pub struct ContractDescription {
    contract: Option<Contract>,
    derivative_sec_types_list: Option<Vec<String>>,
}

pub type ContractDescriptionList = Vec<ContractDescription>;

