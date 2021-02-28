use rust_decimal::prelude::*;
use chrono::NaiveDateTime;
use crate::account::Position;
use crate::utils::ib_message;
use crate::ib_contract;
use crate::utils::ib_message::decode;
use crate::order;
use crate::bars;
use crate::ib_enums::*;

use enumset::EnumSetType;
use enumset::EnumSet;
use bitvec::prelude::*;

#[derive(EnumSetType, Debug)]
pub enum TickAttribute {
   CanAutoExecute,
   PastLimit,
   PreOpen
}

pub enum IBFrame {
    AccountType(Option<String>),
    AccountCode(Option<String>),
    CashBalance(Option<Decimal>),
    EquityWithLoanValue(Option<Decimal>),
    ExcessLiquidity(Option<Decimal>),
    NetLiquidation(Option<Decimal>),
    RealizedPnL(Option<Decimal>),
    UnrealizedPnL(Option<Decimal>),
    TotalCashBalance(Option<Decimal>),
    AccountUpdateEnd(Option<String>),
    AccountUpdateTime(Option<String>),
    PortfolioValue(Position),
    CurrentTime(NaiveDateTime),
    ContractDetails{
        req_id: usize, contract_details: ib_contract::ContractDetails
    },
    ContractDetailsEnd(usize),
    OrderID(usize),
    OpenOrder{
        order: order::Order, order_state: order::OrderState
    },
    Execution(order::Execution),
    CommissionReport(order::CommissionReport),
    OrderStatus(order::OrderStatus),
    PriceTick{id: usize, kind: TickType, price: f64, size: Option<i32>, attributes: EnumSet<TickAttribute>},
    SizeTick{id: usize, kind: TickType, size: i32},
    StringTick{id: usize, kind: TickType, val: Option<String>},
    GenericTick{id: usize, kind: TickType, val: f64},
    Bars{id: usize, data: bars::BarSeries},
    NotImplemented
}

impl IBFrame {
    pub fn parse (msg: &[u8]) -> Self {
        let mut it = ib_message::iter_ib_message(&msg);
        let msg_type: Incoming = it.next().unwrap().parse().expect("Could not parse message type.");
        match msg_type {
            Incoming::AcctValue => {
                it.next(); //skip version
                match it.next().unwrap() {
                    "AccountCode" => IBFrame::AccountCode(decode(&mut it)),
                    "AccountType" => IBFrame::AccountType(decode(&mut it)),
                    "CashBalance" => IBFrame::CashBalance(decode(&mut it)),
                    "EquityWithLoanValue" => IBFrame::EquityWithLoanValue(decode(&mut it)),
                    "ExcessLiquidity" => IBFrame::ExcessLiquidity(decode(&mut it)),
                    "NetLiquidation" => IBFrame::NetLiquidation(decode(&mut it)),
                    "RealizedPnL" => IBFrame::RealizedPnL(decode(&mut it)),
                    "UnrealizedPnL" => IBFrame::UnrealizedPnL(decode(&mut it)),
                    "TotalCashBalance" => IBFrame::TotalCashBalance(decode(&mut it)),
                    &_ => IBFrame::NotImplemented
                }
            },
            Incoming::AcctDownloadEnd => {
                it.next(); //skip version
                IBFrame::AccountUpdateEnd(decode(&mut it))
            },
            Incoming::AcctUpdateTime => {
                it.next(); //skip version
                IBFrame::AccountUpdateTime(decode(&mut it))
            },
            Incoming::PortfolioValue => {
                let version: i32 = decode(&mut it).unwrap();
                let con_id: i32 = decode(&mut it).unwrap();
                let mut contract = ib_contract::Contract::default();
                contract.con_id = Some(con_id);
                contract.symbol = decode(&mut it);
                contract.sec_type = decode(&mut it);
                contract.last_trade_date_or_contract_month = decode(&mut it);
                contract.strike = decode(&mut it);
                contract.right = decode(&mut it);
                if version >= 7 {
                    contract.multiplier = decode(&mut it);
                    contract.primary_exchange = decode(&mut it);
                }
                contract.currency = decode(&mut it);
                contract.local_symbol = decode(&mut it);
                if version >= 8 {
                    contract.trading_class = decode(&mut it);
                }
                IBFrame::PortfolioValue(Position {
                    contract,
                    position: decode(&mut it),
                    market_price: decode(&mut it),
                    market_value: decode(&mut it),
                    average_cost: decode(&mut it),
                    unrealized_pnl: decode(&mut it),
                    realized_pnl: decode(&mut it)
                })

            },
            Incoming::CurrentTime => {
                it.next(); //skip version
                let unix_time: i64 = decode(&mut it).unwrap();
                IBFrame::CurrentTime(NaiveDateTime::from_timestamp(unix_time, 0))
            },
            Incoming::ContractData => {
                it.next(); //skip version
                let req_id : usize = decode(&mut it).unwrap();
                let mut contract = ib_contract::Contract {
                    symbol : decode(&mut it),
                    sec_type: decode(&mut it),
                    last_trade_date_or_contract_month: decode(&mut it),
                    strike: decode(&mut it),
                    right: decode(&mut it),
                    exchange: decode(&mut it),
                    currency: decode(&mut it),
                    local_symbol: decode(&mut it),
                    ..Default::default()
                };
                let mut details = ib_contract::ContractDetails {
                    market_name: decode(&mut it),
                    ..Default::default()
                };
                contract.trading_class = decode(&mut it);
                contract.con_id = decode(&mut it);
                details.min_tick = decode(&mut it);
                details.md_size_multiplier = decode(&mut it);
                contract.multiplier = decode(&mut it);
                details.order_types = decode(&mut it);
                details.valid_exchanges = decode(&mut it);
                details.price_magnifier = decode(&mut it);
                details.under_con_id = decode(&mut it);
                details.long_name = decode(&mut it);
                contract.primary_exchange = decode(&mut it);
                details.contract_month = decode(&mut it);
                details.industry = decode(&mut it);
                details.category = decode(&mut it);
                details.subcategory = decode(&mut it);
                details.timezone_id = decode(&mut it);
                details.trading_hours = decode(&mut it);
                details.liquid_hours = decode(&mut it);
                details.ev_rule = decode(&mut it);
                details.ev_multiplier = decode(&mut it);
                let sec_id_list_count: Option<usize> = decode(&mut it);
                details.sec_id_list = match sec_id_list_count {
                    Some(count) => {
                        let mut sec_ids: Vec<(String,String)> = Vec::with_capacity(count);
                        for i in 0..count {
                            sec_ids.push((decode(&mut it).unwrap(), decode(&mut it).unwrap()));
                        }
                        Some(sec_ids)
                    },
                    None => None
                };
                details.agg_group = decode(&mut it);
                details.under_symbol = decode(&mut it);
                details.under_sec_type = decode(&mut it);
                details.market_rule_ids = decode(&mut it);
                details.real_expiration_date = decode(&mut it);
                details.contract = Some(contract);
                IBFrame::ContractDetails{
                    req_id,
                    contract_details: details
                }
            }
            Incoming::ContractDataEnd => {
                it.next(); //skip version
                IBFrame::ContractDetailsEnd(decode(&mut it).unwrap())
            }
            Incoming::NextValidId => {
                it.next(); //skip version
                IBFrame::OrderID(decode(&mut it).unwrap())
            }
            Incoming::OpenOrder => {
                let order_id: usize = decode(&mut it).unwrap();
                //decode contract
                let contract = ib_contract::Contract {
                    con_id: decode(&mut it),
                    symbol: decode(&mut it),
                    sec_type: decode(&mut it),
                    last_trade_date_or_contract_month: decode(&mut it),
                    strike: decode(&mut it),
                    right: decode(&mut it),
                    multiplier: decode(&mut it),
                    exchange: decode(&mut it),
                    currency: decode(&mut it),
                    local_symbol: decode(&mut it),
                    trading_class: decode(&mut it),
                    ..Default::default()
                };
                let mut order = order::Order {
                    contract,
                    order_id,
                    action: decode(&mut it).unwrap(),
                    total_qty: decode(&mut it).unwrap(),
                    order_type: decode(&mut it).unwrap(),
                    lmt_price: decode(&mut it),
                    aux_price: decode(&mut it),
                    tif: decode(&mut it),
                    oca_group: decode(&mut it),
                    account: decode(&mut it),
                    open_close:  decode(&mut it),
                    origin: decode(&mut it),
                    order_ref: decode(&mut it),
                    client_id: decode(&mut it).unwrap(),
                    perm_id: decode(&mut it).unwrap(),
                    outside_rth: decode(&mut it).unwrap(),
                    hidden: decode(&mut it).unwrap(),
                    discretionary_amt: decode(&mut it).unwrap(),
                    good_after_time: decode(&mut it),
                    fa_group: {it.next(); decode(&mut it)},
                    fa_method: decode(&mut it),
                    fa_percentage: decode(&mut it),
                    fa_profile: decode(&mut it),
                    model_code: decode(&mut it),
                    good_till_date: decode(&mut it),
                    rule_80A: decode(&mut it),
                    percent_offset: decode(&mut it),
                    settling_firm: decode(&mut it),
                    short_sale_slot: decode(&mut it),
                    designated_location: decode(&mut it),
                    exempt_code: decode(&mut it).unwrap(),
                    auction_strategy: decode(&mut it),
                    starting_price: decode(&mut it),
                    stock_ref_price: decode(&mut it),
                    delta: decode(&mut it),
                    stock_range_lower: decode(&mut it),
                    stock_range_upper: decode(&mut it),
                    display_size: decode(&mut it).unwrap(),
                    block_order: decode(&mut it).unwrap(),
                    sweep_to_fill: decode(&mut it).unwrap(),
                    all_or_none: decode(&mut it).unwrap(),
                    min_qty: decode(&mut it),
                    oca_type: decode(&mut it),
                    e_trade_only: decode(&mut it).unwrap(),
                    firm_quote_only: decode(&mut it).unwrap(),
                    nbbo_price_cap: decode(&mut it),
                    parent_id: decode(&mut it).unwrap(),
                    trigger_method: decode(&mut it),
                    volatility: decode(&mut it),
                    volatility_type: decode(&mut it),
                    delta_neutral_order_type: decode(&mut it),
                    ..Default::default()
                };
                if order.delta_neutral_order_type.is_some() {
                    order.delta_neutral_con_id = decode(&mut it).unwrap();
                    order.delta_neutral_settling_firm = decode(&mut it);
                    order.delta_neutral_clearing_account = decode(&mut it);
                    order.delta_neutral_clearing_intent = decode(&mut it);
                    order.delta_neutral_open_close = decode(&mut it);
                    order.delta_neutral_short_sale = decode(&mut it).unwrap();
                    order.delta_neutral_short_sale_slot = decode(&mut it).unwrap();
                    order.delta_neutral_designated_location = decode(&mut it);
                }
                order.continuous_update = decode(&mut it).unwrap();
                order.reference_price_type = decode(&mut it);
                order.trail_stop_price = decode(&mut it);
                order.trailing_percent = decode(&mut it);
                order.basis_points = decode(&mut it);
                order.basis_points_type = decode(&mut it);
                order.contract.combo_legs_description = decode(&mut it);
                let combo_legs_count: Option<usize> = decode(&mut it);
                if let Some(n) = combo_legs_count {
                    let mut legs = Vec::with_capacity(n);
                    for i in 0..n {
                        legs.push(ib_contract::ComboLeg {
                            con_id: decode(&mut it).unwrap(),
                            ratio: decode(&mut it).unwrap(),
                            action: decode(&mut it).unwrap(),
                            exchange: decode(&mut it).unwrap(),
                            open_close: decode(&mut it).unwrap(),
                            shortsale_slot: decode(&mut it).unwrap(),
                            designated_location: decode(&mut it).unwrap(),
                            exempt_code: decode(&mut it).unwrap(),
                        })
                    }
                    order.contract.combo_legs = Some(legs);
                }
                let order_combo_legs_count: Option<usize> = decode(&mut it);
                if let Some(n) = order_combo_legs_count {
                    let order_legs: Vec<Option<Decimal>> = Vec::with_capacity(n);
                    for i in 0..n {
                        order_legs.push(decode(&mut it));
                    }
                    order.order_combo_legs = Some(order_legs);
                }
                let smart_combo_routing_params_count: Option<usize> = decode(&mut it);
                if let Some(n) = smart_combo_routing_params_count {
                    let combo_params: Vec<(String,String)> = Vec::with_capacity(n);
                    for i in 0..n {
                        combo_params.push((decode(&mut it).unwrap(), decode(&mut it).unwrap()));
                    }
                }
                order.scale_init_level_size = decode(&mut it);
                order.scale_subs_level_size = decode(&mut it);
                order.scale_price_increment = decode(&mut it);
                if let Some(incr) = order.scale_price_increment {
                    if incr > 0.0 {
                        order.scale_price_adjust_value = decode(&mut it);
                        order.scale_price_adjust_interval = decode(&mut it);
                        order.scale_profit_offset = decode(&mut it);
                        order.scale_auto_reset = decode(&mut it).unwrap();
                        order.scale_init_position = decode(&mut it);
                        order.scale_init_fill_qty = decode(&mut it);
                        order.scale_random_percent = decode(&mut it).unwrap();
                    }
                }
                order.hedge_type = decode(&mut it);
                if let Some(ht) = order.hedge_type {
                    if ht != HedgeType::Undefined {
                        order.hedge_param = decode(&mut it);
                    }
                }
                order.opt_out_smart_routing = decode(&mut it).unwrap();
                order.clearing_account = decode(&mut it);
                order.clearing_intent = decode(&mut it);
                order.not_held = decode(&mut it).unwrap();
                let has_delta_neutral_contract: Option<bool> = decode(&mut it);
                if let Some(has_dnc) = has_delta_neutral_contract {
                    if has_dnc {
                        order.contract.delta_neutral_contract = Some(ib_contract::DeltaNeutralContract{
                            con_id: decode(&mut it).unwrap(),
                            delta: decode(&mut it).unwrap(),
                            price: decode(&mut it).unwrap()
                        });
                    }
                }
                order.algo_strategy = decode(&mut it);
                if order.algo_strategy.is_some() {
                    let params_count: Option<usize> = decode(&mut it);
                    if let Some(n) = params_count {
                        let params: Vec<(String,String)> = Vec::with_capacity(n);
                        for i in 0..n {
                            params.push((decode(&mut it).unwrap(),decode(&mut it).unwrap()));
                        }
                        order.algo_params = Some(params);
                    }
                }
                order.solicited = decode(&mut it).unwrap();
                order.what_if = decode(&mut it).unwrap();
                let order_state = order::OrderState{
                    status: decode(&mut it),
                    init_margin_before: decode(&mut it),
                    maint_margin_before: decode(&mut it),
                    equity_with_loan_value_before: decode(&mut it),
                    init_margin_change: decode(&mut it),
                    maint_margin_change: decode(&mut it),
                    equity_with_loan_change: decode(&mut it),
                    init_margin_after: decode(&mut it),
                    maint_margin_after: decode(&mut it),
                    equity_with_loan_after: decode(&mut it),
                    commission: decode(&mut it),
                    min_commission: decode(&mut it),
                    max_commission: decode(&mut it),
                    commission_currency: decode(&mut it),
                    warning_text: decode(&mut it),
                    ..Default::default()
                };
                order.randomize_size = decode(&mut it).unwrap();
                order.randomize_price = decode(&mut it).unwrap();
                if order.order_type == OrderType::PeggedToBenchmark {
                    order.reference_contract_id = decode(&mut it).unwrap();
                    order.is_pegged_change_amount_decrease = decode(&mut it).unwrap();
                    order.pegged_change_amount = decode(&mut it).unwrap();
                    order.reference_change_amount = decode(&mut it).unwrap();
                    order.reference_exchange_id = decode(&mut it);
                }
                let conditions_count: Option<usize> = decode(&mut it);
                if let Some(n) = conditions_count {
                    order.conditions = Some(Vec::with_capacity(n));
                    for i in 0..n {
                        order.conditions.unwrap().push(decode(&mut it).unwrap());
                    }
                    order.conditions_ignore_rth = decode(&mut it).unwrap();
                    order.conditions_cancel_order = decode(&mut it).unwrap();
                }
                order.adjusted_order_type = decode(&mut it);
                order.trigger_price = decode(&mut it);
                order.trail_stop_price = decode(&mut it);
                order.lmt_price_offset = decode(&mut it);
                order.adjusted_stop_price = decode(&mut it);
                order.adjusted_stop_limit_price = decode(&mut it);
                order.adjusted_trailing_amount = decode(&mut it);
                order.adjustable_trailing_unit = decode(&mut it).unwrap();
                let name: Option<String> = decode(&mut it);
                let val: Option<String> = decode(&mut it);
                let display_name: Option<String> = decode(&mut it);
                if name.is_some() || val.is_some() || display_name.is_some() {
                    order.soft_dollar_tier = Some(order::SoftDollarTier{
                        name,val,display_name
                    })
                }
                order.cash_qty = decode(&mut it);
                order.dont_use_auto_price_for_hedge = decode(&mut it).unwrap();
                order.is_oms_container = decode(&mut it).unwrap();
                order.discretionary_up_to_limit_price = decode(&mut it).unwrap();
                order.use_price_mgmt_algo = decode(&mut it);                
                IBFrame::OpenOrder{
                    order, order_state
                }
            },
            Incoming::CommissionReport => {
                it.next(); //skip version
                IBFrame::CommissionReport(
                    order::CommissionReport {
                        exec_id: decode(&mut it).unwrap(),
                        commission: decode(&mut it).unwrap(),
                        currency: decode(&mut it).unwrap(),
                        realized_pnl: decode(&mut it),
                        yield_amount: decode(&mut it),
                        yield_redemption_date: decode(&mut it)
                    }
                )
            },
            Incoming::ExecutionData => {
                it.next(); //skip version
                let order_id: usize = decode(&mut it).unwrap();
                let contract = ib_contract::Contract {
                    con_id: decode(&mut it),
                    symbol : decode(&mut it),
                    sec_type: decode(&mut it),
                    last_trade_date_or_contract_month: decode(&mut it),
                    strike: decode(&mut it),
                    right: decode(&mut it),
                    multiplier: decode(&mut it),
                    exchange: decode(&mut it),
                    currency: decode(&mut it),
                    local_symbol: decode(&mut it),
                    trading_class: decode(&mut it),
                    ..Default::default()
                };
                IBFrame::Execution(order::Execution {
                    order_id,
                    contract,
                    exec_id: decode(&mut it).unwrap(),
                    time: decode(&mut it).unwrap(),
                    acct_number: decode(&mut it).unwrap(),
                    exchange: decode(&mut it).unwrap(),
                    side: decode(&mut it).unwrap(),
                    shares: decode(&mut it).unwrap(),
                    price: decode(&mut it).unwrap(),
                    perm_id: decode(&mut it).unwrap(),
                    client_id: decode(&mut it).unwrap(),
                    liquidation: decode(&mut it).unwrap(),
                    cum_qty: decode(&mut it).unwrap(),
                    avg_price: decode(&mut it).unwrap(),
                    order_ref: decode(&mut it),
                    ev_rule: decode(&mut it),
                    ev_multiplier: decode(&mut it),
                    model_code: decode(&mut it),
                    last_liquidity: decode(&mut it)
                })
            },
            Incoming::OrderStatus => {
                IBFrame::OrderStatus(order::OrderStatus {
                    order_id: decode(&mut it).unwrap(),
                    status: decode(&mut it).unwrap(),
                    filled: decode(&mut it).unwrap(),
                    remaining: decode(&mut it).unwrap(),
                    avg_fill_price: decode(&mut it).unwrap(),
                    perm_id: decode(&mut it).unwrap(),
                    parent_id: decode(&mut it).unwrap(),
                    last_fill_price: decode(&mut it).unwrap(),
                    client_id: decode(&mut it).unwrap(),
                    why_held: decode(&mut it)
                })
            },
            Incoming::TickPrice => {
                it.next(); //skip version
                let id = decode(&mut it).unwrap();
                let kind = decode(&mut it).unwrap();
                let price = decode(&mut it).unwrap();
        
                let size = decode(&mut it);
                let mask: u32 = decode(&mut it).unwrap();
                let bits = BitSlice::<Lsb0, _>::from_element(&mask);
                let mut attributes = EnumSet::new();
                if bits[0] == true {attributes.insert(TickAttribute::CanAutoExecute);}
                if bits[1] == true {attributes.insert(TickAttribute::PastLimit);}
                if bits[2] == true {attributes.insert(TickAttribute::PreOpen);}
                IBFrame::PriceTick {
                    id,
                    kind,
                    price,
                    size,
                    attributes
                }
            }
            Incoming::TickSize => {
                it.next(); //skip version
                IBFrame::SizeTick {
                    id: decode(&mut it).unwrap(),
                    kind: decode(&mut it).unwrap(),
                    size: decode(&mut it).unwrap(),
                }
            },
            Incoming::TickString => {
                it.next(); //skip version
                IBFrame::StringTick {
                    id: decode(&mut it).unwrap(),
                    kind: decode(&mut it).unwrap(),
                    val: decode(&mut it)
                }
            },
            Incoming::TickGeneric => {
                it.next(); //skip version
                IBFrame::GenericTick {
                    id: decode(&mut it).unwrap(),
                    kind: decode(&mut it).unwrap(),
                    val: decode(&mut it).unwrap()
                }
            },
            Incoming::HistoricalData => {
                let id = decode(&mut it).unwrap();
                let start_dt: String = decode(&mut it).unwrap();
                let end_dt: String = decode(&mut it).unwrap();
                let n_bars = decode(&mut it).unwrap();
                let data = if n_bars > 0 {
                    let mut bar_data = Vec::with_capacity(n_bars);
                    for i in 0..n_bars {
                        bar_data.push(bars::Bar {
                            t_stamp: decode(&mut it).unwrap(),
                            open: decode(&mut it).unwrap(),
                            high: decode(&mut it).unwrap(),
                            low: decode(&mut it).unwrap(),
                            close: decode(&mut it).unwrap(),
                            volume: decode(&mut it).unwrap(),
                            wap: decode(&mut it).unwrap(),
                            count: decode(&mut it).unwrap()
                        });
                    }
                    Some(bar_data)
                } else {None};
                IBFrame::Bars{id, data: bars::BarSeries{start_dt, end_dt, n_bars, data}}
            }
            _ => IBFrame::NotImplemented
        }
        
    }
}