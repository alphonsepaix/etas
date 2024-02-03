use std::fmt::Display;

use crate::constants::*;
use clap::Parser;
use eframe::egui;

pub enum AppError {
    Save(String),
    Simulation(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Save(why) => write!(f, "Error when saving: {why}"),
            AppError::Simulation(why) => {
                write!(f, "Error during simulation: {why}")
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
    pub no_gui: bool,

    #[arg(long)]
    pub seed: Option<u64>,
}

impl App {
    pub fn build() -> Result<Self, &'static str> {
        let args = App::parse();

        if args.p <= 1.0 {
            return Err("p must be > 1");
        }

        if args.alpha >= args.beta {
            return Err("alpha must be < beta");
        }

        Ok(args)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My app");
            ui.add(
                egui::Slider::new(&mut self.t_end, 0.0..=10000.0).text("T"),
            );
            if ui.button("Increment").clicked() {
                self.t_end += 100.0;
            }
        });
    }
}
