use eframe::egui;
use etas::app::App;
use etas::constants::*;
use etas::simulation::Sequence;
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
            sequence.save(path, verbose)
        }) {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    } else {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
            ..Default::default()
        };
        eframe::run_native("ETAS", options, Box::new(|_| Box::new(args)))
            .unwrap();
    }
}
