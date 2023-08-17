use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::thread_rng;
use rand_distr::{Distribution, Exp, Poisson, Uniform};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Parser)]
#[command(about = "Programme de simulation pour le modèle ETAS")]
pub struct Args {
    #[arg(long, default_value_t = 1.0)]
    pub mu: f32,

    #[arg(long, default_value_t = 2.0)]
    pub alpha: f32,

    #[arg(long, default_value_t = 0.9)]
    pub bar_n: f32,

    #[arg(long, default_value_t = 1.1)]
    pub p: f32,

    #[arg(long, default_value_t = 1e-9)]
    pub c: f32,

    #[arg(long, default_value_t = 10_f32.ln())]
    pub beta: f32,

    #[arg(long, default_value_t = 1e3)]
    pub t_end: f32,

    #[arg(long)]
    pub max_len: Option<usize>,

    #[arg(long, default_value_t = String::from("data.csv"))]
    pub filename: String,

    #[arg(long, default_value_t = false)]
    pub verbose: bool,
}

impl Args {
    pub fn build() -> Result<Self, &'static str> {
        let args = Args::parse();

        if args.p <= 1.0 {
            return Err("p must be > 1");
        }

        if args.alpha >= args.beta {
            return Err("alpha must be < beta");
        }

        Ok(args)
    }
}

pub struct Event {
    pub t: f32,
    pub m: f32,
    pub parent: i32,
}

pub fn generate_sequence(args: &Args) -> Option<Vec<Event>> {
    let mut rng = thread_rng();
    let num_events = Poisson::new(args.mu * args.t_end).unwrap().sample(&mut rng) as usize;

    if num_events == 0 {
        return None;
    }

    let a = args.bar_n / (args.beta * args.c.powf(1.0 - args.p))
        * (args.p - 1.0)
        * (args.beta - args.alpha);
    let exp = Exp::<f32>::new(args.beta).unwrap();
    let uniform = Uniform::<f32>::from(0.0..1.0);

    let bg_t: Vec<f32> = Uniform::from(0.0..args.t_end)
        .sample_iter(&mut rng)
        .take(num_events)
        .collect();
    let bg_m: Vec<f32> = exp.sample_iter(&mut rng).take(num_events).collect();
    let mut seq: Vec<Event> = Vec::new();
    for (t, m) in bg_t.iter().zip(&bg_m) {
        seq.push(Event {
            t: *t,
            m: *m,
            parent: 0,
        });
    }
    seq.sort_by(|e1, e2| e1.t.partial_cmp(&e2.t).unwrap());

    let bar = ProgressBar::new(args.t_end as u64);
    let template = "[{elapsed_precise}] {bar:50.cyan/blue} \
{pos}/{len} -- {msg}";
    bar.set_style(
        ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars("##-"),
    );
    let mut m_max = bg_m
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .cloned()
        .unwrap();
    let mut n = 0;
    let mut simulation_ended = false;

    loop {
        let mut tc = 0.0;

        if args.verbose {
            bar.set_position(seq[n].t as u64);
            bar.set_message(m_max.to_string());
        }

        loop {
            let tmp = (tc + args.c).powf(1.0 - args.p)
                + (args.p - 1.0) / (a * (args.alpha * seq[n].m).exp())
                    * uniform.sample(&mut rng).ln();

            if tmp > 0.0 {
                let dt = tmp.powf(1.0 / (1.0 - args.p)) - tc - args.c;
                tc += dt;
                if tc + seq[n].t < args.t_end {
                    let t = tc + seq[n].t;
                    let m = exp.sample(&mut rng);
                    m_max = if m_max > m { m_max } else { m };
                    let parent = n as i32;
                    seq.push(Event { t, m, parent });
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        seq.sort_by(|e1, e2| e1.t.partial_cmp(&e2.t).unwrap());

        n += 1;

        if let Some(max_len) = args.max_len {
            if max_len == n - 1 {
                seq = seq.into_iter().take(max_len).collect();
                break;
            }
        }

        if n == seq.len() {
            simulation_ended = true;
            break;
        }
    }

    if args.verbose {
        if simulation_ended {
            bar.finish();
        } else {
            bar.abandon();
        }
    }

    if seq.is_empty() {
        return None;
    }
    Some(seq)
}

pub fn write_to_file(seq: &Vec<Event>, filename: &String, verbose: bool) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = BufWriter::new(File::create(path).unwrap());

    file.write_all(b"id,time,magnitude,parent\n").unwrap();

    for (i, _) in seq.iter().enumerate() {
        let e = &seq[i];
        file.write_fmt(format_args!("{},{},{},{}\n", i, e.t, e.m, e.parent))
            .unwrap();
    }

    let length = seq.len();
    if verbose {
        println!(
            "{} event{} written to file '{}'.",
            length,
            if length == 1 { '\0' } else { 's' },
            display
        );
    }
}
