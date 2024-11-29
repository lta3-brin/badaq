use polars::prelude::LazyFrame;

#[derive(Default)]
pub struct AppState {
    pub seq: String,
    pub nama: String,
    pub corr: LazyFrame,
    pub coef_calib: LazyFrame,
}
