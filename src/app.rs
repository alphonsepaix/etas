use std::fmt::Display;

use crate::constants::*;
use clap::Parser;

#[derive(Debug)]
pub enum AppError {
    Save(String),
    Simulation(String),
    InvalidArgument(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Save(why) => write!(f, "Error when saving: {why}"),
            AppError::Simulation(why) => {
                write!(f, "Error during simulation: {why}")
            }
            AppError::InvalidArgument(why) => {
                write!(f, "Wrong argument: {why}")
            }
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = DEFAULT_MU)]
    pub mu: f32,

    #[arg(long, default_value_t = DEFAULT_ALPHA)]
    pub alpha: f32,

    #[arg(long, default_value_t = DEFAULT_BAR_N)]
    pub bar_n: f32,

    #[arg(long, default_value_t = DEFAULT_P)]
    pub p: f32,

    #[arg(long, default_value_t = DEFAULT_C)]
    pub c: f32,

    #[arg(long, default_value_t = f32::ln(DEFAULT_EXP_BETA))]
    pub beta: f32,

    #[arg(long, default_value_t = DEFAULT_T_END)]
    pub t_end: f32,

    #[arg(long)]
    pub max_len: Option<usize>,

    #[arg(long, default_value_t = String::from(DEFAULT_FILENAME))]
    pub filename: String,

    #[arg(long)]
    pub verbose: bool,

    #[arg(long)]
    pub headers: bool,

    #[arg(long)]
    pub no_gui: bool,

    #[arg(long)]
    pub seed: Option<u64>,
}

impl Args {
    pub fn build() -> AppResult<Self> {
        let args = Args::parse();
        args.check_arguments()?;
        Ok(args)
    }

    pub fn check_arguments(&self) -> AppResult<()> {
        if self.beta < 0.0 {
            return Err(AppError::InvalidArgument(
                "beta must be >= 0".to_owned(),
            ));
        }
        if self.mu < 0.0 {
            return Err(AppError::InvalidArgument(
                "mu must be >= 0".to_owned(),
            ));
        }
        if self.p <= 1.0 {
            return Err(AppError::InvalidArgument("p must be > 1".to_owned()));
        }
        if self.bar_n >= 1.0 {
            return Err(AppError::InvalidArgument(
                "bar_n must be < 1".to_owned(),
            ));
        }
        if self.t_end <= 0.0 {
            return Err(AppError::InvalidArgument(
                "t_end must be > 0".to_owned(),
            ));
        }
        if self.alpha >= self.beta {
            return Err(AppError::InvalidArgument(
                "alpha must be < beta".to_owned(),
            ));
        }
        Ok(())
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            mu: DEFAULT_MU,
            alpha: DEFAULT_ALPHA,
            bar_n: DEFAULT_BAR_N,
            p: DEFAULT_P,
            c: DEFAULT_C,
            beta: f32::ln(DEFAULT_EXP_BETA),
            t_end: DEFAULT_T_END,
            max_len: None,
            filename: DEFAULT_FILENAME.to_owned(),
            verbose: false,
            headers: false,
            no_gui: false,
            seed: None,
        }
    }
}
