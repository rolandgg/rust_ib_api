
use rust_decimal::prelude::*;
use tokio::sync::watch;

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

pub struct TickerSender {
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
    pub fn new() -> (TickerSender, Ticker) {
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
    pub fn midpoint(&self) -> Option<f64> {
        if let Some(bid) = &*self.bid.borrow() {
            if let Some(ask) = &*self.ask.borrow() {
                Some( (ask+bid) / 2.0)
            }
            else {None}
        }
        else {None}
    }

    pub fn bid(&self) -> Option<f64> {
        if let Some(bid) = &*self.bid.borrow() {
            Some(bid.clone())
        }
        else {None}
    }

    pub fn ask(&self) -> Option<f64> {
        if let Some(ask) = &*self.ask.borrow() {
            Some(ask.clone())
        }
        else {None}
    }

    pub fn shortable_shares(&self) -> Option<i32> {
        if let Some(shares) = &*self.shortable_shares.borrow() {
            Some(shares.clone())
        }
        else {None}
    }

    pub fn short_availability(&self) -> Option<ShortAvailability> {
        if let Some(avail) = &*self.short_availability.borrow() {
            Some(avail.clone())
        }
        else {None}
    }

}
