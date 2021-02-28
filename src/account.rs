use tokio::sync::watch;
use crate::ib_contract;
use rust_decimal::prelude::*;

type Updating<T> = watch::Receiver<Option<T>>;
type Sender<T> = watch::Sender<Option<T>>;
#[derive(Debug)]
pub struct Position {
    pub contract: ib_contract::Contract,
    pub position: Option<Decimal>,
    pub market_price: Option<Decimal>,
    pub market_value: Option<Decimal>,
    pub average_cost: Option<Decimal>,
    pub unrealized_pnl: Option<Decimal>,
    pub realized_pnl: Option<Decimal>
}

pub struct AccountReceiver {
    pub update_time: Updating<String>,
    pub account_code: Updating<String>,
    pub account_type: Updating<String>,
    pub cash_balance: Updating<Decimal>,
    pub equity_with_loan_value: Updating<Decimal>,
    pub excess_liquidity: Updating<Decimal>,
    pub net_liquidation: Updating<Decimal>,
    pub realized_pnl: Updating<Decimal>,
    pub unrealized_pnl: Updating<Decimal>,
    pub total_cash_balance: Updating<Decimal>,
    pub portfolio: Updating<Vec<Position>>
}

pub struct AccountSender {
    pub update_time: Sender<String>,
    pub account_code: Sender<String>,
    pub account_type: Sender<String>,
    pub cash_balance: Sender<Decimal>,
    pub equity_with_loan_value: Sender<Decimal>,
    pub excess_liquidity: Sender<Decimal>,
    pub net_liquidation: Sender<Decimal>,
    pub realized_pnl: Sender<Decimal>,
    pub unrealized_pnl: Sender<Decimal>,
    pub total_cash_balance: Sender<Decimal>,
    pub portfolio: Sender<Vec<Position>>
}

pub fn init_account_channel() -> (AccountSender, AccountReceiver) {
    let (update_time_t, update_time_r) = watch::channel(None);
    let (account_code_t, account_code_r) = watch::channel(None);
    let (account_type_t, account_type_r) = watch::channel(None);
    let (cash_balance_t, cash_balance_r) = watch::channel(None);
    let (equity_with_loan_value_t, equity_with_loan_value_r) = watch::channel(None);
    let (excess_liquidity_t, excess_liquidity_r) = watch::channel(None);
    let (net_liquidation_t, net_liquidation_r) = watch::channel(None);
    let (realized_pnl_t, realized_pnl_r) = watch::channel(None);
    let (unrealized_pnl_t, unrealized_pnl_r) = watch::channel(None);
    let (total_cash_balance_t, total_cash_balance_r) = watch::channel(None);
    let (portfolio_t, portfolio_r) = watch::channel(None);
    let sender = AccountSender {
        update_time : update_time_t,
        account_code: account_code_t,
        account_type: account_type_t,
        cash_balance: cash_balance_t,
        equity_with_loan_value: equity_with_loan_value_t,
        excess_liquidity: excess_liquidity_t,
        net_liquidation: net_liquidation_t,
        realized_pnl: realized_pnl_t,
        unrealized_pnl: unrealized_pnl_t,
        total_cash_balance: total_cash_balance_t,
        portfolio: portfolio_t
    };
    let receiver = AccountReceiver {
        update_time : update_time_r,
        account_code: account_code_r,
        account_type: account_type_r,
        cash_balance: cash_balance_r,
        equity_with_loan_value: equity_with_loan_value_r,
        excess_liquidity: excess_liquidity_r,
        net_liquidation: net_liquidation_r,
        realized_pnl: realized_pnl_r,
        unrealized_pnl: unrealized_pnl_r,
        total_cash_balance: total_cash_balance_r,
        portfolio: portfolio_r
    };
    (sender, receiver)


}
    