use crate::app::App;
use crate::app::AppError;
use crate::app::AppResult;
use crate::constants::TEMPLATE;
use indicatif::{ProgressBar, ProgressStyle};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Exp, Poisson, Uniform};
use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::{Deref, DerefMut},
    path::Path,
};

pub struct Event {
    pub t: f32,
    pub m: f32,
    pub parent: usize,
}

pub struct Sequence(Vec<Event>);

impl Deref for Sequence {
    type Target = Vec<Event>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Sequence {
    pub fn generate(args: &App) -> AppResult<Self> {
        let mut rng = if let Some(seed) = args.seed {
            ChaCha8Rng::seed_from_u64(seed)
        } else {
            ChaCha8Rng::from_entropy()
        };

        let num_events = Poisson::new(args.mu * args.t_end)
            .map_err(|e| AppError::Simulation(e.to_string()))?
            .sample(&mut rng) as usize;

        // No events were generated
        if num_events == 0 {
            return Ok(Self(vec![]));
        }

        let a = args.bar_n / (args.beta * args.c.powf(1.0 - args.p))
            * (args.p - 1.0)
            * (args.beta - args.alpha);
        let exp = Exp::<f32>::new(args.beta)
            .map_err(|e| AppError::Simulation(e.to_string()))?;
        let uniform = Uniform::<f32>::from(0.0..1.0);

        // Generate the background events
        let bg_t: Vec<f32> = Uniform::from(0.0..args.t_end)
            .sample_iter(&mut rng)
            .take(num_events)
            .collect();
        let bg_m: Vec<f32> =
            exp.sample_iter(&mut rng).take(num_events).collect();
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
        bar.set_style(
            ProgressStyle::with_template(TEMPLATE)
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
            return Ok(Self(vec![]));
        }

        Ok(Self(seq))
    }

    pub fn save(
        &self,
        path: &Path,
        verbose: bool,
        headers: bool,
    ) -> AppResult<()> {
        if self.is_empty() {
            if verbose {
                println!("Empty sequence: no events were saved.");
            }
            return Ok(());
        }

        let mut file = BufWriter::new(
            File::create(path).map_err(|e| AppError::Save(e.to_string()))?,
        );

        if headers {
            file.write_all(b"id,time,magnitude,parent\n")
                .map_err(|e| AppError::Save(e.to_string()))?;
        }

        for (i, e) in self.iter().enumerate() {
            file.write_fmt(format_args!(
                "{},{},{},{}\n",
                i, e.t, e.m, e.parent
            ))
            .map_err(|err| AppError::Save(err.to_string()))?;
        }

        if verbose {
            println!(
                "{} event{} written to file '{}'.",
                self.len(),
                if self.len() == 1 { '\0' } else { 's' },
                path.display()
            );
        }
        Ok(())
    }
}
