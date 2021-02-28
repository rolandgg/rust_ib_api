use crate::ib_enums::*;
use rust_decimal::prelude::*;
use crate::utils::ib_message::Encodable;
use crate::ib_contract::Contract;
use crossbeam::channel;
use tokio::sync::watch;
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

pub struct OrderTrackerSender {
    pub executions_tx: channel::Sender<Execution>,
    pub order_tx: watch::Sender<Order>,
    pub order_status_tx: watch::Sender<Option<OrderStatus>>,
    pub order_state_tx: watch::Sender<OrderState>,
    pub commission_reports_tx: channel::Sender<CommissionReport>
}

impl OrderTracker {
    pub fn new(order: Order, order_state: OrderState) -> (OrderTrackerSender, Self) {
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

    pub fn status(& mut self) -> Option<String> {
        match &*self.order_status_rx.borrow() {
            Some(stat) => Some(stat.status.clone()),
            None => None
        }
    }

    pub fn avg_fill_price(&mut self) -> Option<Decimal> {
        match &*self.order_status_rx.borrow() {
            Some(stat) => Some(stat.avg_fill_price.clone()),
            None => None
        }
    }

    pub fn commissions_paid(&mut self) -> Option<Decimal> {
        self.update_com();
        if self.commission_reports.len() > 0 {
            Some(self.commission_reports.iter().fold(Decimal::new(0, 2), |acc, x| acc + x.commission))
        }
        else {
            None
        }
    }
}

pub struct SoftDollarTier {
    pub name: Option<String>,
    pub val: Option<String>,
    pub display_name: Option<String>
}

#[derive(Default)]
pub struct Order {

    //contract
    pub contract: Contract,

    //order identification
    pub order_id: usize,
    pub client_id: usize,
    pub perm_id: i32,

    //main order fields
    pub action: Action,
    pub total_qty: Decimal,
    pub order_type: OrderType,
    pub lmt_price: Option<Decimal>,
    pub aux_price: Option<Decimal>,

    //extended order fields
    pub tif: Option<TimeInForce>,
    pub active_start_time: Option<String>,
    pub active_stop_time: Option<String>,
    pub oca_group: Option<String>,
    pub oca_type: Option<OCAType>,
    pub order_ref: Option<String>,
    pub transmit: bool,
    pub parent_id: usize,
    pub block_order: bool,
    pub sweep_to_fill: bool,
    pub display_size: i32,
    pub trigger_method: Option<TriggerMethod>,
    pub outside_rth: bool,
    pub hidden: bool,
    pub good_after_time: Option<String>,
    pub good_till_date: Option<String>,
    pub override_percentage_constraints: bool,
    pub rule_80A: Option<Rule80A>,
    pub all_or_none: bool,
    pub min_qty: Option<i32>,
    pub percent_offset: Option<f64>,
    pub trail_stop_price: Option<Decimal>,
    pub trailing_percent: Option<f64>,

    // financial advisor fields
    pub fa_group: Option<String>,
    pub fa_profile: Option<String>,
    pub fa_method: Option<String>,
    pub fa_percentage: Option<String>,

    // institutional (i.e. non-cleared) only
    pub open_close: Option<OrderOpenClose>,
    pub origin: Option<Origin>,
    pub short_sale_slot: Option<ShortSaleSlot>,
    pub designated_location: Option<String>,
    pub exempt_code: i32,

    // SMART routing fields
    pub discretionary_amt: f64,
    pub e_trade_only: bool,
    pub firm_quote_only: bool,
    pub nbbo_price_cap: Option<Decimal>,
    pub opt_out_smart_routing: bool,

    // BOX exchange order fields
    pub auction_strategy: Option<AuctionStrategy>,
    pub starting_price: Option<Decimal>,
    pub stock_ref_price: Option<Decimal>,
    pub delta: Option<f64>,

    // Pegged to stock and VOL order fields

    pub stock_range_lower: Option<Decimal>,
    pub stock_range_upper: Option<Decimal>,

    pub randomize_size: bool,
    pub randomize_price: bool,

    // Volatility order fields
    pub volatility: Option<f64>,
    pub volatility_type: Option<VolatilityType>,
    pub delta_neutral_order_type: Option<OrderType>,
    pub delta_neutral_aux_price: Option<Decimal>,
    pub delta_neutral_con_id: usize,
    pub delta_neutral_settling_firm: Option<String>,
    pub delta_neutral_clearing_account: Option<String>,
    pub delta_neutral_clearing_intent: Option<String>,
    pub delta_neutral_open_close: Option<String>,
    pub delta_neutral_short_sale: bool,
    pub delta_neutral_short_sale_slot: bool,
    pub delta_neutral_designated_location: Option<String>,
    pub continuous_update: bool,
    pub reference_price_type: Option<ReferencePriceType>,

    // Combo order fields
    pub basis_points: Option<Decimal>,
    pub basis_points_type: Option<BasisPointsType>,

    // Scale order fields
    pub scale_init_level_size: Option<i32>,
    pub scale_subs_level_size: Option<i32>,
    pub scale_price_increment: Option<f64>,
    pub scale_price_adjust_value: Option<f64>,
    pub scale_price_adjust_interval: Option<i32>,
    pub scale_profit_offset: Option<f64>,
    pub scale_auto_reset: bool,
    pub scale_init_position: Option<i32>,
    pub scale_init_fill_qty: Option<i32>,
    pub scale_random_percent: bool,
    pub scale_table: Option<String>,

    // Hedge order fields
    pub hedge_type: Option<HedgeType>,
    pub hedge_param: Option<String>, // 'beta=X' value for beta hedge, 'ratio=Y' for pair hedge

    // Clearing info
    pub account: Option<String>,
    pub settling_firm: Option<String>,
    pub clearing_account: Option<String>,
    pub clearing_intent: Option<ClearingIntent>,

    // Algo order fields
    pub algo_strategy: Option<String>,
    pub algo_params: Option<Vec<(String,String)>>,
    pub smart_combo_routing_params: Option<Vec<(String,String)>>,
    pub algo_id: Option<String>,

    // What-if
    pub what_if: bool,

    // Not held
    pub not_held: bool,
    pub solicited: bool,

    // Models
    pub model_code: Option<String>,

    // Order combo legs

    pub order_combo_legs: Option<Vec<Option<Decimal>>>,
    pub order_misc_options: Option<Vec<(String,String)>>,

    // VER PEG2BENCH fields

    pub reference_contract_id: i32,
    pub pegged_change_amount: f64,
    pub is_pegged_change_amount_decrease: bool,
    pub reference_change_amount: f64,
    pub reference_exchange_id: Option<String>,
    pub adjusted_order_type: Option<String>,
    pub trigger_price: Option<f64>,
    pub adjusted_stop_price: Option<f64>,
    pub adjusted_stop_limit_price: Option<f64>,
    pub adjusted_trailing_amount: Option<f64>,
    pub adjustable_trailing_unit: i32,
    pub lmt_price_offset: Option<f64>,

    pub conditions: Option<Vec<OrderConditionType>>,
    pub conditions_cancel_order: bool,
    pub conditions_ignore_rth: bool,

    // ext operator
    pub ext_operator: Option<String>,

    pub soft_dollar_tier: Option<SoftDollarTier>,

    pub cash_qty: Option<Decimal>,

    pub mifid_2_decision_maker: Option<String>,
    pub mifid_2_decision_algo: Option<String>,
    pub mifid_2_execution_trader: Option<String>,
    pub mifid_2_execution_algo: Option<String>,

    pub dont_use_auto_price_for_hedge: bool,
    pub is_oms_container: bool,
    pub discretionary_up_to_limit_price: bool,
    pub auto_cancel_date: Option<String>,
    pub filled_quantity: Option<Decimal>,
    pub ref_futures_con_id: Option<usize>,
    pub auto_cancel_parent: bool,
    pub shareholder: Option<String>,
    pub imbalance_only: bool,
    pub route_marketable_to_bbo: bool,
    pub parent_perm_id: Option<usize>,
    pub use_price_mgmt_algo: Option<UsePriceMgmtAlgo>
}

impl Order {
    fn new() -> Self {
        Order {
            transmit: true,
            open_close: Some(OrderOpenClose::Open),
            origin: Some(Origin::Customer),
            exempt_code: -1,
            e_trade_only: true,
            firm_quote_only: true,
            auction_strategy: Some(AuctionStrategy::NoAuctionStrategy),
            ..Default::default()
        }
    }

    pub fn market(contract: Contract, action: Action, qty: Decimal) -> Self {
        let mut order = Order::new();
        order.action = action;
        order.contract = contract;
        order.total_qty = qty;
        order
    }

    pub fn market_on_close(contract: Contract, action: Action, qty: Decimal) -> Self {
        let mut order = Order::new();
        order.action = action;
        order.contract = contract;
        order.total_qty = qty;
        order.order_type = OrderType::MarketOnClose;
        order
    }

    pub fn limit(contract: Contract, action: Action, qty: Decimal, lmt: Decimal, tif: TimeInForce) -> Self {
        let mut order = Order::new();
        order.action = action;
        order.contract = contract;
        order.total_qty = qty;
        order.order_type = OrderType::Limit;
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
        if let Some(sec) = self.contract.sec_type {
            if sec == SecType::Combo {
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
        match self.contract.delta_neutral_contract {
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
            match self.algo_params {
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

        if self.order_type == OrderType::PeggedToBenchmark {
            code.push_str(&self.reference_contract_id.encode());
            code.push_str(&self.is_pegged_change_amount_decrease.encode());
            code.push_str(&self.pegged_change_amount.encode());
            code.push_str(&self.reference_change_amount.encode());
            code.push_str(&self.reference_exchange_id.encode());
        }

        match self.conditions {
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
        match self.soft_dollar_tier {
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

        code.push_str(&self.is_oms_container.encode());
        code.push_str(&self.discretionary_up_to_limit_price.encode());
        code.push_str(&self.use_price_mgmt_algo.encode());
        code
    }
}  

#[derive(Default)]
pub struct OrderState {
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

pub struct OrderStatus {
    pub order_id: usize,
    pub status: String,
    pub filled: Decimal,
    pub remaining: Decimal,
    pub avg_fill_price: Decimal,
    pub perm_id: i32,
    pub parent_id: usize,
    pub last_fill_price: Decimal,
    pub client_id:  usize,
    pub why_held: Option<String>
}

pub struct Execution {
    pub exec_id: String,
    pub time: String,
    pub acct_number: String,
    pub exchange: String,
    pub side: Side,
    pub shares: Decimal,
    pub price: Decimal,
    pub perm_id: i32,
    pub client_id: usize,
    pub order_id: usize,
    pub contract: Contract,
    pub liquidation: i32,
    pub cum_qty: Decimal,
    pub avg_price: Decimal,
    pub order_ref: Option<String>,
    pub ev_rule: Option<String>,
    pub ev_multiplier: Option<Decimal>,
    pub model_code: Option<String>,
    pub last_liquidity: Option<i32>
}

pub struct CommissionReport {
    pub exec_id: String,
    pub commission: Decimal,
    pub currency: String,
    pub realized_pnl: Option<Decimal>,
    pub yield_amount: Option<Decimal>,
    pub yield_redemption_date: Option<i32>
}
