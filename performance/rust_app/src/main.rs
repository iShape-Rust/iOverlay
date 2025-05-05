use std::env;
use std::collections::HashMap;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::{MultithreadOptions, Precision, Solver, Strategy};
use crate::test::test_0_checkerboard::CheckerboardTest;
use crate::test::test_1_not_overlap::NotOverlapTest;
use crate::test::test_2_lines_net::LinesNetTest;
use crate::test::test_3_spiral::SpiralTest;
use crate::test::test_4_windows::WindowsTest;
use crate::test::test_5_nested_squares::CrossTest;
use crate::test::test_6_corrosion::CorrosionTest;
use crate::test::test_7_concentric::ConcentricTest;
use crate::test::test_8_wind_mill::WindMillTest;

mod test;

fn main() {
    let args = env::args();
    let mut args_iter = args.peekable();
    let mut args_map = HashMap::new();

    while let Some(arg) = args_iter.next() {
        if arg.starts_with("--") {
            let key = arg.trim_start_matches("--").to_owned();
            // If the next argument is also a key, store a boolean flag; otherwise, store the value.
            let value = if args_iter.peek().map_or(false, |a| a.starts_with("--")) {
                "true".to_string()
            } else {
                args_iter.next().unwrap()
            };
            args_map.insert(key, value);
        }
    }

    #[cfg(debug_assertions)]
    {
        if args_map.is_empty() {
            args_map.insert("multithreading".to_string(), "false".to_string());
            args_map.insert("complex".to_string(), "false".to_string());
            args_map.insert("test".to_string(), 7.to_string());
            let count = 32;
            args_map.insert("count".to_string(), count.to_string());
            args_map.insert("geom".to_string(), true.to_string());
        }
    }

    let test_key = args_map.get("test").expect("Test number is not set");
    let multithreading_key = args_map.get("multithreading").expect("Multithreading is not set");
    let complex_key = args_map.get("complex").expect("Complex is not set");
    let geom_key = args_map.get("geom");

    let test: usize = test_key.parse().expect("Unable to parse test as an integer");
    let multithreading: bool = multithreading_key.parse().expect("Unable to parse multithreading as an boolean");
    let complex: bool = complex_key.parse().expect("Unable to parse complex as an boolean");

    let geom = if let Some(key) = geom_key {
        let value: bool = key.parse().expect("Unable to parse complex as an boolean");
        value
    } else {
        false
    };

    let multithreading = if multithreading {
        Some(MultithreadOptions::default())
    } else {
        None
    };

    let solver = Solver { strategy: Strategy::Auto, precision: Precision::HIGH, multithreading};

    if complex {
        match test {
            0 => {
                run_test_0(geom, solver);
            }
            1 => {
                run_test_1(geom, solver);
            }
            2 => {
                run_test_2(geom, solver);
            }
            3 => {
                run_test_3();
            }
            4 => {
                run_test_4(geom, solver);
            }
            5 => {
                run_test_5(geom, solver);
            }
            6 => {
                run_test_6(solver);
            }
            7 => {
                run_test_7(solver);
            }
            8 => {
                run_test_8(geom, solver);
            }
            _ => {
                println!("Test is not found");
            }
        }
    } else {
        let count_key = args_map.get("count").expect("Count is not set");
        let count: usize = count_key.parse().expect("Unable to parse count as an integer");
        match test {
            0 => {
                CheckerboardTest::run(count, OverlayRule::Xor, solver, 1.0, geom);
            }
            1 => {
                NotOverlapTest::run(count, OverlayRule::Union, solver, 1.0, geom);
            }
            2 => {
                LinesNetTest::run(count, OverlayRule::Intersect, solver, 1.0, geom);
            }
            3 => {
                SpiralTest::run(count, 100.0);
            }
            4 => {
                WindowsTest::run(count, OverlayRule::Difference, solver, 1.0, geom);
            }
            5 => {
                CrossTest::run(count, OverlayRule::Xor, solver, 1.0, geom);
            }
            6 => {
                CorrosionTest::run(count, OverlayRule::Difference, solver, 1.0);
            }
            7 => {
                ConcentricTest::run(count, OverlayRule::Intersect, solver, 1.0);
            }
            8 => {
                WindMillTest::run(count, OverlayRule::Intersect, solver, 1.0, geom);
            }
            _ => {
                println!("Test is not found");
            }
        }
    }
}

fn run_test_0(geom: bool, solver: Solver) {
    println!("run Checkerboard test");
    for i in 1..12 {
        let n = 1 << i;
        CheckerboardTest::run(n, OverlayRule::Xor, solver, 1000.0, geom);
    }
}

fn run_test_1(geom: bool, solver: Solver) {
    println!("run NotOverlap test");
    for i in 1..12 {
        let n = 1 << i;
        NotOverlapTest::run(n, OverlayRule::Xor, solver, 1000.0, geom);
    }
}

fn run_test_2(geom: bool, solver: Solver) {
    println!("run LinesNet test");
    for i in 1..12 {
        let n = 1 << i;
        LinesNetTest::run(n, OverlayRule::Intersect, solver, 500.0, geom);
    }
}

fn run_test_3() {
    println!("run Spiral test");
    for i in 1..21 {
        let n = 1 << i;
        SpiralTest::run(n, 1000.0)
    }
}

fn run_test_4(geom: bool, solver: Solver) {
    println!("run Windows test");
    for i in 1..12 {
        let n = 1 << i;
        WindowsTest::run(n, OverlayRule::Difference, solver, 500.0, geom);
    }
}

fn run_test_5(geom: bool, solver: Solver) {
    println!("run NestedSquares test");
    for i in 1..19 {
        let n = 1 << i;
        CrossTest::run(n, OverlayRule::Xor, solver, 500.0, geom);
    }
}

fn run_test_6(solver: Solver) {
    println!("run Corrosion test");
    let mut n = 1;
    for _ in 1..12 {
        CorrosionTest::run(n, OverlayRule::Difference, solver, 100.0);
        n = n << 1;
    }
}

fn run_test_7(solver: Solver) {
    println!("run Concentric test");
    let mut n = 1;
    for _ in 1..12 {
        ConcentricTest::run(n, OverlayRule::Intersect, solver, 100.0);
        n = n << 1;
    }
}

fn run_test_8(geom: bool, solver: Solver) {
    println!("run WindMill test");
    let mut n = 1;
    for _ in 1..12 {
        WindMillTest::run(n, OverlayRule::Difference, solver, 100.0, geom);
        n = n << 1;
    }
    WindMillTest::validate(100, OverlayRule::Difference, solver);
}