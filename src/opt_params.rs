use rust_decimal::Decimal;

pub struct OptParams {
    strikes: Vec<Decimal>,
    expirations: Vec<String>,
}