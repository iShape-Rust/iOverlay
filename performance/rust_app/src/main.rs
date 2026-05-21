use crate::test::test_0_checkerboard::CheckerboardTest;
use crate::test::test_1_not_overlap::NotOverlapTest;
use crate::test::test_2_lines_net::LinesNetTest;
use crate::test::test_3_spiral::SpiralTest;
use crate::test::test_4_windows::WindowsTest;
use crate::test::test_5_nested_squares::CrossTest;
use crate::test::test_6_corrosion::CorrosionTest;
use crate::test::test_7_concentric::ConcentricTest;
use crate::test::test_8_wind_mill::WindMillTest;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::{MultithreadOptions, Precision, Solver, Strategy};
use std::collections::HashMap;
use std::env;

mod test;

#[derive(Clone, Copy)]
enum IntEngine {
    I16,
    I32,
    I64,
}

impl IntEngine {
    fn parse(value: &str) -> Vec<Self> {
        match value {
            "i16" | "16" => vec![Self::I16],
            "i32" | "32" => vec![Self::I32],
            "i64" | "64" => vec![Self::I64],
            "all" => vec![Self::I16, Self::I32, Self::I64],
            _ => panic!("Unknown int engine: {value}. Use i16, i32, i64, or all"),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
        }
    }
}

fn main() {
    let args = env::args();
    let mut args_iter = args.peekable();
    let mut args_map = HashMap::new();

    while let Some(arg) = args_iter.next() {
        if let Some(key) = arg.strip_prefix("--").or_else(|| arg.strip_prefix('-')) {
            // If the next argument is also a key, store a boolean flag; otherwise, store the value.
            let value = if args_iter.peek().map_or(false, |a| a.starts_with('-')) {
                "true".to_string()
            } else {
                args_iter.next().unwrap()
            };
            args_map.insert(key.to_owned(), value);
        }
    }

    #[cfg(debug_assertions)]
    {
        if args_map.is_empty() {
            args_map.insert("multithreading".to_string(), "false".to_string());
            args_map.insert("complex".to_string(), "false".to_string());
            args_map.insert("test".to_string(), 7.to_string());
            args_map.insert("int".to_string(), "i32".to_string());
            let count = 32;
            args_map.insert("count".to_string(), count.to_string());
        }
    }

    let test_key = args_map.get("test").expect("Test number is not set");
    let multithreading_key = args_map
        .get("multithreading")
        .expect("Multithreading is not set");
    let complex_key = args_map.get("complex").expect("Complex is not set");
    let int_key = args_map.get("int").map(String::as_str).unwrap_or("i32");

    let test: usize = test_key
        .parse()
        .expect("Unable to parse test as an integer");
    let multithreading: bool = multithreading_key
        .parse()
        .expect("Unable to parse multithreading as an boolean");
    let complex: bool = complex_key
        .parse()
        .expect("Unable to parse complex as an boolean");
    let int_engines = IntEngine::parse(int_key);

    let multithreading = if multithreading {
        Some(MultithreadOptions::default())
    } else {
        None
    };

    let solver = Solver {
        strategy: Strategy::Auto,
        precision: Precision::HIGH,
        multithreading,
    };

    for int_engine in int_engines {
        println!("int engine: {}", int_engine.label());
        run_selected_test(int_engine, test, complex, &args_map, solver);
    }
}

fn run_selected_test(
    int_engine: IntEngine,
    test: usize,
    complex: bool,
    args_map: &HashMap<String, String>,
    solver: Solver,
) {
    match int_engine {
        IntEngine::I16 => run_selected_test_with_int::<i16>(test, complex, args_map, solver),
        IntEngine::I32 => run_selected_test_with_int::<i32>(test, complex, args_map, solver),
        IntEngine::I64 => run_selected_test_with_int::<i64>(test, complex, args_map, solver),
    }
}

fn run_selected_test_with_int<I: test::util::OverlayInt>(
    test: usize,
    complex: bool,
    args_map: &HashMap<String, String>,
    solver: Solver,
) {
    if complex {
        match test {
            0 => {
                run_test_0::<I>(solver);
            }
            1 => {
                run_test_1::<I>(solver);
            }
            2 => {
                run_test_2::<I>(solver);
            }
            3 => {
                run_test_3::<I>(solver);
            }
            4 => {
                run_test_4::<I>(solver);
            }
            5 => {
                run_test_5::<I>(solver);
            }
            6 => {
                run_test_6::<I>(solver);
            }
            7 => {
                run_test_7::<I>(solver);
            }
            8 => {
                run_test_8::<I>(solver);
            }
            _ => {
                println!("Test is not found");
            }
        }
    } else {
        let count_key = args_map.get("count").expect("Count is not set");
        let count: usize = count_key
            .parse()
            .expect("Unable to parse count as an integer");
        match test {
            0 => {
                CheckerboardTest::run::<I>(count, OverlayRule::Xor, solver, 1.0);
            }
            1 => {
                NotOverlapTest::run::<I>(count, OverlayRule::Union, solver, 1.0);
            }
            2 => {
                LinesNetTest::run::<I>(count, OverlayRule::Intersect, solver, 1.0);
            }
            3 => {
                SpiralTest::run::<I>(count, solver, 100.0);
            }
            4 => {
                WindowsTest::run::<I>(count, OverlayRule::Difference, solver, 1.0);
            }
            5 => {
                CrossTest::run::<I>(count, OverlayRule::Xor, solver, 1.0);
            }
            6 => {
                CorrosionTest::run::<I>(count, OverlayRule::Difference, solver, 1.0);
            }
            7 => {
                ConcentricTest::run::<I>(count, OverlayRule::Intersect, solver, 1.0);
            }
            8 => {
                WindMillTest::run::<I>(count, OverlayRule::Intersect, solver, 1.0);
            }
            _ => {
                println!("Test is not found");
            }
        }
    }
}

fn run_test_0<I: test::util::OverlayInt>(solver: Solver) {
    println!("run Checkerboard test");
    for i in 1..12 {
        let n = 1 << i;
        CheckerboardTest::run::<I>(n, OverlayRule::Xor, solver, 1000.0);
    }
}

fn run_test_1<I: test::util::OverlayInt>(solver: Solver) {
    println!("run NotOverlap test");
    for i in 1..12 {
        let n = 1 << i;
        NotOverlapTest::run::<I>(n, OverlayRule::Xor, solver, 1000.0);
    }
}

fn run_test_2<I: test::util::OverlayInt>(solver: Solver) {
    println!("run LinesNet test");
    for i in 1..12 {
        let n = 1 << i;
        LinesNetTest::run::<I>(n, OverlayRule::Intersect, solver, 500.0);
    }
}

fn run_test_3<I: test::util::OverlayInt>(solver: Solver) {
    println!("run Spiral test");
    for i in 1..21 {
        let n = 1 << i;
        SpiralTest::run::<I>(n, solver, 1000.0)
    }
}

fn run_test_4<I: test::util::OverlayInt>(solver: Solver) {
    println!("run Windows test");
    for i in 1..12 {
        let n = 1 << i;
        WindowsTest::run::<I>(n, OverlayRule::Difference, solver, 500.0);
    }
}

fn run_test_5<I: test::util::OverlayInt>(solver: Solver) {
    println!("run NestedSquares test");
    for i in 1..18 {
        let n = 1 << i;
        CrossTest::run::<I>(n, OverlayRule::Xor, solver, 500.0);
    }
}

fn run_test_6<I: test::util::OverlayInt>(solver: Solver) {
    println!("run Corrosion test");
    let mut n = 1;
    for _ in 1..12 {
        CorrosionTest::run::<I>(n, OverlayRule::Difference, solver, 100.0);
        n = n << 1;
    }
}

fn run_test_7<I: test::util::OverlayInt>(solver: Solver) {
    println!("run Concentric test");
    let mut n = 1;
    for _ in 1..12 {
        ConcentricTest::run::<I>(n, OverlayRule::Intersect, solver, 100.0);
        n = n << 1;
    }
}

fn run_test_8<I: test::util::OverlayInt>(solver: Solver) {
    println!("run WindMill test");
    let mut n = 1;
    for _ in 1..12 {
        WindMillTest::run::<I>(n, OverlayRule::Difference, solver, 100.0);
        n = n << 1;
    }
    WindMillTest::validate::<I>(100, OverlayRule::Difference, solver);
}
