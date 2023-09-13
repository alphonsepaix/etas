use etas::{generate_sequence, write_to_file, Args};
use std::path::Path;
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
        Ok(result) => match result {
            Some(seq) => {
                let path = Path::new(&args.filename);
                match write_to_file(&seq, path) {
                    Ok(_) => {
                        let length = seq.len();
                        if args.verbose {
                            println!(
                                "{} event{} written to file '{}'.",
                                length,
                                if length == 1 { '\0' } else { 's' },
                                path.display()
                            );
                        }
                    }
                    Err(why) => {
                        eprintln!("Error: {why}");
                        process::exit(1);
                    }
                }
            }
            None => println!("The sequence was empty."),
        },
        Err(why) => {
            eprintln!("Error: {why}");
            process::exit(1);
        }
    }
}
