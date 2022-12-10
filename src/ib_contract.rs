use crate::ib_enums;
use rust_decimal::prelude::*;
use crate::ib_enums::*;
use crate::utils::ib_message::Encodable;
use chrono::{DateTime,NaiveDateTime,Utc,TimeZone};
use chrono_tz::Tz;
use chrono_tz::{UTC,US};
#[derive(Debug,Clone)]
pub struct ComboLeg {
    pub con_id: Option<i32>,
    pub ratio: Option<i32>,
    pub action: Option<ib_enums::ComboAction>,
    pub exchange: Option<String>,
    pub open_close: Option<ib_enums::OptionOpenClose>,
    pub shortsale_slot: Option<ib_enums::ShortSaleSlot>,
    pub designated_location: Option<String>,
    pub exempt_code: Option<i32>,
}

impl ComboLeg {
    pub fn new(con_id: i32, ratio: i32, action: ib_enums::ComboAction, exchange: &str) -> ComboLeg {
        ComboLeg{
            con_id: Some(con_id), ratio: Some(ratio), action: Some(action), exchange: Some(exchange.to_string()), open_close: None, shortsale_slot: None, designated_location: None, exempt_code: None
        }
    }
}
#[derive(Default,Debug,Clone)]
pub struct DeltaNeutralContract {
    pub con_id: Option<i32>,
    pub delta: Option<Decimal>,
    pub price: Option<Decimal>,
}

#[derive(Default,Debug,Clone)]
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

    pub fn encode_for_hist_data(&self) -> String {
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
        code
    }

    pub fn stock_spread_smart_usd(contract_1: &Contract, ratio_1: i32, contract_2: &Contract, ratio_2: i32) -> Option<Contract> {
        let mut ret = None;
        if let Some(con_id_1) = contract_1.con_id {
            if let Some(con_id_2) = contract_2.con_id {
                if let Some(symbol_1) = &contract_1.symbol { 
                    if let Some(symbol_2) = &contract_2.symbol {
                        let mut legs = Vec::new();
                        legs.push(ComboLeg::new(con_id_1, ratio_1, ComboAction::Buy, "SMART")); //IBKR
                        legs.push(ComboLeg::new(con_id_2, ratio_2, ComboAction::Sell, "SMART")); //MCD
                        ret = Some(Contract {
                            symbol: Some(symbol_1.clone() + "," + &symbol_2),
                            exchange: Some("SMART".to_string()),
                            sec_type: Some(SecType::Combo),
                            currency: Some("USD".to_string()),
                            combo_legs: Some(legs),
                            ..Default::default()
                        });
                    }
                }
            }
        }
        ret
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

impl ContractDetails {
    pub fn liquid_hours(&self) -> Option<Vec<(DateTime<Tz>, DateTime<Tz>)>> {

        let mut liq_hours_it = self.liquid_hours.as_ref()?.split(";");
        let mut ret = Vec::new();
        while let Some(liq_hours) = liq_hours_it.next() {
            if liq_hours.contains("CLOSED") {continue}
            else {
                let mut hours_it = liq_hours.split("-");
                let open_str = hours_it.next()?;
                let close_str = hours_it.next()?;
                let open_dt = NaiveDateTime::parse_from_str(open_str, "%Y%m%d:%H%M").unwrap();
                let close_dt = NaiveDateTime::parse_from_str(close_str, "%Y%m%d:%H%M").unwrap();
                if let Some(tz) = &self.timezone_id {
                    if tz.contains("US/Eastern") {
                        ret.push((US::Eastern.from_local_datetime(&open_dt).unwrap(), US::Eastern.from_local_datetime(&close_dt).unwrap()));
                    }
                    else {
                        ret.push((UTC.from_local_datetime(&open_dt).unwrap(), UTC.from_local_datetime(&close_dt).unwrap()));
                    }
                } 
            }
        }
       Some(ret)
    }
}

pub struct ContractDescription {
    contract: Option<Contract>,
    derivative_sec_types_list: Option<Vec<String>>,
}

pub type ContractDescriptionList = Vec<ContractDescription>;

