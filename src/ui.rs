use std::path::Path;

use crate::{app::App, simulation::Sequence};
use eframe::egui;

pub struct WidgetGallery {
    app: App,
    sequence: Option<Sequence>,
}

impl WidgetGallery {
    pub fn build(app: App) -> Self {
        Self {
            app,
            sequence: None,
        }
    }

    fn save_sequence(&mut self, clear: bool) {
        if let Some(seq) = &self.sequence {
            let path = Path::new(&self.app.filename);
            seq.save(path, self.app.verbose, self.app.headers)
                .map(|_| {
                    if clear {
                        self.sequence = None;
                    }
                })
                .expect("Could not save sequence on disk");
        }
    }
}

impl eframe::App for WidgetGallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Simulation parameters");
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    egui::Grid::new("Simulation").num_columns(4).show(
                        ui,
                        |ui| {
                            ui.label("μ");
                            ui.add(
                                egui::DragValue::new(&mut self.app.mu)
                                    .speed(0.5),
                            );
                            ui.label("p");
                            ui.add(
                                egui::DragValue::new(&mut self.app.p)
                                    .speed(0.1)
                                    .clamp_range(1.000001..=f32::MAX),
                            );
                            ui.end_row();

                            ui.label("α");
                            ui.add(
                                egui::DragValue::new(&mut self.app.alpha)
                                    .speed(0.5),
                            );
                            ui.label("c");
                            ui.add(
                                egui::DragValue::new(&mut self.app.c)
                                    .speed(0.1)
                                    .clamp_range(1.000001..=f32::MAX),
                            );
                            ui.end_row();

                            ui.label("n̄");
                            ui.add(
                                egui::DragValue::new(&mut self.app.bar_n)
                                    .speed(0.1)
                                    .clamp_range(1.000001..=f32::MAX),
                            );
                            ui.label("β");
                            ui.add(
                                egui::DragValue::new(&mut self.app.beta)
                                    .speed(0.1)
                                    .clamp_range(1.000001..=f32::MAX),
                            );
                            ui.end_row();
                        },
                    );
                });

                ui.group(|ui| {
                    egui::Grid::new("Duration").num_columns(2).show(
                        ui,
                        |ui| {
                            ui.label("T");
                            ui.add(
                                egui::DragValue::new(&mut self.app.t_end)
                                    .speed(1000.0)
                                    .clamp_range(0.0..=f32::MAX),
                            )
                            .on_hover_text("Interval size");
                            ui.end_row();

                            ui.label("N");
                            ui.add(
                                egui::DragValue::new(&mut self.app.p)
                                    .speed(0.1)
                                    .clamp_range(1.000001..=f32::MAX),
                            )
                            .on_hover_text("Sequence size limit");
                            ui.end_row();
                        },
                    );
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    self.sequence = Sequence::generate(&self.app).ok();
                }
                if ui.button("Clear").clicked() {
                    self.sequence = None;
                }
            });

            let n = self.sequence.as_ref().map(|s| s.len()).unwrap_or(0);
            let label = if n > 0 {
                format!(
                    "{} event{} in memory",
                    n,
                    if n > 1 { 's' } else { '\0' }
                )
            } else {
                "No events in memory".to_string()
            };

            ui.separator();

            ui.label(label);
            ui.horizontal(|ui| {
                ui.label("Filename: ");
                ui.text_edit_singleline(&mut self.app.filename);
            });

            ui.checkbox(&mut self.app.headers, "CSV headers");

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.save_sequence(false);
                }
                if ui.button("Save and clear").clicked() {
                    self.save_sequence(true);
                }
            });

            ui.separator();

            ui.vertical_centered(|ui| {
                ui.hyperlink_to(
                    "Created with egui",
                    "https://github.com/emilk/egui",
                );

                ui.hyperlink_to(
                    format!("{} Repository", egui::special_emojis::GITHUB),
                    "https://github.com/alphonsepaix/etas/",
                );
            })
        });
    }
}
