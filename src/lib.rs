use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Exp, Poisson, Uniform};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub type CResult<T> = Result<T, Box<dyn Error>>;

/// A structure used to parse command line arguments
/// which holds the parameters needed to complete a simulation
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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

    /// The end of the interval
    #[arg(long, default_value_t = 1e3)]
    pub t_end: f32,

    /// The maximum number of elements in the generated sequence
    #[arg(long)]
    pub max_len: Option<usize>,

    /// The output filename
    #[arg(long, default_value_t = String::from("data.csv"))]
    pub filename: String,

    /// Display a progress bar during simulation
    #[arg(long, default_value_t = false)]
    pub verbose: bool,

    /// Create the PRNG using the given seed
    #[arg(long)]
    pub seed: Option<u64>,
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

/// A structure that represents a single seismic event
pub struct Event {
    /// The arrival time of the event
    pub t: f32,
    /// The corresponding magnitude
    pub m: f32,
    /// The ID of the event that triggered this one
    pub parent: usize,
}

/// Generate an ETAS sequence
///
/// ## Usage
/// ```
/// use etas::{generate_sequence, Args};
///
/// let args = Args::build().unwrap();
/// let result = generate_sequence(&args).unwrap();
///
/// if let Some(seq) = result {
///     println!("{} events were generated!", seq.len());
/// }
/// ```
pub fn generate_sequence(args: &Args) -> CResult<Option<Vec<Event>>> {
    let mut rng = if let Some(seed) = args.seed {
        ChaCha8Rng::seed_from_u64(seed)
    } else {
        ChaCha8Rng::from_entropy()
    };

    let num_events =
        Poisson::new(args.mu * args.t_end)?.sample(&mut rng) as usize;

    // No events were generated
    if num_events == 0 {
        return Ok(None);
    }

    let a = args.bar_n / (args.beta * args.c.powf(1.0 - args.p))
        * (args.p - 1.0)
        * (args.beta - args.alpha);
    let exp = Exp::<f32>::new(args.beta)?;
    let uniform = Uniform::<f32>::from(0.0..1.0);

    // Generate the background events
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

    // Sort the background events
    seq.sort_by(|e1, e2| e1.t.partial_cmp(&e2.t).unwrap());

    let bar = ProgressBar::new(args.t_end as u64);
    let template = "[{elapsed_precise}] {bar:50.cyan/blue} \
{pos}/{len} -- {msg}";
    bar.set_style(
        ProgressStyle::with_template(template)?.progress_chars("##-"),
    );
    let mut m_max = bg_m
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .cloned()
        .unwrap();
    let mut n = 0;
    let mut simulation_ended = false;

    // Generate all aftershocks
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
                    let parent = n;
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

        // The next two blocks check if the simulation is over or not
        // (we reached the end of the interval or we generated at least
        // n - 1 aftershocks with the argument max_len set to n).

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
        return Ok(None);
    }

    Ok(Some(seq))
}

/// Write a sequence of events to file
///
/// ## Usage
/// ```
/// use etas::{generate_sequence, write_to_file, Args};
/// use std::path::Path;
///
/// let args = Args::build().unwrap();
/// let path = Path::new(&args.filename);
/// let result = generate_sequence(&args).unwrap();
///
/// if let Some(seq) = result {
///     write_to_file(&seq, path).unwrap();
/// }
/// ```
pub fn write_to_file(seq: &[Event], path: &Path) -> CResult<()> {
    let mut file = BufWriter::new(File::create(path)?);

    file.write_all(b"id,time,magnitude,parent\n")?;

    for (i, _) in seq.iter().enumerate() {
        let e = &seq[i];
        file.write_fmt(format_args!("{},{},{},{}\n", i, e.t, e.m, e.parent))?;
    }

    Ok(())
}
