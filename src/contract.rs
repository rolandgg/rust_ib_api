use crate::enums;
use rust_decimal::prelude::*;
use crate::enums::*;
use crate::utils::ib_message::Encodable;
use chrono::{DateTime,NaiveDateTime,Utc,TimeZone};
use chrono_tz::Tz;
use chrono_tz::{UTC,US};
#[derive(Debug,Clone)]
pub struct ComboLeg {
    pub(crate) con_id: Option<i32>,
    pub(crate) ratio: Option<i32>,
    pub(crate) action: Option<enums::ComboAction>,
    pub(crate) exchange: Option<String>,
    pub(crate) open_close: Option<enums::OptionOpenClose>,
    pub(crate) shortsale_slot: Option<enums::ShortSaleSlot>,
    pub(crate) designated_location: Option<String>,
    pub(crate) exempt_code: Option<i32>,
}

impl ComboLeg {
    pub fn new(con_id: i32, ratio: i32, action: enums::ComboAction, exchange: &str) -> ComboLeg {
        ComboLeg{
            con_id: Some(con_id), ratio: Some(ratio), action: Some(action), exchange: Some(exchange.to_string()), open_close: None, shortsale_slot: None, designated_location: None, exempt_code: None
        }
    }
}
#[derive(Default,Debug,Clone)]
pub(crate) struct DeltaNeutralContract {
    pub con_id: Option<i32>,
    pub delta: Option<Decimal>,
    pub price: Option<Decimal>,
}

#[derive(Default,Debug,Clone)]
pub struct Contract {
    pub(crate) con_id: Option<i32>,
    pub(crate) symbol: Option<String>,
    pub(crate) sec_type: Option<enums::SecType>,
    pub(crate) last_trade_date_or_contract_month: Option<String>,
    pub(crate) strike: Option<Decimal>,
    pub(crate) right: Option<enums::OptionRight>,
    pub(crate) multiplier: Option<String>,
    pub(crate) exchange: Option<String>,
    pub(crate) currency: Option<String>,
    pub(crate) local_symbol: Option<String>,
    pub(crate) primary_exchange: Option<String>,
    pub(crate) trading_class: Option<String>,
    pub(crate) include_expired: Option<bool>,
    pub(crate) sec_id_type: Option<enums::SecIdType>,
    pub(crate) sec_id: Option<String>,
    pub(crate) issuer_id: Option<String>,
    pub(crate) combo_legs_description: Option<String>,
    pub(crate) combo_legs: Option<Vec<ComboLeg>>,
    pub(crate) delta_neutral_contract: Option<DeltaNeutralContract>,
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
        code.push_str(&self.issuer_id.encode());
        code
    }
}

impl Contract {
    pub(crate) fn encode_for_order(&self) -> String {
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

    pub(crate) fn encode_for_ticker(&self) -> String {
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
        match &self.sec_type {
            Some(SecType::Combo) => {
                match &self.combo_legs {
                    Some(legs) => {
                        code.push_str(&legs.len().encode());
                        for leg in legs {
                            code.push_str(&leg.con_id.encode());
                            code.push_str(&leg.ratio.encode());
                            code.push_str(&leg.action.encode());
                            code.push_str(&leg.exchange.encode());
                        };
                    },
                    None => code.push_str("0\0"),
                }
            },
            _ => ()
        }
        if let Some(delta_n_contract) = &self.delta_neutral_contract {
            code.push_str(&true.encode());
            code.push_str(&delta_n_contract.con_id.encode());
            code.push_str(&delta_n_contract.delta.encode());
            code.push_str(&delta_n_contract.price.encode());
        }
        else {
            code.push_str(&false.encode());
        } 
        code
    }

    pub(crate) fn encode_for_hist_data(&self) -> String {
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
    /// Creates a stock contract.
    pub fn stock(symbol: &str, exchange: &str, currency: &str) -> Self {
        Contract {
            symbol: Some(symbol.to_string()),
            exchange: Some(exchange.to_string()),
            sec_type: Some(SecType::Stock),
            currency: Some(currency.to_string()),
            ..Default::default()
        }
    }
    /// Creates a stock contract for US stocks with SMART routing.
    pub fn stock_us_smart(symbol: &str) -> Self {
        Self::stock(symbol, "SMART", "USD")
    }
    pub fn combo(symbol: &str, exchange: &str, currency: &str) -> Self {
        Contract {
            symbol: Some(symbol.to_string()),
            exchange: Some(exchange.to_string()),
            sec_type: Some(SecType::Combo),
            currency: Some(currency.to_string()),
            ..Default::default()
        }
    }
    pub fn add_leg(&mut self, leg: ComboLeg) {
        if self.sec_type == Some(SecType::Combo) {
            match &mut self.combo_legs {
                Some(legs) => legs.push(leg),
                None => ()
            };
        }
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
    pub fn symbol(&self) -> &Option<String> {
        &self.symbol
    }
    pub fn con_id(&self) -> Option<i32> {
        self.con_id
    }
}

#[derive(Default,Debug)]
pub struct ContractDetails {
    pub(crate) contract: Option<Contract>,
    pub(crate) market_name: Option<String>,
    pub(crate) min_tick: Option<Decimal>,
    pub(crate) price_magnifier: Option<i32>,
    pub(crate) order_types: Option<String>,
    pub(crate) valid_exchanges: Option<String>,
    pub(crate) under_con_id: Option<i32>,
    pub(crate) long_name: Option<String>,
    pub(crate) contract_month: Option<String>,
    pub(crate) industry: Option<String>,
    pub(crate) category: Option<String>,
    pub(crate) subcategory: Option<String>,
    pub(crate) timezone_id: Option<String>,
    pub(crate) trading_hours: Option<String>,
    pub(crate) liquid_hours: Option<String>,
    pub(crate) ev_rule: Option<String>,
    pub(crate) ev_multiplier: Option<String>,
    pub(crate) md_size_multiplier: Option<String>,
    pub(crate) agg_group: Option<i32>,
    pub(crate) sec_id_list: Option<Vec<(String, String)>>,
    pub(crate) under_symbol: Option<String>,
    pub(crate) under_sec_type: Option<enums::SecType>,
    pub(crate) market_rule_ids: Option<String>,
    pub(crate) real_expiration_date: Option<String>,
    pub(crate) last_trade_time: Option<String>,
    pub(crate) stock_type: Option<String>,
    pub(crate) cusip: Option<String>,
    pub(crate) ratings: Option<String>,
    pub(crate) desc_append: Option<String>,
    pub(crate) bond_type: Option<String>,
    pub(crate) coupon_type: Option<String>,
    pub(crate) callable: Option<bool>,
    pub(crate) putable: Option<bool>,
    pub(crate) coupon: Option<bool>,
    pub(crate) convertible: Option<bool>,
    pub(crate) maturity: Option<bool>,
    pub(crate) issue_date: Option<bool>,
    pub(crate) next_option_date: Option<bool>,
    pub(crate) next_option_type: Option<bool>,
    pub(crate) notes: Option<String>,
    pub(crate) min_size: Option<String>,
    pub(crate) size_increment: Option<String>,
    pub(crate) suggested_size_increment: Option<String>
}
///Returns the liquid hours (i.e. trading calendar) of the contract.
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
    /// Returns the underlying contract.
    pub fn contract(&self) -> &Option<Contract> {
        &self.contract
    }
}

pub(crate) struct ContractDescription {
    contract: Option<Contract>,
    derivative_sec_types_list: Option<Vec<String>>,
}

pub(crate) type ContractDescriptionList = Vec<ContractDescription>;

