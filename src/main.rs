use eframe::egui::{self, FontData, FontDefinitions};
use etas::app::App;
use etas::constants::*;
use etas::simulation::Sequence;
use etas::ui::WidgetGallery;
use std::path::Path;
use std::process;

fn main() {
    let args = match App::build() {
        Ok(args) => args,
        Err(why) => {
            eprintln!("Error: {why}");
            process::exit(1);
        }
    };

    if args.no_gui {
        if let Err(e) = Sequence::generate(&args).map(|sequence| {
            let path = Path::new(&args.filename);
            let verbose = args.verbose;
            sequence.save(path, verbose, args.headers)
        }) {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    } else {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size((WINDOW_WIDTH, WINDOW_HEIGHT))
                .with_resizable(false),
            follow_system_theme: true,
            ..Default::default()
        };
        eframe::run_native(
            WINDOW_TITLE,
            options,
            Box::new(|ctx| {
                let mut fonts = FontDefinitions::default();

                fonts.font_data.insert(
                    "jetbrains".to_owned(),
                    FontData::from_static(include_bytes!(
                        "../fonts/JetBrainsMono.ttf"
                    )),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "jetbrains".to_owned());

                fonts.font_data.insert(
                    "ubuntu".to_owned(),
                    FontData::from_static(include_bytes!(
                        "../fonts/Ubuntu.ttf"
                    )),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "ubuntu".to_owned());

                ctx.egui_ctx.set_fonts(fonts);
                Box::new(WidgetGallery::build(args))
            }),
        )
        .unwrap();
    }
}
