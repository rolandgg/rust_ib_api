use rust_decimal::prelude::*;
use chrono::NaiveDateTime;
use crate::account::Position;
use crate::contract;
use crate::utils::ib_message::decode;
use crate::order;
use crate::bars;
use crate::enums::*;
use log::debug;

use enumset::EnumSetType;
use enumset::EnumSet;
use bitvec::prelude::*;

#[derive(EnumSetType, Debug)]
pub enum TickAttribute {
   CanAutoExecute,
   PastLimit,
   PreOpen
}

pub(crate) enum IBFrame {
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
        req_id: i32, contract_details: contract::ContractDetails
    },
    ContractDetailsEnd(i32),
    OrderID(i32),
    OpenOrder{
        order: order::Order, order_state: order::OrderState
    },
    Execution(order::Execution),
    CommissionReport(order::CommissionReport),
    OrderStatus(order::OrderStatus),
    PriceTick{id: i32, kind: TickType, price: f64, size: Option<i32>, attributes: Option<EnumSet<TickAttribute>>},
    SizeTick{id: i32, kind: TickType, size: i32},
    StringTick{id: i32, kind: TickType, val: Option<String>},
    GenericTick{id: i32, kind: TickType, val: f64},
    Bars{id: i32, data: bars::BarSeries},
    Error{id: Option<i32>, code: Option<i32>, msg: Option<String>},
    NotImplemented
}

impl IBFrame {
    pub fn parse (msg: &[u8]) -> Option<Self> {
        let utf8msg = String::from_utf8_lossy(msg);
        let mut it = utf8msg.split("\0");
        let msg_type: Incoming = it.next()?.parse().ok()?;
        match msg_type {
            Incoming::AcctValue => {
                it.next(); //skip version
                match it.next() {
                    Some(val) => Some(match val {
                        "AccountCode" => IBFrame::AccountCode(decode(&mut it)),
                        "AccountType" => IBFrame::AccountType(decode(&mut it)),
                        "CashBalance" => IBFrame::CashBalance(decode(&mut it)),
                        "EquityWithLoanValue" => IBFrame::EquityWithLoanValue(decode(&mut it)),
                        "ExcessLiquidity" => IBFrame::ExcessLiquidity(decode(&mut it)),
                        "NetLiquidation" => IBFrame::NetLiquidation(decode(&mut it)),
                        "RealizedPnL" => IBFrame::RealizedPnL(decode(&mut it)),
                        "UnrealizedPnL" => IBFrame::UnrealizedPnL(decode(&mut it)),
                        "TotalCashBalance" => IBFrame::TotalCashBalance(decode(&mut it)),
                        &_ => IBFrame::NotImplemented}),
                    None => None
                }
            },
            Incoming::AcctDownloadEnd => {
                it.next(); //skip version
                Some(IBFrame::AccountUpdateEnd(decode(&mut it)))
            },
            Incoming::AcctUpdateTime => {
                it.next(); //skip version
                Some(IBFrame::AccountUpdateTime(decode(&mut it)))
            },
            Incoming::PortfolioValue => {
                let version: Option<i32> = decode(&mut it);
                let con_id = decode(&mut it);
                let mut contract = contract::Contract::default();
                contract.con_id = con_id;
                contract.symbol = decode(&mut it);
                contract.sec_type = decode(&mut it);
                contract.last_trade_date_or_contract_month = decode(&mut it);
                contract.strike = decode(&mut it);
                contract.right = decode(&mut it);
                if let Some(v) = version {
                    if v >= 7 {
                        contract.multiplier = decode(&mut it);
                        contract.primary_exchange = decode(&mut it);
                    }
                    contract.currency = decode(&mut it);
                    contract.local_symbol = decode(&mut it);
                    if v >= 8 {
                        contract.trading_class = decode(&mut it);
                    }
                }  
                Some(IBFrame::PortfolioValue(Position {
                    contract,
                    position: decode(&mut it),
                    market_price: decode(&mut it),
                    market_value: decode(&mut it),
                    average_cost: decode(&mut it),
                    unrealized_pnl: decode(&mut it),
                    realized_pnl: decode(&mut it)
                }))
            },
            Incoming::CurrentTime => {
                it.next(); //skip version
                let unix_time= decode(&mut it);
                if let Some(t) = unix_time {
                    Some(IBFrame::CurrentTime(NaiveDateTime::from_timestamp(t, 0)))
                }
                else {None}
            },
            Incoming::ContractData => {
                match decode(&mut it) {
                    None => None,
                    Some(id) => {
                        let req_id = id;
                        let mut contract = contract::Contract {
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
                        let mut details = contract::ContractDetails {
                            market_name: decode(&mut it),
                            ..Default::default()
                        };
                        contract.trading_class = decode(&mut it);
                        contract.con_id = decode(&mut it);
                        details.min_tick = decode(&mut it);
                        //details.md_size_multiplier = decode(&mut it);
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
                                    sec_ids.push((decode(&mut it).unwrap_or(String::from("")), decode(&mut it).unwrap_or(String::from(""))));
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
                        details.stock_type = decode(&mut it);
                        details.min_size = decode(&mut it);
                        details.size_increment = decode(&mut it);
                        details.suggested_size_increment = decode(&mut it);
                        details.contract = Some(contract);
                        Some(IBFrame::ContractDetails{
                            req_id,
                            contract_details: details
                        })
                    }
                }
            },
            Incoming::ContractDataEnd => {
                it.next(); //skip version
                Some(IBFrame::ContractDetailsEnd(decode(&mut it)?))
            },
            Incoming::NextValidId => {
                it.next(); //skip version
                Some(IBFrame::OrderID(decode(&mut it)?))
            },
            Incoming::OpenOrder => {
                let order_id: i32 = decode(&mut it)?;
                //decode contract
                let contract = contract::Contract {
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
                    action: decode(&mut it),
                    total_qty: decode(&mut it),
                    order_type: decode(&mut it),
                    lmt_price: decode(&mut it),
                    aux_price: decode(&mut it),
                    tif: decode(&mut it),
                    oca_group: decode(&mut it),
                    account: decode(&mut it),
                    open_close:  decode(&mut it),
                    origin: decode(&mut it),
                    order_ref: decode(&mut it),
                    client_id: decode(&mut it),
                    perm_id: decode(&mut it),
                    outside_rth: decode(&mut it),
                    hidden: decode(&mut it),
                    discretionary_amt: decode(&mut it),
                    good_after_time: decode(&mut it),
                    //skip shares allocation

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
                    exempt_code: decode(&mut it),
                    auction_strategy: decode(&mut it),
                    starting_price: decode(&mut it),
                    stock_ref_price: decode(&mut it),
                    delta: decode(&mut it),
                    stock_range_lower: decode(&mut it),
                    stock_range_upper: decode(&mut it),
                    display_size: decode(&mut it),
                    block_order: decode(&mut it),
                    sweep_to_fill: decode(&mut it),
                    all_or_none: decode(&mut it),
                    min_qty: decode(&mut it),
                    oca_type: decode(&mut it),
                    //e_trade_only: decode(&mut it),
                    //firm_quote_only: decode(&mut it),
                    //nbbo_price_cap: decode(&mut it),
                    parent_id: {it.next(); it.next(); it.next(); decode(&mut it)},
                    trigger_method: decode(&mut it),
                    volatility: decode(&mut it),
                    volatility_type: decode(&mut it),
                    delta_neutral_order_type: decode(&mut it),
                    delta_neutral_aux_price: decode(&mut it),
                    ..Default::default()
                };
                if order.delta_neutral_order_type.is_some() {
                    order.delta_neutral_con_id = decode(&mut it);
                    order.delta_neutral_settling_firm = decode(&mut it);
                    order.delta_neutral_clearing_account = decode(&mut it);
                    order.delta_neutral_clearing_intent = decode(&mut it);
                    order.delta_neutral_open_close = decode(&mut it);
                    order.delta_neutral_short_sale = decode(&mut it);
                    order.delta_neutral_short_sale_slot = decode(&mut it);
                    order.delta_neutral_designated_location = decode(&mut it);
                }
                order.continuous_update = decode(&mut it);
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
                        legs.push(contract::ComboLeg {
                            con_id: decode(&mut it),
                            ratio: decode(&mut it),
                            action: decode(&mut it),
                            exchange: decode(&mut it),
                            open_close: decode(&mut it),
                            shortsale_slot: decode(&mut it),
                            designated_location: decode(&mut it),
                            exempt_code: decode(&mut it),
                        })
                    }
                    order.contract.combo_legs = Some(legs);
                }
                let order_combo_legs_count: Option<usize> = decode(&mut it);
                if let Some(n) = order_combo_legs_count {
                    let mut order_legs: Vec<Option<Decimal>> = Vec::with_capacity(n);
                    for i in 0..n {
                        order_legs.push(decode(&mut it));
                    }
                    order.order_combo_legs = Some(order_legs);
                }
                let smart_combo_routing_params_count: Option<usize> = decode(&mut it);
                if let Some(n) = smart_combo_routing_params_count {
                    let mut combo_params: Vec<(String,String)> = Vec::with_capacity(n);
                    for i in 0..n {
                        combo_params.push((decode(&mut it).unwrap_or(String::from("")), decode(&mut it).unwrap_or(String::from(""))));
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
                        order.scale_auto_reset = decode(&mut it);
                        order.scale_init_position = decode(&mut it);
                        order.scale_init_fill_qty = decode(&mut it);
                        order.scale_random_percent = decode(&mut it);
                    }
                }
                order.hedge_type = decode(&mut it);
                if let Some(ht) = &order.hedge_type {
                    if *ht != HedgeType::Undefined {
                        order.hedge_param = decode(&mut it);
                    }
                }
                order.opt_out_smart_routing = decode(&mut it);
                order.clearing_account = decode(&mut it);
                order.clearing_intent = decode(&mut it);
                order.not_held = decode(&mut it);
                let has_delta_neutral_contract: Option<bool> = decode(&mut it);
                if let Some(has_dnc) = has_delta_neutral_contract {
                    if has_dnc {
                        order.contract.delta_neutral_contract = Some(contract::DeltaNeutralContract{
                            con_id: decode(&mut it),
                            delta: decode(&mut it),
                            price: decode(&mut it)
                        });
                    }
                }
                order.algo_strategy = decode(&mut it);
                if order.algo_strategy.is_some() {
                    let params_count: Option<usize> = decode(&mut it);
                    if let Some(n) = params_count {
                        let mut params: Vec<(String,String)> = Vec::with_capacity(n);
                        for i in 0..n {
                            params.push((decode(&mut it).unwrap_or(String::from("")),decode(&mut it).unwrap_or(String::from(""))));
                        }
                        order.algo_params = Some(params);
                    }
                }
                order.solicited = decode(&mut it);
                order.what_if = decode(&mut it);
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
                order.randomize_size = decode(&mut it);
                order.randomize_price = decode(&mut it);
                if order.order_type == Some(OrderType::PeggedToBenchmark) {
                    order.reference_contract_id = decode(&mut it);
                    order.is_pegged_change_amount_decrease = decode(&mut it);
                    order.pegged_change_amount = decode(&mut it);
                    order.reference_change_amount = decode(&mut it);
                    order.reference_exchange_id = decode(&mut it);
                }
                let conditions_count: Option<usize> = decode(&mut it);
                if let Some(n) = conditions_count 
                {
                    if n > 0 {
                        let mut conditions = Vec::with_capacity(n);
                        for i in 0..n {
                            conditions.push(decode(&mut it));
                        }
                        order.conditions = Some(conditions);
                        order.conditions_ignore_rth = decode(&mut it);
                        order.conditions_cancel_order = decode(&mut it);
                    }  
                }
                order.adjusted_order_type = decode(&mut it);
                order.trigger_price = decode(&mut it);
                order.trail_stop_price = decode(&mut it);
                order.lmt_price_offset = decode(&mut it);
                order.adjusted_stop_price = decode(&mut it);
                order.adjusted_stop_limit_price = decode(&mut it);
                order.adjusted_trailing_amount = decode(&mut it);
                order.adjustable_trailing_unit = decode(&mut it);
                let name: Option<String> = decode(&mut it);
                let val: Option<String> = decode(&mut it);
                let display_name: Option<String> = decode(&mut it);
                if name.is_some() || val.is_some() || display_name.is_some() {
                    order.soft_dollar_tier = Some(order::SoftDollarTier{
                        name,val,display_name
                    })
                }
                order.cash_qty = decode(&mut it);
                order.dont_use_auto_price_for_hedge = decode(&mut it);
                order.is_oms_container = decode(&mut it);
                order.discretionary_up_to_limit_price = decode(&mut it);
                order.use_price_mgmt_algo = decode(&mut it); 
                order.duration = decode(&mut it);
                order.post_to_ats = decode(&mut it);
                order.auto_cancel_parent = decode(&mut it);
                order.min_trade_qty = decode(&mut it);
                order.min_compete_size = decode(&mut it);
                order.compete_against_best_offset = decode(&mut it);
                order.mid_offset_at_whole = decode(&mut it);
                order.mid_offset_at_half = decode(&mut it);

                Some(IBFrame::OpenOrder{
                    order, order_state
                })
            },
            Incoming::CommissionReport => {
                it.next(); //skip version
                Some(IBFrame::CommissionReport(
                    order::CommissionReport {
                        exec_id: decode(&mut it),
                        commission: decode(&mut it),
                        currency: decode(&mut it),
                        realized_pnl: decode(&mut it),
                        yield_amount: decode(&mut it),
                        yield_redemption_date: decode(&mut it)
                    }
                ))
            },
            Incoming::ExecutionData => {
                it.next(); //skip version
                let order_id: i32 = decode(&mut it)?;
                let contract = contract::Contract {
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
                Some(IBFrame::Execution(order::Execution {
                    order_id,
                    contract,
                    exec_id: decode(&mut it),
                    time: decode(&mut it),
                    acct_number: decode(&mut it),
                    exchange: decode(&mut it),
                    side: decode(&mut it),
                    shares: decode(&mut it),
                    price: decode(&mut it),
                    perm_id: decode(&mut it),
                    client_id: decode(&mut it),
                    liquidation: decode(&mut it),
                    cum_qty: decode(&mut it),
                    avg_price: decode(&mut it),
                    order_ref: decode(&mut it),
                    ev_rule: decode(&mut it),
                    ev_multiplier: decode(&mut it),
                    model_code: decode(&mut it),
                    last_liquidity: decode(&mut it)
                }))
            },
            Incoming::OrderStatus => {
                Some(IBFrame::OrderStatus(order::OrderStatus {
                    order_id: decode(&mut it)?,
                    status: decode(&mut it),
                    filled: decode(&mut it),
                    remaining: decode(&mut it),
                    avg_fill_price: decode(&mut it),
                    perm_id: decode(&mut it),
                    parent_id: decode(&mut it),
                    last_fill_price: decode(&mut it),
                    client_id: decode(&mut it),
                    why_held: decode(&mut it),
                    mkt_cap_price: decode(&mut it)
                }))
            },
            Incoming::TickPrice => {
                it.next(); //skip version
                let id = decode(&mut it)?;
                let kind = decode(&mut it)?;
                let price = decode(&mut it)?;
        
                let size = decode(&mut it);
                let mask: Option<u32> = decode(&mut it);
                let attributes = match mask {
                    Some(m) => {
                        let bits = m.view_bits::<Lsb0>();
                        let mut attributes = EnumSet::new();
                        if bits[0] == true {attributes.insert(TickAttribute::CanAutoExecute);}
                        if bits[1] == true {attributes.insert(TickAttribute::PastLimit);}
                        if bits[2] == true {attributes.insert(TickAttribute::PreOpen);}
                        Some(attributes)
                    }
                    None => None
                };
                
                Some(IBFrame::PriceTick {
                    id,
                    kind,
                    price,
                    size,
                    attributes
                })
            }
            Incoming::TickSize => {
                it.next(); //skip version
                Some(IBFrame::SizeTick {
                    id: decode(&mut it)?,
                    kind: decode(&mut it)?,
                    size: decode(&mut it)?,
                })
            },
            Incoming::TickString => {
                it.next(); //skip version
                Some(IBFrame::StringTick {
                    id: decode(&mut it)?,
                    kind: decode(&mut it)?,
                    val: decode(&mut it)
                })
            },
            Incoming::TickGeneric => {
                it.next(); //skip version
                Some(IBFrame::GenericTick {
                    id: decode(&mut it)?,
                    kind: decode(&mut it)?,
                    val: decode(&mut it)?
                })
            },
            Incoming::HistoricalData => {
                let id = decode(&mut it)?;
                let start_dt = decode(&mut it);
                let end_dt = decode(&mut it);
                let n_bars = decode(&mut it);
                let data = if let Some(nb) = n_bars {
                    if nb > 0 {
                        let mut bar_data = Vec::with_capacity(nb);
                        for i in 0..nb {
                            bar_data.push(bars::Bar {
                                t_stamp: decode(&mut it),
                                open: decode(&mut it),
                                high: decode(&mut it),
                                low: decode(&mut it),
                                close: decode(&mut it),
                                volume: decode(&mut it),
                                wap: decode(&mut it),
                                count: decode(&mut it)
                            });
                        }
                        Some(bar_data)} else {None}
                } else {None};
                Some(IBFrame::Bars{id, data: bars::BarSeries{start_dt, end_dt, n_bars, data}})
            }
            Incoming::ErrMsg => {
                it.next(); //skip version
                Some(IBFrame::Error {
                    id: decode(&mut it),
                    code: decode(&mut it),
                    msg: decode(&mut it)
                })
            }
            _ => Some(IBFrame::NotImplemented)
        }
        
    }
}