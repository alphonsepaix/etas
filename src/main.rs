use clap::Parser;
use etas::{generate_sequence, write_to_file, Args};

fn main() {
    let args = Args::parse();
    match generate_sequence(&args) {
        Some(seq) => write_to_file(&seq, &args.filename, args.verbose),
        None => println!("The sequence was empty."),
    }
}
