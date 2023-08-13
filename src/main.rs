use etas::{generate_sequence, write_to_file, Args};
use std::process;

fn main() {
    let args = match Args::build() {
        Ok(args) => args,
        Err(why) => {
            eprintln!("Error: {why}");
            process::exit(1);
        }
    };

    match generate_sequence(&args) {
        Some(seq) => write_to_file(&seq, &args.filename, args.verbose),
        None => println!("The sequence was empty."),
    }
}
