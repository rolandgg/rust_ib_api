use std::collections::HashSet;
use rust_decimal::Decimal;

pub struct OptParams {
    pub underlying_con_id: Option<String>,
    pub exchange: Option<String>,
    pub trading_class: Option<String>,
    pub multiplier: Option<String>,
    pub strikes: HashSet<Decimal>,
    pub expirations: HashSet<String>
}