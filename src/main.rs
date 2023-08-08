use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 1.0)]
    mu: f32,

    #[arg(long, default_value_t = 2.0)]
    alpha: f32,

    #[arg(long, default_value_t = 0.9)]
    bar_n: f32,

    #[arg(long, default_value_t = 1.1)]
    p: f32,

    #[arg(long, default_value_t = 1e-9)]
    c: f32,

    #[arg(long, default_value_t = 10_f32.ln())]
    beta: f32,

    #[arg(long, default_value_t = 1e3)]
    t_end: f32,

    #[arg(long, default_value_t = String::from("data.csv"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    verbose: bool
}

struct Event {
    t: f32,
    m: f32,
    parent: i32
}

fn logrand() -> f32 {
    let mut rng = thread_rng();
    let x: f32 = rng.gen_range(0.0..1.0);
    x.ln()
}

fn etas(args: &Args) -> Option<Vec<Event>> {
    let a = args.bar_n
        / (args.beta * args.c.powf(1.0 - args.p))
        * (args.p - 1.0)
        * (args.beta - args.alpha);

    if a <= 0.0 {
        return None
    }

    let bar = ProgressBar::new(args.t_end as u64);
    let template = "[{elapsed_precise}] {bar:50.cyan/blue} \
{pos}/{len} -- {msg}";
    bar.set_style(ProgressStyle::with_template(template)
        .unwrap()
        .progress_chars("##-"));

    let mut tc = 0.0_f32;
    let mut seq: Vec<Event> = Vec::new();
    let mut m_max: f32 = 0.0;

    while tc < args.t_end {
        let dt = -1.0 / args.mu * logrand();
        tc += dt;
        if tc < args.t_end {
            let t = tc;
            let m = -1.0 / args.beta * logrand();
            m_max = if m_max > m { m_max } else { m };
            let parent = 0;
            seq.push(Event{ t, m, parent });
        }
    }

    if seq.len() == 0 {
        return None;
    }

    let mut n = 0;
    loop {
        tc = 0.0;

        if args.verbose {
            bar.set_position(seq[n].t as u64);
            bar.set_message(m_max.to_string());
        }

        loop {
            let tmp = (tc + args.c).powf(1.0 - args.p)
                + (args.p - 1.0)
                    / (a * (args.alpha * seq[n].m).exp()) * logrand();

            if tmp > 0.0 {
                let dt = tmp.powf(1.0 / (1.0 - args.p)) - tc - args.c;
                tc += dt;
                if tc + seq[n].t < args.t_end {
                    let t = tc + seq[n].t;
                    let m = -1.0 / args.beta * logrand();
                    m_max = if m_max > m { m_max } else { m };
                    let parent = n as i32;
                    seq.push(Event{ t, m, parent });
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        seq.sort_by(|e1, e2| e1.t.partial_cmp(&e2.t).unwrap());
        n += 1;

        if n == seq.len() {
            break;
        }
    }

    if args.verbose {
        bar.finish();
    }

    Some(seq)
}

fn write_to_file(seq: &Vec<Event>, filename: &String, verbose: bool) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    file.write(b"ID,TIME,MAGNITUDE,PARENT\n").unwrap();

    for i in 0..seq.len() {
        let e = &seq[i];
        file.write_fmt(format_args!("{},{},{},{}\n", i, e.t, e.m, e.parent))
            .unwrap();
    }

    if verbose {
        println!("{} events written to file '{}'.", seq.len(), display);
    }
}

fn main() {
    let args = Args::parse();
    match etas(&args) {
        Some(seq) => write_to_file(&seq, &args.filename, args.verbose),
        None => println!("The sequence was empty."),
    }
}
