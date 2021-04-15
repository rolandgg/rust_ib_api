
#[derive(Debug,Clone)]
pub struct Bar {
    pub t_stamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub wap: f64,
    pub volume: i64,
    pub count: isize
}
#[derive(Debug,Clone)]
pub struct BarSeries {
    pub start_dt: String,
    pub end_dt: String,
    pub n_bars: usize,
    pub data: Option<Vec<Bar>>
}

