use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::{app::Args, simulation::Sequence};
use eframe::egui;

pub struct WidgetGallery {
    args: Args,
    sequence: Arc<Mutex<Option<Sequence>>>,
}

impl WidgetGallery {
    pub fn build(app: Args) -> Self {
        Self {
            args: app,
            sequence: Arc::new(Mutex::new(None)),
        }
    }

    fn save_sequence(&mut self, clear: bool) {
        if let Some(seq) = self.sequence.lock().unwrap().as_ref() {
            let path = Path::new(&self.args.filename);
            seq.save(path, self.args.verbose, self.args.headers)
                .map(|_| {
                    if clear {
                        *self.sequence.lock().unwrap() = None;
                    }
                })
                .expect("Could not save sequence on disk");
        }
    }
}

impl eframe::App for WidgetGallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Epidemic-Type Aftershock-Sequence\nmodel");
            });

            ui.add_space(10.0);
            ui.separator();

            ui.label("Simulation parameters");
            ui.add_space(5.0);
            egui::Grid::new("param_grid_1")
                .num_columns(4)
                .show(ui, |ui| {
                    ui.label("μ");
                    ui.add(
                        egui::DragValue::new(&mut self.args.mu)
                            .speed(0.1)
                            .clamp_range(0.0..=f32::MAX),
                    );
                    ui.label("p");
                    ui.add(
                        egui::DragValue::new(&mut self.args.p)
                            .speed(0.1)
                            .clamp_range(1.000001..=f32::MAX),
                    );
                    ui.end_row();

                    ui.label("α");
                    ui.add(
                        egui::DragValue::new(&mut self.args.alpha)
                            .speed(0.1)
                            .clamp_range(0.0..=self.args.beta),
                    );
                    ui.label("c");
                    ui.add(egui::DragValue::new(&mut self.args.c).speed(0.1));
                    ui.end_row();

                    ui.label("n̄");
                    ui.add(
                        egui::DragValue::new(&mut self.args.bar_n)
                            .speed(0.1)
                            .clamp_range(0.0..=1.0),
                    );
                    ui.label("β");
                    ui.add(
                        egui::DragValue::new(&mut self.args.beta)
                            .speed(0.1)
                            .clamp_range(0.0..=f32::MAX),
                    );
                    ui.end_row();
                });

            ui.add_space(5.0);

            egui::Grid::new("param_grid_2")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("T").on_hover_text("Interval size");
                    ui.add(
                        egui::DragValue::new(&mut self.args.t_end)
                            .speed(100.0)
                            .clamp_range(0.0..=f32::MAX),
                    );
                    ui.end_row();

                    ui.label("N").on_hover_text("Limit the number of events");
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(
                                self.args.max_len.is_none(),
                                "None",
                            )
                            .clicked()
                        {
                            self.args.max_len = None;
                        }
                        if ui
                            .selectable_label(
                                self.args.max_len.is_some(),
                                "Set",
                            )
                            .clicked()
                        {
                            self.args.max_len = Some(1000);
                        }

                        if let Some(max_len) = self.args.max_len.as_mut() {
                            ui.add(
                                egui::DragValue::new(max_len)
                                    .speed(200)
                                    .clamp_range(0..=usize::MAX),
                            );
                        }
                    });
                });

            ui.add_space(5.0);
            if ui.button("Reset").clicked() {
                self.args = Args::default();
            }

            ui.separator();

            let n = self.sequence.lock().unwrap().as_ref().map(|s| s.len()).unwrap_or(0);
            let label = if n > 0 {
                format!(
                    "{} event{} in memory",
                    n,
                    if n > 1 { 's' } else { '\0' }
                )
            } else {
                "No events in memory".to_string()
            };
            ui.label(label);

            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    thread::spawn({
                        let sequence = self.sequence.clone();
                        let args = self.args.clone();
                        move || {
                            *sequence.lock().unwrap() = Sequence::generate(&args).ok();
                        }
                    });
                }
                if ui.button("Clear").clicked() {
                    *self.sequence.lock().unwrap() = None;
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Filename: ");
                ui.text_edit_singleline(&mut self.args.filename);
            });

            ui.checkbox(&mut self.args.headers, "CSV headers");

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.save_sequence(false);
                }
                if ui.button("Save and clear").clicked() {
                    self.save_sequence(true);
                }
            });

            ui.separator();
            ui.add_space(5.0);

            ui.vertical_centered(|ui| {
                ui.hyperlink_to(
                    "Created with egui",
                    "https://github.com/emilk/egui",
                );

                ui.add_space(3.0);

                ui.hyperlink_to(
                    format!(
                        "{} See the GitHub repository",
                        egui::special_emojis::GITHUB
                    ),
                    "https://github.com/alphonsepaix/etas",
                );
            })
        });
    }
}
