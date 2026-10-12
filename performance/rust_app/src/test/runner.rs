use super::data::{self, Fixture, Path};
use crate::config::Config;
use i_overlay::core::{
    fill_rule::FillRule,
    integer::OverlayInt,
    overlay::Overlay,
    solver::{MultithreadOptions, Precision, Solver, Strategy},
};
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_shape::int::{path::IntPaths, shape::IntShapes};
use std::{
    hint::black_box,
    io::{self, Write},
    time::Instant,
};
pub trait Engine: OverlayInt {
    fn from_i64(n: i64) -> Self;
    fn to_i64(self) -> i64;
}
macro_rules! engine {
    ($i:ty) => {
        impl Engine for $i {
            fn from_i64(n: i64) -> Self {
                n as Self
            }
            fn to_i64(self) -> i64 {
                self as i64
            }
        }
    };
}
engine!(i16);
engine!(i32);
engine!(i64);

pub fn solver(strategy: &str, multithreading: bool) -> Solver {
    Solver {
        strategy: match strategy {
            "auto" => Strategy::Auto,
            "list" => Strategy::List,
            "tree" => Strategy::Tree,
            "frag" => Strategy::Frag,
            _ => unreachable!(),
        },
        precision: Precision::HIGH,
        multithreading: multithreading.then(MultithreadOptions::default),
    }
}

pub fn convert<I: Engine>(f: &Fixture) -> Result<(IntPaths<I>, IntPaths<I>), String> {
    let limit = 1i128 << (I::BITS - 2);
    let b = f.bounds();
    if [b.0, b.1].iter().any(|&v| i128::from(v) < -limit)
        || [b.2, b.3].iter().any(|&v| i128::from(v) >= limit)
    {
        return Err(format!(
            "coordinates {:?} exceed i{} integer overlay range [-{},{}]",
            b,
            I::BITS,
            limit,
            limit - 1
        ));
    }
    let cvt = |paths: &[Path]| {
        paths
            .iter()
            .map(|p| {
                p.iter()
                    .map(|&(x, y)| IntPoint::new(I::from_i64(x), I::from_i64(y)))
                    .collect()
            })
            .collect()
    };
    Ok((cvt(&f.subj), cvt(&f.clip)))
}

// Keep winding, remove cyclic-start and contour/shape ordering. Default output
// simplifies collinear vertices. Grouping is unnecessary for filled boundary equality.
pub fn canonical<I: Engine>(shapes: &IntShapes<I>) -> Vec<Path> {
    let mut paths: Vec<Path> = shapes
        .iter()
        .flatten()
        .map(|path| {
            let p: Path = path.iter().map(|p| (p.x.to_i64(), p.y.to_i64())).collect();
            normalize(p)
        })
        .collect();
    paths.sort();
    paths
}
pub fn normalize(p: Path) -> Path {
    if p.is_empty() {
        return p;
    }
    // Lexicographic cyclic minimum handles repeated vertices as well.
    let start = (0..p.len())
        .min_by(|&a, &b| {
            (0..p.len())
                .map(|i| p[(a + i) % p.len()])
                .cmp((0..p.len()).map(|i| p[(b + i) % p.len()]))
        })
        .unwrap();
    (0..p.len()).map(|i| p[(start + i) % p.len()]).collect()
}
pub fn area2(paths: &[Path]) -> i128 {
    paths
        .iter()
        .map(|p| {
            p.iter()
                .zip(p.iter().cycle().skip(1))
                .take(p.len())
                .map(|(&(x, y), &(u, v))| {
                    i128::from(x) * i128::from(v) - i128::from(y) * i128::from(u)
                })
                .sum::<i128>()
        })
        .sum()
}

fn csv(out: &mut impl Write, values: &[String]) -> io::Result<()> {
    writeln!(
        out,
        "{}",
        values
            .iter()
            .map(|v| format!("\"{}\"", v.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(",")
    )
}
pub const HEADER: &str = "schema,fixture_version,runner_version,library_version,library_rev,library_dirty,library_source,runner_source,build_unix,rustc,target,profile,opt_level,debug_info,rustflags,host_os,logical_cpus,run_unix,label,threads_env,mt_compiled,test,case,variant,size,seed,strategy,split_route_hint,fill_route_hint,int,multithreading,precision,rule,mode,input_edges,subj_contours,clip_contours,min_x,min_y,max_x,max_y,input_fingerprint,output_contours,output_edges,output_area2,validated,warmup,samples,iterations,median_ns,min_ns,max_ns,samples_ns";

// Hints only: the splitter receives normalized/merged segments, and may change
// strategy on subsequent passes; fill thresholds concern post-split geometry.
fn route(strategy: &str, edges: usize) -> &'static str {
    match strategy {
        "list" => "list",
        "frag" => "frag",
        "tree" if edges > 16000 => "frag",
        "tree" => "tree",
        "auto" if edges < 4000 => "list",
        "auto" if edges <= 16000 => "tree",
        _ => "frag",
    }
}

pub fn run<I: Engine>(
    config: &Config,
    id: usize,
    variant: &str,
    size: usize,
    f: &Fixture,
    deadline: Option<Instant>,
    out: &mut impl Write,
) -> Result<bool, String> {
    let case = super::NAMES[id];
    let (subj, clip) = match convert::<I>(f) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("skip {case} size={size}: {e}");
            return Ok(true);
        }
    };
    // Reference and all requested strategies are checked outside timing.
    let reference = canonical(
        &Overlay::from_subj_and_clip_custom(
            &subj,
            &clip,
            Default::default(),
            solver("list", false),
        )
        .overlay(f.rule, FillRule::NonZero),
    );
    for strategy in &config.strategies {
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            return Ok(false);
        }
        let s = solver(strategy, config.multithreading);
        let mut reusable = Overlay::from_subj_and_clip_custom(&subj, &clip, Default::default(), s);
        let result = reusable.overlay(f.rule, FillRule::NonZero);
        if canonical(&result) != reference {
            return Err(format!(
                "{case} size={size} i{} {strategy}: geometry differs from List reference",
                I::BITS
            ));
        }
        // Check reinit after the splitter has already modified the segment buffer.
        reusable.reinit_with_subj_and_clip(&subj, &clip);
        if canonical(&reusable.overlay(f.rule, FillRule::NonZero)) != reference {
            return Err(format!("{case}: reinit result differs"));
        }
        if id == 9 {
            let mut rotated = f.clone();
            data::rotate(&mut rotated.subj);
            data::rotate(&mut rotated.clip);
            let (rs, rc) = convert::<I>(&rotated)?;
            let actual = canonical(
                &Overlay::from_subj_and_clip_custom(&rs, &rc, Default::default(), s)
                    .overlay(f.rule, FillRule::NonZero),
            );
            let mut expected = reference.clone();
            data::rotate(&mut expected);
            expected = expected.into_iter().map(normalize).collect();
            expected.sort();
            if actual != expected {
                return Err(format!("{case}: exact rotation invariant failed"));
            }
        }
        for mode in &config.modes {
            let mut operation = || {
                if mode == "cold" {
                    let mut overlay = Overlay::from_subj_and_clip_custom(
                        black_box(&subj),
                        black_box(&clip),
                        Default::default(),
                        s,
                    );
                    black_box(overlay.overlay(black_box(f.rule), FillRule::NonZero));
                } else {
                    reusable.reinit_with_subj_and_clip(black_box(&subj), black_box(&clip));
                    black_box(reusable.overlay(black_box(f.rule), FillRule::NonZero));
                }
            };
            for _ in 0..config.warmup {
                if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                    return Ok(false);
                }
                operation();
            }
            let mut samples = Vec::with_capacity(config.samples);
            for _ in 0..config.samples {
                if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                    return Ok(false);
                }
                let start = Instant::now();
                for _ in 0..config.iterations {
                    operation();
                }
                samples.push(start.elapsed().as_nanos() / config.iterations as u128);
            }
            let mut sorted = samples.clone();
            sorted.sort_unstable();
            let median = if sorted.len() % 2 == 0 {
                (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2
            } else {
                sorted[sorted.len() / 2]
            };
            let b = f.bounds();
            let fill_hint = match strategy.as_str() {
                "list" => "list",
                "auto" => "post_split_threshold_8000",
                _ => "tree",
            };
            let values = vec![
                "2".into(),
                "2".into(),
                env!("CARGO_PKG_VERSION").into(),
                env!("BENCH_LIBRARY_VERSION").into(),
                env!("BENCH_LIBRARY_REV").into(),
                env!("BENCH_LIBRARY_DIRTY").into(),
                env!("BENCH_LIBRARY_SOURCE").into(),
                env!("BENCH_RUNNER_SOURCE").into(),
                env!("BENCH_BUILD_UNIX").into(),
                env!("BENCH_RUSTC").into(),
                env!("BENCH_TARGET").into(),
                env!("BENCH_PROFILE").into(),
                env!("BENCH_OPT_LEVEL").into(),
                env!("BENCH_DEBUG").into(),
                env!("BENCH_RUSTFLAGS").into(),
                std::env::consts::OS.into(),
                std::thread::available_parallelism()
                    .map(|n| n.get().to_string())
                    .unwrap_or_else(|_| "unknown".into()),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
                config.label.clone(),
                std::env::var("RAYON_NUM_THREADS").unwrap_or_default(),
                cfg!(feature = "multithreading").to_string(),
                id.to_string(),
                case.into(),
                variant.into(),
                size.to_string(),
                config.seed.to_string(),
                strategy.clone(),
                route(strategy, f.edges()).into(),
                fill_hint.into(),
                format!("i{}", I::BITS),
                config.multithreading.to_string(),
                "HIGH".into(),
                format!("{:?}", f.rule),
                mode.clone(),
                f.edges().to_string(),
                f.subj.len().to_string(),
                f.clip.len().to_string(),
                b.0.to_string(),
                b.1.to_string(),
                b.2.to_string(),
                b.3.to_string(),
                format!("{:016x}", f.fingerprint()),
                reference.len().to_string(),
                reference.iter().map(Vec::len).sum::<usize>().to_string(),
                area2(&reference).to_string(),
                "true".into(),
                config.warmup.to_string(),
                config.samples.to_string(),
                config.iterations.to_string(),
                median.to_string(),
                sorted[0].to_string(),
                sorted.last().unwrap().to_string(),
                samples
                    .iter()
                    .map(u128::to_string)
                    .collect::<Vec<_>>()
                    .join(";"),
            ];
            csv(out, &values).map_err(|e| e.to_string())?;
        }
    }
    Ok(true)
}

#[cfg(test)]
mod checks {
    use super::*;
    use i_overlay::core::overlay_rule::OverlayRule;
    #[test]
    fn integer_coordinate_budget() {
        let f = Fixture {
            subj: vec![data::rect(16380, 0, 8, 8)],
            clip: vec![],
            rule: OverlayRule::Union,
        };
        assert!(convert::<i16>(&f).is_err());
        assert!(convert::<i32>(&f).is_ok());
        let f = Fixture {
            subj: vec![data::rect(-16384, -16384, 32767, 32767)],
            clip: vec![],
            rule: OverlayRule::Union,
        };
        assert!(convert::<i16>(&f).is_ok());
    }
}
