pub mod app;
pub mod simulation;
pub mod ui;

pub mod constants {
    pub const TEMPLATE: &str =
        "[{elapsed_precise}] {bar:50.cyan/blue} {pos}/{len} -- {msg}";

    pub const WINDOW_TITLE: &str = "ETAS";
    pub const WINDOW_WIDTH: f32 = 330.0;
    pub const WINDOW_HEIGHT: f32 = 300.0;

    pub const DEFAULT_MU: f32 = 1.0;
    pub const DEFAULT_ALPHA: f32 = 2.0;
    pub const DEFAULT_BAR_N: f32 = 0.9;
    pub const DEFAULT_P: f32 = 1.1;
    pub const DEFAULT_C: f32 = 1e-9;
    pub const DEFAULT_EXP_BETA: f32 = 10.0;
    pub const DEFAULT_T_END: f32 = 1e3;
    pub const DEFAULT_MAX_LEN: usize = 10000;
    pub const DEFAULT_FILENAME: &str = "data.csv";
}
