use std::{collections::BTreeMap, time::Duration};

#[derive(Debug)]
pub struct Config {
    pub tests: Vec<usize>,
    pub counts: Vec<usize>,
    pub seed: u64,
    pub strategies: Vec<String>,
    pub engines: Vec<String>,
    pub modes: Vec<String>,
    pub multithreading: bool,
    pub label: String,
    pub warmup: usize,
    pub samples: usize,
    pub iterations: usize,
    pub budget: Option<Duration>,
    pub complex: bool,
    pub orientation: String,
    pub outlier: String,
    pub spiral: String,
    pub layout: String,
}
impl Config {
    pub fn variants(&self, id: usize) -> Vec<&str> {
        let (value, all): (&str, &[&str]) = match id {
            1 => (&self.layout, &["grid", "line"]),
            3 => (&self.spiral, &["float", "integer"]),
            9 => (&self.orientation, &["horizontal", "vertical"]),
            12 => (&self.outlier, &["cluster", "outlier"]),
            _ => return vec!["default"],
        };
        if value == "all" {
            all.to_vec()
        } else {
            vec![value]
        }
    }
}
const HELP: &str = "iOverlay numbered performance tests (release)\n\
--test 0..13|all (comma lists; default all) --count 8 (comma lists)\n\
--complex false|true (true uses 2,4,8,16,32,64, caps quadratic tests)\n\
--strategy auto|list|tree|frag|all (default all) --int i16|i32|i64|all (default i32)\n\
--mode cold|reuse|all --multithreading false|true (requires feature multithreading)\n\
--seed 0 --warmup 2 --samples 7 --iterations 3 --budget-seconds 30 (0 disables budget) --label local\n\
--orientation horizontal|vertical|all (test 9; default all)\n\
--outlier false|true|all (test 12; default all)\n\
--spiral float|integer|all (test 3; default float)\n\
--layout grid|line|all (test 1; default grid)\n\
--list lists numbered tests. stdout is CSV; diagnostics are on stderr.\n";

pub fn parse(args: impl Iterator<Item = String>) -> Result<Option<Config>, String> {
    let mut args = args.peekable();
    let mut map = BTreeMap::new();
    while let Some(arg) = args.next() {
        if arg == "--help" || arg == "-h" {
            eprint!("{HELP}");
            return Ok(None);
        }
        if arg == "--list" {
            for (id, name) in crate::test::NAMES.iter().enumerate() {
                eprintln!("test_{id}_{name}");
            }
            return Ok(None);
        }
        if arg == "--extended" {
            map.insert("complex".to_string(), "true".to_string());
            continue;
        }
        let key = arg
            .strip_prefix("--")
            .or_else(|| arg.strip_prefix('-'))
            .ok_or_else(|| format!("expected --option: {arg}"))?;
        let key = if key == "size" { "count" } else { key };
        if ![
            "test",
            "count",
            "complex",
            "seed",
            "strategy",
            "int",
            "mode",
            "multithreading",
            "warmup",
            "samples",
            "iterations",
            "budget-seconds",
            "label",
            "orientation",
            "outlier",
            "spiral",
            "layout",
        ]
        .contains(&key)
        {
            return Err(format!("unknown option: {arg}"));
        }
        let value = if ["complex", "multithreading"].contains(&key)
            && args.peek().is_none_or(|s| s.starts_with('-'))
        {
            "true".into()
        } else {
            args.next()
                .filter(|s| !s.starts_with('-'))
                .ok_or_else(|| format!("missing value for {arg}"))?
        };
        if map.insert(key.to_string(), value).is_some() {
            return Err(format!("duplicate option: {arg}"));
        }
    }
    let get = |key: &str, default: &str| map.get(key).cloned().unwrap_or_else(|| default.into());
    let choices = |key: &str, default: &str, allowed: &[&str]| -> Result<Vec<String>, String> {
        let value = get(key, default);
        let result: Vec<String> = if value == "all" {
            allowed.iter().map(|s| s.to_string()).collect()
        } else {
            value.split(',').map(str::to_string).collect()
        };
        if result.iter().any(|s| !allowed.contains(&s.as_str())) {
            return Err(format!("invalid {key}: {value}"));
        }
        let mut unique = result.clone();
        unique.sort();
        unique.dedup();
        if unique.len() != result.len() {
            return Err(format!("duplicate {key} values"));
        }
        Ok(result)
    };
    let number = |key: &str, default: &str, min: usize, max: usize| -> Result<usize, String> {
        let n = get(key, default)
            .parse::<usize>()
            .map_err(|_| format!("invalid {key}"))?;
        if !(min..=max).contains(&n) {
            return Err(format!("{key} must be in {min}..={max}"));
        }
        Ok(n)
    };
    let complex = get("complex", "false")
        .parse::<bool>()
        .map_err(|_| "invalid complex boolean".to_string())?;
    let tests = if get("test", "all") == "all" {
        (0..crate::test::NAMES.len()).collect()
    } else {
        get("test", "all")
            .split(',')
            .map(|s| {
                s.parse::<usize>()
                    .map_err(|_| "invalid test number".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?
    };
    if tests.iter().any(|&id| id >= crate::test::NAMES.len()) {
        return Err("test must be in 0..=13".into());
    }
    let mut unique = tests.clone();
    unique.sort();
    unique.dedup();
    if unique.len() != tests.len() {
        return Err("duplicate test numbers".into());
    }
    let counts = if complex {
        vec![2, 4, 8, 16, 32, 64]
    } else {
        get("count", "8")
            .split(',')
            .map(|s| s.parse::<usize>().map_err(|_| "invalid count".to_string()))
            .collect::<Result<Vec<_>, _>>()?
    };
    if counts.iter().any(|n| !(1..=4096).contains(n)) {
        return Err("count must be in 1..=4096".into());
    }
    let seed = get("seed", "0");
    let seed = if let Some(s) = seed.strip_prefix("0x") {
        u64::from_str_radix(s, 16)
    } else {
        seed.parse()
    }
    .map_err(|_| "invalid seed".to_string())?;
    let multithreading = get("multithreading", "false")
        .parse::<bool>()
        .map_err(|_| "invalid multithreading boolean".to_string())?;
    if multithreading && !cfg!(feature = "multithreading") {
        return Err("multithreading=true requires --features multithreading".into());
    }
    let mut engines = choices("int", "i32", &["i16", "i32", "i64", "16", "32", "64"])?;
    if get("int", "i32") == "all" {
        engines = vec!["i16".into(), "i32".into(), "i64".into()];
    }
    for e in &mut engines {
        if !e.starts_with('i') {
            *e = format!("i{e}");
        }
    }
    let mut engine_unique = engines.clone();
    engine_unique.sort();
    engine_unique.dedup();
    if engine_unique.len() != engines.len() {
        return Err("duplicate integer engines".into());
    }
    let single = |key: &str, default: &str, allowed: &[&str]| -> Result<String, String> {
        let v = get(key, default);
        if v != "all" && !allowed.contains(&v.as_str()) {
            return Err(format!("invalid {key}: {v}"));
        }
        Ok(v)
    };
    let outlier = single("outlier", "all", &["false", "true"])?;
    let outlier = match outlier.as_str() {
        "false" => "cluster".into(),
        "true" => "outlier".into(),
        _ => outlier,
    };
    let config = Config {
        tests,
        counts,
        seed,
        strategies: choices("strategy", "all", &["auto", "list", "tree", "frag"])?,
        engines,
        modes: choices("mode", "all", &["cold", "reuse"])?,
        multithreading,
        label: get("label", "local"),
        warmup: number("warmup", "2", 1, 100)?,
        samples: number("samples", "7", 3, 101)?,
        iterations: number("iterations", "3", 1, 10000)?,
        budget: match number("budget-seconds", "30", 0, 3600)? {
            0 => None,
            seconds => Some(Duration::from_secs(seconds as u64)),
        },
        complex,
        orientation: single("orientation", "all", &["horizontal", "vertical"])?,
        outlier,
        spiral: single("spiral", "float", &["float", "integer"])?,
        layout: single("layout", "grid", &["grid", "line"])?,
    };
    if !complex {
        for &id in &config.tests {
            for variant in config.variants(id) {
                for &n in &config.counts {
                    if n > crate::test::max_count(id, variant) {
                        return Err(format!(
                            "test {id} {variant}: count exceeds {}",
                            crate::test::max_count(id, variant)
                        ));
                    }
                }
            }
        }
    }
    Ok(Some(config))
}
#[cfg(test)]
mod checks {
    use super::*;
    fn p(args: &[&str]) -> Result<Option<Config>, String> {
        parse(args.iter().map(|s| s.to_string()))
    }
    #[test]
    fn original_numeric_cli() {
        let c = p(&[
            "--test",
            "0",
            "--count",
            "32",
            "--complex",
            "false",
            "--multithreading",
            "false",
            "--int",
            "all",
        ])
        .unwrap()
        .unwrap();
        assert_eq!(c.tests, vec![0]);
        assert_eq!(c.counts, vec![32]);
        assert_eq!(c.engines, vec!["i16", "i32", "i64"]);
        assert_eq!(
            p(&["--test", "3"]).unwrap().unwrap().variants(3),
            vec!["float"]
        );
        assert_eq!(
            p(&["--test", "1"]).unwrap().unwrap().variants(1),
            vec!["grid"]
        );
        assert_eq!(
            p(&["-test", "9", "-count", "8", "-int", "32"])
                .unwrap()
                .unwrap()
                .engines,
            vec!["i32"]
        );
    }
    #[test]
    fn limits_and_variants() {
        assert!(p(&["--budget-seconds", "0"])
            .unwrap()
            .unwrap()
            .budget
            .is_none());
        assert_eq!(
            p(&[]).unwrap().unwrap().budget,
            Some(Duration::from_secs(30))
        );
        assert!(p(&["--test", "14"]).is_err());
        assert!(p(&["--test"]).is_err());
        assert!(p(&["--count", "0"]).is_err());
        assert!(p(&["--samples", "2"]).is_err());
        assert!(p(&["--test", "2", "--count", "65"]).is_err());
        assert_eq!(
            p(&["--test", "9"]).unwrap().unwrap().variants(9),
            vec!["horizontal", "vertical"]
        );
        assert_eq!(
            p(&["--test", "12", "--outlier", "true"])
                .unwrap()
                .unwrap()
                .variants(12),
            vec!["outlier"]
        );
    }
}
