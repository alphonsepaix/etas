use etas::{generate_sequence, write_to_file, Args};
use std::process;

fn main() {
    // Try to parse the command line arguments
    let args = match Args::build() {
        Ok(args) => args,
        Err(why) => {
            eprintln!("Error: {why}");
            process::exit(1);
        }
    };

    // Launch a simulation with the provided arguments
    match generate_sequence(&args) {
        Some(seq) => write_to_file(&seq, &args.filename, args.verbose),
        None => println!("The sequence was empty."),
    }
}
