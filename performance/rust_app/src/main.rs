mod config;
mod test;
use config::Config;
use std::{
    io::{self, Write},
    time::Instant,
};

fn execute(config: Config) -> Result<(), String> {
    if cfg!(debug_assertions) {
        return Err("use cargo run --release".into());
    }
    let deadline = Instant::now() + config.budget;
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    writeln!(out, "{}", test::runner::HEADER).map_err(|e| e.to_string())?;
    for &id in &config.tests {
        for variant in config.variants(id) {
            for &n in &config.counts {
                if n > test::max_count(id, variant) {
                    if config.complex {
                        eprintln!("skip test {id} {variant} count={n}: size cap");
                        continue;
                    }
                    return Err(format!(
                        "test {id} {variant}: count exceeds {}",
                        test::max_count(id, variant)
                    ));
                }
                for engine in &config.engines {
                    if Instant::now() >= deadline {
                        eprintln!("budget reached; remaining combinations omitted");
                        return Ok(());
                    }
                    let completed = match engine.as_str() {
                        "i16" => {
                            let f = test::generate::<i16>(id, n, config.seed, variant)?;
                            test::runner::run::<i16>(
                                &config, id, variant, n, &f, deadline, &mut out,
                            )?
                        }
                        "i32" => {
                            let f = test::generate::<i32>(id, n, config.seed, variant)?;
                            test::runner::run::<i32>(
                                &config, id, variant, n, &f, deadline, &mut out,
                            )?
                        }
                        "i64" => {
                            let f = test::generate::<i64>(id, n, config.seed, variant)?;
                            test::runner::run::<i64>(
                                &config, id, variant, n, &f, deadline, &mut out,
                            )?
                        }
                        _ => unreachable!(),
                    };
                    if !completed {
                        eprintln!("budget reached; incomplete row omitted");
                        return Ok(());
                    }
                }
            }
        }
    }
    out.flush().map_err(|e| e.to_string())
}
fn main() {
    let result =
        config::parse(std::env::args().skip(1)).and_then(|c| c.map(execute).unwrap_or(Ok(())));
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(2);
    }
}
