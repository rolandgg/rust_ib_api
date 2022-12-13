
use rust_decimal::prelude::*;
use tokio::sync::watch;


/// Short availability as classified by the TWS API.
#[derive(Clone)]
pub enum ShortAvailability {
    Available, HardToBorrow, Unavailable
}

impl ShortAvailability {
    pub fn from_f64(val: f64) -> Self {
        if val > 2.5 {Self::Available}
        else if val > 1.5 {Self::HardToBorrow}
        else {Self::Unavailable}
    }
}

use std::fmt;
impl fmt::Display for ShortAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShortAvailability::Available => write!(f, "Available"),
            ShortAvailability::HardToBorrow => write!(f, "HardToBorrow"),
            ShortAvailability::Unavailable => write!(f, "Unavailable")
        }
    }
}

/// The `Ticker` is returned after a successful request for market data and receives streaming market data for the
/// requested contract.
pub struct Ticker {
    bid: watch::Receiver<Option<f64>>,
    bid_size: watch::Receiver<Option<i32>>,
    ask: watch::Receiver<Option<f64>>,
    ask_size: watch::Receiver<Option<i32>>,
    last: watch::Receiver<Option<f64>>,
    last_size: watch::Receiver<Option<i32>>,
    shortable_shares: watch::Receiver<Option<i32>>,
    short_availability: watch::Receiver<Option<ShortAvailability>>
}

pub(crate) struct TickerSender {
    pub bid: watch::Sender<Option<f64>>,
    pub bid_size: watch::Sender<Option<i32>>,
    pub ask: watch::Sender<Option<f64>>,
    pub ask_size: watch::Sender<Option<i32>>,
    pub last: watch::Sender<Option<f64>>,
    pub last_size: watch::Sender<Option<i32>>,
    pub shortable_shares: watch::Sender<Option<i32>>,
    pub short_availability: watch::Sender<Option<ShortAvailability>>
}

impl Ticker {
    pub(crate) fn new() -> (TickerSender, Ticker) {
        let (bid_tx, bid_rx) = watch::channel(None);
        let (bid_size_tx, bid_size_rx) = watch::channel(None);
        let (ask_tx, ask_rx) = watch::channel(None);
        let (ask_size_tx, ask_size_rx) = watch::channel(None);
        let (last_tx, last_rx) = watch::channel(None);
        let (last_size_tx, last_size_rx) = watch::channel(None);
        let (short_s_tx, short_s_rx) = watch::channel(None);
        let (short_a_tx, short_a_rx) = watch::channel(None);

        (
            TickerSender {
                bid: bid_tx,
                bid_size: bid_size_tx,
                ask: ask_tx,
                ask_size: ask_size_tx,
                last: last_tx,
                last_size: last_size_tx,
                shortable_shares: short_s_tx,
                short_availability: short_a_tx
            },
            Ticker {
                bid: bid_rx,
                bid_size: bid_size_rx,
                ask: ask_rx,
                ask_size: ask_size_rx,
                last: last_rx,
                last_size: last_size_rx,
                shortable_shares: short_s_rx,
                short_availability: short_a_rx
            }
        )
    }
    /// Returns the latest midpoint price, if any.
    pub fn midpoint(&self) -> Option<f64> {
        if let Some(bid) = &*self.bid.borrow() {
            if let Some(ask) = &*self.ask.borrow() {
                Some( (ask+bid) / 2.0)
            }
            else {None}
        }
        else {None}
    }
    /// Returns the latest bid price if a bid was received.
    pub fn bid(&self) -> Option<f64> {
        self.bid.borrow().clone()
    }
    /// Returns the latest ask price if an ask price was received.
    pub fn ask(&self) -> Option<f64> {
        self.ask.borrow().clone()
    }
    /// Returns the latest bid size if received.
    pub fn bid_size(&self) -> Option<i32> {
        self.bid_size.borrow().clone()
    }
    /// Returns the latest ask size if received.
    pub fn ask_size(&self) -> Option<i32> {
        self.ask_size.borrow().clone()
    }
    /// Returns the number of shares available to sell short.
    pub fn shortable_shares(&self) -> Option<i32> {
        self.shortable_shares.borrow().clone()
    }
    /// Returns the current short availability.
    pub fn short_availability(&self) -> Option<ShortAvailability> {
        self.short_availability.borrow().clone()
    }

}
