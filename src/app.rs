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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct App {
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

impl App {
    pub fn build() -> AppResult<Self> {
        let args = App::parse();
        // args.validate()?;
        Ok(args)
    }

    // Validate arguments
    pub fn validate(&self) -> AppResult<()> {
        if self.p <= 1.0 {
            return Err(AppError::InvalidArgument("p must be > 1".to_owned()));
        }

        if self.alpha >= self.beta {
            return Err(AppError::InvalidArgument(
                "alpha must be < beta".to_owned(),
            ));
        }

        // Check other arguments
        todo!()
    }
}
