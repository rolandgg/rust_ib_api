use crate::enums::*;
use rust_decimal::prelude::*;
use crate::utils::ib_message::Encodable;
use crate::contract::Contract;
use crossbeam::channel;
use tokio::sync::watch;

///Is returned upon successful submission of an order and allows to track the order state.
#[derive(Debug,Clone)]
pub struct OrderTracker {
    order_rx: watch::Receiver<Order>,
    error: Option<(i32,String)>,
    //order_state: OrderState,
    order_state_rx: watch::Receiver<OrderState>,
    order_status_rx: watch::Receiver<Option<OrderStatus>>,
    //order_status: Option<OrderStatus>,
    executions: Vec<Execution>,
    executions_rx: channel::Receiver<Execution>,
    commission_reports: Vec<CommissionReport>,
    commission_reports_rx: channel::Receiver<CommissionReport>
}

pub(crate) struct OrderTrackerSender {
    pub executions_tx: channel::Sender<Execution>,
    pub order_tx: watch::Sender<Order>,
    pub order_status_tx: watch::Sender<Option<OrderStatus>>,
    pub order_state_tx: watch::Sender<OrderState>,
    pub commission_reports_tx: channel::Sender<CommissionReport>
}

impl OrderTracker {
    pub(crate) fn new(order: Order, order_state: OrderState) -> (OrderTrackerSender, Self) {
        let (executions_tx, executions_rx) = channel::unbounded();
        let (commission_reports_tx, commission_reports_rx) = channel::unbounded();
        let (order_status_tx, order_status_rx) = watch::channel(None);
        let (order_state_tx, order_state_rx) = watch::channel(order_state);
        let (order_tx, order_rx) = watch::channel(order);
        (OrderTrackerSender {
            order_tx,
            executions_tx,
            commission_reports_tx,
            order_status_tx,
            order_state_tx,
        },
        OrderTracker {
            order_rx,
            error: None,
            commission_reports: Vec::new(),
            executions: Vec::new(),
            executions_rx,
            commission_reports_rx,
            order_status_rx,
            order_state_rx,
        })
    }

    fn update_exec(&mut self) {
        while let Ok(ex) = self.executions_rx.try_recv() {
            self.executions.push(ex);
        }
    }

    fn update_com(&mut self) {
        while let Ok(com) = self.commission_reports_rx.try_recv() {
            self.commission_reports.push(com);
        }
    }
    ///Returns the order status.
    pub fn status(&self) -> Option<String> {
        match &*self.order_status_rx.borrow() {
            Some(stat) => stat.status.clone(),
            None => None
        }
    }
    ///Checks if the order status is 'filled', i.e. the order execution is completed.
    pub fn is_filled(&self) -> Option<bool> {
        if let Some(stat) = &*self.order_status_rx.borrow() {
            if let Some(st) = &stat.status{
                if st == "Filled" {
                return Some(true);
                } 
            }
            else {return Some(false)};
        }
        return None;
    }
    ///Returns the time when the order was completely filled.
    pub fn fill_time(&self) -> Option<String> {
        self.order_state_rx.borrow().completed_time.clone() 
    }
    ///Returns the currently filled quantity.
    pub fn qty_filled(&self) -> Option<Decimal> {
        match &*self.order_status_rx.borrow() {
            Some(stat) => stat.filled.clone(),
            None => None
        }
    }
    ///Returns the average fill price.
    pub fn avg_fill_price(&self) -> Option<Decimal> {
        match &*self.order_status_rx.borrow() {
            Some(stat) => stat.avg_fill_price.clone(),
            None => None
        }
    }
    ///Returns total commissions paid.
    pub fn commissions_paid(&mut self) -> Option<Decimal> {
        self.update_com();
        if self.commission_reports.len() > 0 {
            Some(self.commission_reports.iter().fold(Decimal::new(0, 2), |acc, x| acc + x.commission.unwrap_or(Decimal::new(0,2))))
        }
        else {
            None
        }
    }
}

#[derive(Default,Debug,Clone)]
pub(crate) struct SoftDollarTier {
    pub(crate) name: Option<String>,
    pub(crate) val: Option<String>,
    pub(crate) display_name: Option<String>
}

/// Holds all order parameters. This is a huge struct with more than 150 fields. Orders are created through
/// factory functions ensuring that the created orders are in a valid state accepted by the TWS API.
/// Only basic order types are currently supported.
#[derive(Default,Debug,Clone)]
pub struct Order {

    //contract
    pub(crate) contract: Contract,

    //order identification
    pub(crate) order_id: i32,
    pub(crate) client_id: Option<usize>,
    pub(crate) perm_id: Option<i32>,

    //main order fields
    pub(crate) action: Option<Action>,
    pub(crate) total_qty: Option<Decimal>,
    pub(crate) order_type: Option<OrderType>,
    pub(crate) lmt_price: Option<Decimal>,
    pub(crate) aux_price: Option<Decimal>,

    //extended order fields
    pub(crate) tif: Option<TimeInForce>,
    pub(crate) active_start_time: Option<String>,
    pub(crate) active_stop_time: Option<String>,
    pub(crate) oca_group: Option<String>,
    pub(crate) oca_type: Option<OCAType>,
    pub(crate) order_ref: Option<String>,
    pub(crate) transmit: Option<bool>,
    pub(crate) parent_id: Option<usize>,
    pub(crate) block_order: Option<bool>,
    pub(crate) sweep_to_fill: Option<bool>,
    pub(crate) display_size: Option<i32>,
    pub(crate) trigger_method: Option<TriggerMethod>,
    pub(crate) outside_rth: Option<bool>,
    pub(crate) hidden: Option<bool>,
    pub(crate) good_after_time: Option<String>,
    pub(crate) good_till_date: Option<String>,
    pub(crate) override_percentage_constraints: Option<bool>,
    pub(crate) rule_80A: Option<Rule80A>,
    pub(crate) all_or_none: Option<bool>,
    pub(crate) min_qty: Option<i32>,
    pub(crate) percent_offset: Option<f64>,
    pub(crate) trail_stop_price: Option<Decimal>,
    pub(crate) trailing_percent: Option<f64>,

    // financial advisor fields
    pub(crate) fa_group: Option<String>,
    pub(crate) fa_profile: Option<String>,
    pub(crate) fa_method: Option<String>,
    pub(crate) fa_percentage: Option<String>,

    // institutional (i.e. non-cleared) only
    pub(crate) open_close: Option<OrderOpenClose>,
    pub(crate) origin: Option<Origin>,
    pub(crate) short_sale_slot: Option<ShortSaleSlot>,
    pub(crate) designated_location: Option<String>,
    pub(crate) exempt_code: Option<i32>,

    // SMART routing fields
    pub(crate) discretionary_amt: Option<f64>,
    pub(crate) e_trade_only: Option<bool>,
    pub(crate) firm_quote_only: Option<bool>,
    pub(crate) nbbo_price_cap: Option<Decimal>,
    pub(crate) opt_out_smart_routing: Option<bool>,

    // BOX exchange order fields
    pub(crate) auction_strategy: Option<AuctionStrategy>,
    pub(crate) starting_price: Option<Decimal>,
    pub(crate) stock_ref_price: Option<Decimal>,
    pub(crate) delta: Option<f64>,

    // Pegged to stock and VOL order fields

    pub(crate) stock_range_lower: Option<Decimal>,
    pub(crate) stock_range_upper: Option<Decimal>,

    pub(crate) randomize_size: Option<bool>,
    pub(crate) randomize_price: Option<bool>,

    // Volatility order fields
    pub(crate) volatility: Option<f64>,
    pub(crate) volatility_type: Option<VolatilityType>,
    pub(crate) delta_neutral_order_type: Option<OrderType>,
    pub(crate) delta_neutral_aux_price: Option<Decimal>,
    pub(crate) delta_neutral_con_id: Option<usize>,
    pub(crate) delta_neutral_settling_firm: Option<String>,
    pub(crate) delta_neutral_clearing_account: Option<String>,
    pub(crate) delta_neutral_clearing_intent: Option<String>,
    pub(crate) delta_neutral_open_close: Option<String>,
    pub(crate) delta_neutral_short_sale: Option<bool>,
    pub(crate) delta_neutral_short_sale_slot: Option<bool>,
    pub(crate) delta_neutral_designated_location: Option<String>,
    pub(crate) continuous_update: Option<bool>,
    pub(crate) reference_price_type: Option<ReferencePriceType>,

    // Combo order fields
    pub(crate) basis_points: Option<Decimal>,
    pub(crate) basis_points_type: Option<BasisPointsType>,

    // Scale order fields
    pub(crate) scale_init_level_size: Option<i32>,
    pub(crate) scale_subs_level_size: Option<i32>,
    pub(crate) scale_price_increment: Option<f64>,
    pub(crate) scale_price_adjust_value: Option<f64>,
    pub(crate) scale_price_adjust_interval: Option<i32>,
    pub(crate) scale_profit_offset: Option<f64>,
    pub(crate) scale_auto_reset: Option<bool>,
    pub(crate) scale_init_position: Option<i32>,
    pub(crate) scale_init_fill_qty: Option<i32>,
    pub(crate) scale_random_percent: Option<bool>,
    pub(crate) scale_table: Option<String>,

    // Hedge order fields
    pub(crate) hedge_type: Option<HedgeType>,
    pub(crate) hedge_param: Option<String>, // 'beta=X' value for beta hedge, 'ratio=Y' for pair hedge

    // Clearing info
    pub(crate) account: Option<String>,
    pub(crate) settling_firm: Option<String>,
    pub(crate) clearing_account: Option<String>,
    pub(crate) clearing_intent: Option<ClearingIntent>,

    // Algo order fields
    pub(crate) algo_strategy: Option<String>,
    pub(crate) algo_params: Option<Vec<(String,String)>>,
    pub(crate) smart_combo_routing_params: Option<Vec<(String,String)>>,
    pub(crate) algo_id: Option<String>,

    // What-if
    pub(crate) what_if: Option<bool>,

    // Not held
    pub(crate) not_held: Option<bool>,
    pub(crate) solicited: Option<bool>,

    // Models
    pub(crate) model_code: Option<String>,

    // Order combo legs

    pub(crate) order_combo_legs: Option<Vec<Option<Decimal>>>,
    pub(crate) order_misc_options: Option<Vec<(String,String)>>,

    // VER PEG2BENCH fields

    pub(crate) reference_contract_id: Option<i32>,
    pub(crate) pegged_change_amount: Option<f64>,
    pub(crate) is_pegged_change_amount_decrease: Option<bool>,
    pub(crate) reference_change_amount: Option<f64>,
    pub(crate) reference_exchange_id: Option<String>,
    pub(crate) adjusted_order_type: Option<String>,
    pub(crate) trigger_price: Option<f64>,
    pub(crate) adjusted_stop_price: Option<f64>,
    pub(crate) adjusted_stop_limit_price: Option<f64>,
    pub(crate) adjusted_trailing_amount: Option<f64>,
    pub(crate) adjustable_trailing_unit: Option<i32>,
    pub(crate) lmt_price_offset: Option<f64>,

    pub(crate) conditions: Option<Vec<Option<OrderConditionType>>>,
    pub(crate) conditions_cancel_order: Option<bool>,
    pub(crate) conditions_ignore_rth: Option<bool>,

    // ext operator
    pub(crate) ext_operator: Option<String>,

    pub(crate) soft_dollar_tier: Option<SoftDollarTier>,

    pub(crate) cash_qty: Option<Decimal>,

    pub(crate) mifid_2_decision_maker: Option<String>,
    pub(crate) mifid_2_decision_algo: Option<String>,
    pub(crate) mifid_2_execution_trader: Option<String>,
    pub(crate) mifid_2_execution_algo: Option<String>,

    pub(crate) dont_use_auto_price_for_hedge: Option<bool>,
    pub(crate) is_oms_container: Option<bool>,
    pub(crate) discretionary_up_to_limit_price: Option<bool>,
    pub(crate) auto_cancel_date: Option<String>,
    pub(crate) filled_quantity: Option<Decimal>,
    pub(crate) ref_futures_con_id: Option<usize>,
    pub(crate) auto_cancel_parent: Option<bool>,
    pub(crate) shareholder: Option<String>,
    pub(crate) imbalance_only: Option<bool>,
    pub(crate) route_marketable_to_bbo: Option<bool>,
    pub(crate) parent_perm_id: Option<usize>,
    pub(crate) use_price_mgmt_algo: Option<UsePriceMgmtAlgo>,
    pub(crate) duration: Option<i32>,
    pub(crate) post_to_ats: Option<i32>,
    pub(crate) advanced_error_override: Option<String>,
    pub(crate) manual_order_time: Option<String>,
    pub(crate) min_trade_qty: Option<i32>
}

impl Order {
    fn new() -> Self {
        Order {
            transmit: Some(true),
            open_close: Some(OrderOpenClose::Open),
            origin: Some(Origin::Customer),
            exempt_code: Some(-1),
            e_trade_only: Some(true),
            firm_quote_only: Some(true),
            auction_strategy: Some(AuctionStrategy::NoAuctionStrategy),
            ..Default::default()
        }
    }
    /// Creates a simple market order.
    pub fn market(contract: Contract, action: Action, qty: Decimal) -> Self {
        let mut order = Order::new();
        order.action = Some(action);
        order.contract = contract;
        order.total_qty = Some(qty);
        order
    }
    /// Sets the necessary parameters to submit a Combo order.
    pub fn combo(mut self) -> Self {
        self.smart_combo_routing_params = Some(vec![("NonGuaranteed".to_string(), "1".to_string())]);
        self
    }
    /// Creates a market-on-close order. Only possible for stocks that trade on NYSE or NASDAQ.
    pub fn market_on_close(contract: Contract, action: Action, qty: Decimal) -> Self {
        let mut order = Order::new();
        order.action = Some(action);
        order.contract = contract;
        order.total_qty = Some(qty);
        order.order_type = Some(OrderType::MarketOnClose);
        order
    }
    /// Creates a relative market order.
    pub fn relative_market(contract: Contract, action: Action, qty: Decimal) -> Self {
        let mut order = Order::new();
        order.action = Some(action);
        order.contract = contract;
        order.total_qty = Some(qty);
        order.order_type = Some(OrderType::RelativeMarket);
        order
    }
    /// Creates a limit order.
    pub fn limit(contract: Contract, action: Action, qty: Decimal, lmt: Decimal, tif: TimeInForce) -> Self {
        let mut order = Order::new();
        order.action = Some(action);
        order.contract = contract;
        order.total_qty = Some(qty);
        order.order_type = Some(OrderType::Limit);
        order.lmt_price = Some(lmt);
        order.tif = Some(tif);
        order
    }
}

impl Encodable for Order {
    fn encode(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.contract.encode_for_order());
        code.push_str(&self.action.encode());
        code.push_str(&self.total_qty.encode());
        code.push_str(&self.order_type.encode());
        code.push_str(&self.lmt_price.encode());
        code.push_str(&self.aux_price.encode());
        code.push_str(&self.tif.encode());
        code.push_str(&self.oca_group.encode());
        code.push_str(&self.account.encode());
        code.push_str(&self.open_close.encode());
        code.push_str(&self.origin.encode());
        code.push_str(&self.order_ref.encode());
        code.push_str(&self.transmit.encode());
        code.push_str(&self.parent_id.encode());
        code.push_str(&self.block_order.encode());
        code.push_str(&self.sweep_to_fill.encode());
        code.push_str(&self.display_size.encode());
        code.push_str(&self.trigger_method.encode());
        code.push_str(&self.outside_rth.encode());
        code.push_str(&self.hidden.encode());
        if let Some(sec) = &self.contract.sec_type {
            if *sec == SecType::Combo {
                match &self.contract.combo_legs {
                    Some(legs) => {
                        code.push_str(&legs.len().encode());
                        for leg in legs {
                            code.push_str(&leg.con_id.encode());
                            code.push_str(&leg.ratio.encode());
                            code.push_str(&leg.action.encode());
                            code.push_str(&leg.exchange.encode());
                            code.push_str(&leg.open_close.encode());
                            code.push_str(&leg.shortsale_slot.encode());
                            code.push_str(&leg.designated_location.encode());
                            code.push_str(&leg.exempt_code.encode());
                        };
                    }
                    None => code.push_str("0\0"),
                }
                match &self.order_combo_legs {
                    Some(legs) => {
                        code.push_str(&legs.len().encode());
                        for leg in legs {
                            code.push_str(&leg.encode());
                        }
                    }
                    None => code.push_str("0\0"),
                }
                match &self.smart_combo_routing_params {
                    Some(tag_val_list) => {
                        code.push_str(&tag_val_list.len().encode());
                        for tv in tag_val_list {
                            code.push_str(&tv.0.encode());
                            code.push_str(&tv.1.encode());
                        };
                    }
                    None => code.push_str("0\0"),
                }
            }  
        }
        code.push_str("\0"); //deprecated shares allocation field
        code.push_str(&self.discretionary_amt.encode());
        code.push_str(&self.good_after_time.encode());
        code.push_str(&self.good_till_date.encode());
        code.push_str(&self.fa_group.encode());
        code.push_str(&self.fa_method.encode());
        code.push_str(&self.fa_percentage.encode());
        code.push_str(&self.fa_profile.encode());
        code.push_str(&self.model_code.encode());
        code.push_str(&self.short_sale_slot.encode());
        code.push_str(&self.designated_location.encode());
        code.push_str(&self.exempt_code.encode());
        code.push_str(&self.oca_type.encode());
        code.push_str(&self.rule_80A.encode());
        code.push_str(&self.settling_firm.encode());
        code.push_str(&self.all_or_none.encode());
        code.push_str(&self.min_qty.encode());
        code.push_str(&self.percent_offset.encode());
        code.push_str(&self.e_trade_only.encode());
        code.push_str(&self.firm_quote_only.encode());
        code.push_str(&self.nbbo_price_cap.encode());
        code.push_str(&self.auction_strategy.encode());
        code.push_str(&self.starting_price.encode());
        code.push_str(&self.stock_ref_price.encode());
        code.push_str(&self.delta.encode());
        code.push_str(&self.stock_range_lower.encode());
        code.push_str(&self.stock_range_upper.encode());
        code.push_str(&self.override_percentage_constraints.encode());
        code.push_str(&self.volatility.encode());
        code.push_str(&self.volatility_type.encode());
        code.push_str(&self.delta_neutral_order_type.encode());
        code.push_str(&self.delta_neutral_aux_price.encode());
        if self.delta_neutral_order_type.is_some() {
            code.push_str(&self.delta_neutral_con_id.encode());
            code.push_str(&self.delta_neutral_settling_firm.encode());
            code.push_str(&self.delta_neutral_clearing_account.encode());
            code.push_str(&self.delta_neutral_clearing_intent.encode());
            code.push_str(&self.delta_neutral_open_close.encode());
            code.push_str(&self.delta_neutral_short_sale.encode());
            code.push_str(&self.delta_neutral_short_sale_slot.encode());
            code.push_str(&self.delta_neutral_designated_location.encode());
        }
        code.push_str(&self.continuous_update.encode());
        code.push_str(&self.reference_price_type.encode());
        code.push_str(&self.trail_stop_price.encode());
        code.push_str(&self.trailing_percent.encode());
        code.push_str(&self.scale_init_level_size.encode());
        code.push_str(&self.scale_subs_level_size.encode());
        code.push_str(&self.scale_price_increment.encode());
        if let Some(inc) = self.scale_price_increment {
            if inc > 0.0 {
                code.push_str(&self.scale_price_adjust_value.encode());
                code.push_str(&self.scale_price_adjust_interval.encode());
                code.push_str(&self.scale_profit_offset.encode());
                code.push_str(&self.scale_auto_reset.encode());
                code.push_str(&self.scale_init_position.encode());
                code.push_str(&self.scale_init_fill_qty.encode());
                code.push_str(&self.scale_random_percent.encode());
            }
        }
        code.push_str(&self.scale_table.encode());
        code.push_str(&self.active_start_time.encode());
        code.push_str(&self.active_stop_time.encode());
        code.push_str(&self.hedge_type.encode());
        if self.hedge_type.is_some() {
            code.push_str(&self.hedge_param.encode());
        }
        code.push_str(&self.opt_out_smart_routing.encode());
        code.push_str(&self.clearing_account.encode());
        code.push_str(&self.clearing_intent.encode());
        code.push_str(&self.not_held.encode());
        match &self.contract.delta_neutral_contract {
            Some(dn) => {
                code.push_str("1\0");
                code.push_str(&dn.con_id.encode());
                code.push_str(&dn.delta.encode());
                code.push_str(&dn.price.encode());
            }
            None => code.push_str("0\0")
        };
        code.push_str(&self.algo_strategy.encode());
        if self.algo_strategy.is_some() {
            match &self.algo_params {
                Some(params) => {
                    code.push_str(&params.len().encode());
                    for param in params {
                        code.push_str(&param.0.encode());
                        code.push_str(&param.1.encode());
                    };
                },
                None => code.push_str("0\0")
            }
        }
        code.push_str(&self.algo_id.encode());
        code.push_str(&self.what_if.encode());
        code.push_str(&self.order_misc_options.encode());
        code.push_str(&self.solicited.encode());
        code.push_str(&self.randomize_size.encode());
        code.push_str(&self.randomize_price.encode());

        if self.order_type == Some(OrderType::PeggedToBenchmark) {
            code.push_str(&self.reference_contract_id.encode());
            code.push_str(&self.is_pegged_change_amount_decrease.encode());
            code.push_str(&self.pegged_change_amount.encode());
            code.push_str(&self.reference_change_amount.encode());
            code.push_str(&self.reference_exchange_id.encode());
        }

        match &self.conditions {
            Some(conds) => {
                code.push_str(&conds.len().encode());
                for cond in conds {
                    //C++ API has some facility for external notification here
                    code.push_str(&cond.encode());
                };
                code.push_str(&self.conditions_ignore_rth.encode());
                code.push_str(&self.conditions_cancel_order.encode());
            }
            None => code.push_str("0\0")
        }

        code.push_str(&self.adjusted_order_type.encode());
        code.push_str(&self.trigger_price.encode());
        code.push_str(&self.lmt_price_offset.encode());
        code.push_str(&self.adjusted_stop_price.encode());
        code.push_str(&self.adjusted_stop_limit_price.encode());
        code.push_str(&self.adjusted_trailing_amount.encode());
        code.push_str(&self.adjustable_trailing_unit.encode());
        code.push_str(&self.ext_operator.encode());
        match &self.soft_dollar_tier {
            Some (tier) => {
                code.push_str(&tier.name.encode());
                code.push_str(&tier.val.encode());
            },
            None => code.push_str("\0\0")
        }
        code.push_str(&self.cash_qty.encode());

        code.push_str(&self.mifid_2_decision_maker.encode());
        code.push_str(&self.mifid_2_decision_algo.encode());
        code.push_str(&self.mifid_2_execution_trader.encode());
        code.push_str(&self.mifid_2_execution_algo.encode());

        code.push_str(&self.dont_use_auto_price_for_hedge.encode());
        code.push_str(&self.is_oms_container.encode());
        code.push_str(&self.discretionary_up_to_limit_price.encode());
        code.push_str(&self.use_price_mgmt_algo.encode());
        code.push_str(&self.duration.encode());
        code.push_str(&self.post_to_ats.encode());
        code.push_str(&self.auto_cancel_parent.encode());
        code.push_str(&self.advanced_error_override.encode());
        code.push_str(&self.manual_order_time.encode());
        if &self.contract.exchange == &Some("IBKRATS".to_string()){
            code.push_str(&self.min_trade_qty);
        }
        code
    }
}  

#[derive(Default,Debug,Clone)]
pub(crate) struct OrderState {
    pub status: Option<String>,
    pub init_margin_before: Option<Decimal>,
    pub maint_margin_before: Option<Decimal>,
    pub init_margin_change: Option<Decimal>,
    pub equity_with_loan_value_before: Option<Decimal>,
    pub maint_margin_change: Option<Decimal>,
    pub equity_with_loan_change: Option<Decimal>,
    pub init_margin_after: Option<Decimal>,
    pub maint_margin_after: Option<Decimal>,
    pub equity_with_loan_after: Option<Decimal>,
    pub commission: Option<Decimal>,
    pub min_commission: Option<Decimal>,
    pub max_commission: Option<Decimal>,
    pub commission_currency: Option<String>,
    pub warning_text: Option<String>,
    pub completed_time: Option<String>,
    pub completed_status: Option<String>,
}
#[derive(Default,Debug,Clone)]
pub(crate) struct OrderStatus {
    pub order_id: i32,
    pub status: Option<String>,
    pub filled: Option<Decimal>,
    pub remaining: Option<Decimal>,
    pub avg_fill_price: Option<Decimal>,
    pub perm_id: Option<i32>,
    pub parent_id: Option<usize>,
    pub last_fill_price: Option<Decimal>,
    pub client_id:  Option<usize>,
    pub why_held: Option<String>
}
#[derive(Debug,Clone)]
pub(crate) struct Execution {
    pub exec_id: Option<String>,
    pub time: Option<String>,
    pub acct_number: Option<String>,
    pub exchange: Option<String>,
    pub side: Option<Side>,
    pub shares: Option<Decimal>,
    pub price: Option<Decimal>,
    pub perm_id: Option<i32>,
    pub client_id: Option<usize>,
    pub order_id: i32,
    pub contract: Contract,
    pub liquidation: Option<i32>,
    pub cum_qty: Option<Decimal>,
    pub avg_price: Option<Decimal>,
    pub order_ref: Option<String>,
    pub ev_rule: Option<String>,
    pub ev_multiplier: Option<Decimal>,
    pub model_code: Option<String>,
    pub last_liquidity: Option<i32>
}
#[derive(Default,Debug,Clone)]
pub(crate) struct CommissionReport {
    pub exec_id: Option<String>,
    pub commission: Option<Decimal>,
    pub currency: Option<String>,
    pub realized_pnl: Option<Decimal>,
    pub yield_amount: Option<Decimal>,
    pub yield_redemption_date: Option<i32>
}
