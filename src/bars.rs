
#[derive(Debug,Clone)]
pub struct Bar {
    pub t_stamp: Option<String>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub wap: Option<f64>,
    pub volume: Option<i64>,
    pub count: Option<isize>
}
#[derive(Debug,Clone)]
pub struct BarSeries {
    pub start_dt: Option<String>,
    pub end_dt: Option<String>,
    pub n_bars: Option<usize>,
    pub data: Option<Vec<Bar>>
}

