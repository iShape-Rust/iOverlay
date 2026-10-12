pub(crate) mod data;
pub(crate) mod runner;
pub(crate) mod test_0_checkerboard;
pub(crate) mod test_10_diagonal;
pub(crate) mod test_11_collinear;
pub(crate) mod test_12_cluster;
pub(crate) mod test_13_seeded;
pub(crate) mod test_1_not_overlap;
pub(crate) mod test_2_lines_net;
pub(crate) mod test_3_spiral;
pub(crate) mod test_4_windows;
pub(crate) mod test_5_nested_squares;
pub(crate) mod test_6_corrosion;
pub(crate) mod test_7_concentric;
pub(crate) mod test_8_wind_mill;
pub(crate) mod test_9_orientation;
pub(crate) mod util;

pub const NAMES: &[&str] = &[
    "checkerboard",
    "not_overlap",
    "lines_net",
    "spiral",
    "windows",
    "nested_squares",
    "corrosion",
    "concentric",
    "wind_mill",
    "orientation",
    "diagonal",
    "collinear",
    "cluster",
    "seeded",
];

pub fn max_count(id: usize, variant: &str) -> usize {
    match id {
        0 | 2 | 4 | 6 | 8 => 64,
        1 if variant == "grid" => 64,
        7 => 16,
        _ => 4096,
    }
}
pub fn generate<I: runner::Engine>(
    id: usize,
    n: usize,
    seed: u64,
    variant: &str,
) -> Result<data::Fixture, String> {
    if id >= NAMES.len() {
        return Err(format!("unknown test: {id}"));
    }
    if n == 0 || n > max_count(id, variant) {
        return Err(format!(
            "test {id} {variant}: count must be in 1..={}",
            max_count(id, variant)
        ));
    }
    Ok(match id {
        0 => test_0_checkerboard::CheckerboardTest::geometry(n),
        1 => test_1_not_overlap::NotOverlapTest::geometry(n, variant == "line"),
        2 => test_2_lines_net::LinesNetTest::geometry(n),
        3 => test_3_spiral::SpiralTest::geometry::<I>(n, variant == "integer"),
        4 => test_4_windows::WindowsTest::geometry(n),
        5 => test_5_nested_squares::CrossTest::geometry(n),
        6 => test_6_corrosion::CorrosionTest::geometry::<I>(n),
        7 => test_7_concentric::ConcentricTest::geometry::<I>(n),
        8 => test_8_wind_mill::WindMillTest::geometry(n),
        9 => test_9_orientation::OrientationTest::geometry(n, variant == "vertical"),
        10 => test_10_diagonal::DiagonalTest::geometry(n),
        11 => test_11_collinear::CollinearTest::geometry(n),
        12 => test_12_cluster::ClusterTest::geometry(n, seed, variant == "outlier"),
        13 => test_13_seeded::SeededTest::geometry(n, seed),
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod checks {
    use super::*;
    use i_overlay::core::{fill_rule::FillRule, overlay::Overlay};
    use runner::{area2, canonical, convert, normalize, solver, Engine};

    fn check<I: Engine>() {
        for id in 0..NAMES.len() {
            let variants: &[&str] = match id {
                1 => &["grid", "line"],
                3 => &["float", "integer"],
                9 => &["horizontal", "vertical"],
                12 => &["cluster", "outlier"],
                _ => &["default"],
            };
            for variant in variants {
                for n in [1, 4, 8] {
                    let f = generate::<I>(id, n, 0, variant).unwrap();
                    let (subj, clip) = convert::<I>(&f).unwrap();
                    let expected = canonical(
                        &Overlay::from_subj_and_clip_custom(
                            &subj,
                            &clip,
                            Default::default(),
                            solver("list", false),
                        )
                        .overlay(f.rule, FillRule::NonZero),
                    );
                    for strategy in ["auto", "tree", "frag"] {
                        let mut overlay = Overlay::from_subj_and_clip_custom(
                            &subj,
                            &clip,
                            Default::default(),
                            solver(strategy, false),
                        );
                        assert_eq!(
                            canonical(&overlay.overlay(f.rule, FillRule::NonZero)),
                            expected,
                            "test {id} {variant} n={n} {strategy} i{}",
                            I::BITS
                        );
                        overlay.reinit_with_subj_and_clip(&subj, &clip);
                        assert_eq!(
                            canonical(&overlay.overlay(f.rule, FillRule::NonZero)),
                            expected
                        );
                    }
                    let area = match id {
                        1 if *variant == "line" => Some(2 * n as i128 * 64),
                        1 => Some((n * n + (n - 1) * (n - 1)) as i128 * 100),
                        2 => Some(50 * n as i128 * (2 * n as i128 - 1)),
                        4 => Some(n as i128 * n as i128 * 300),
                        9 => Some(n as i128 * (96 * n as i128 + 24)),
                        _ => None,
                    };
                    if let Some(a) = area {
                        assert_eq!(area2(&expected), 2 * a, "test {id} {variant}");
                    }
                    if id == 8 {
                        assert_eq!(expected.len(), 4 * n * n);
                    }
                    if id == 9 {
                        let v = generate::<I>(
                            9,
                            n,
                            0,
                            if *variant == "horizontal" {
                                "vertical"
                            } else {
                                "horizontal"
                            },
                        )
                        .unwrap();
                        let (vs, vc) = convert::<I>(&v).unwrap();
                        let actual = canonical(
                            &Overlay::from_subj_and_clip_custom(
                                &vs,
                                &vc,
                                Default::default(),
                                solver("auto", false),
                            )
                            .overlay(v.rule, FillRule::NonZero),
                        );
                        let mut turned = expected.clone();
                        data::rotate(&mut turned);
                        if *variant == "vertical" {
                            data::rotate(&mut turned);
                            data::rotate(&mut turned);
                        }
                        turned = turned.into_iter().map(normalize).collect();
                        turned.sort();
                        assert_eq!(turned, actual);
                    }
                }
            }
        }
    }
    #[test]
    fn geometry_i16() {
        check::<i16>();
    }
    #[test]
    fn geometry_i32() {
        check::<i32>();
    }
    #[test]
    fn geometry_i64() {
        check::<i64>();
    }
    #[test]
    fn preserved_integer_geometry_fingerprints() {
        for (id, variant, h) in [
            (0, "default", 0xa5ecab9f1142e330),
            (1, "grid", 0xc4788e07a48e5030),
            (2, "default", 0x1a29e4309422f995),
            (4, "default", 0x7d79add12de90845),
            (5, "default", 0xf9e56b15fab43ea5),
            (8, "default", 0xa296ee90b1d68f8d),
        ] {
            for actual in [
                generate::<i16>(id, 8, 0, variant).unwrap(),
                generate::<i32>(id, 8, 0, variant).unwrap(),
                generate::<i64>(id, 8, 0, variant).unwrap(),
            ] {
                assert_eq!(actual.fingerprint(), h, "test {id}");
            }
        }
        for (id, variant, h) in [
            (1, "line", 0x5bc125b5c077b125),
            (3, "integer", 0x15c955bcde8dd41b),
            (9, "horizontal", 0x2e8379b513bb23a5),
            (9, "vertical", 0x2e6025fd4dd7ea65),
            (10, "default", 0xf5d2281a93ecb815),
            (11, "default", 0x83e26e355c378485),
            (12, "cluster", 0x4db2dda094b2b7a0),
            (12, "outlier", 0x61db173897cc7820),
            (13, "default", 0xbd5ec9746049328d),
        ] {
            assert_eq!(
                generate::<i32>(id, 32, 0, variant).unwrap().fingerprint(),
                h
            );
        }
    }

    #[test]
    fn seeded_vectors_and_pair_structure() {
        for (seed, h) in [
            (0, 0x77ac91e43630115d),
            (42, 0x95eba339e7c58a49),
            (u64::MAX, 0xb3bbd963a77647f5),
        ] {
            assert_eq!(
                generate::<i32>(13, 8, seed, "default")
                    .unwrap()
                    .fingerprint(),
                h
            );
        }
        let mut h = generate::<i32>(9, 32, 0, "horizontal").unwrap();
        data::rotate(&mut h.subj);
        data::rotate(&mut h.clip);
        let v = generate::<i32>(9, 32, 0, "vertical").unwrap();
        assert_eq!(h.subj, v.subj);
        assert_eq!(h.clip, v.clip);
        let a = generate::<i16>(12, 32, 0, "cluster").unwrap();
        let b = generate::<i16>(12, 32, 0, "outlier").unwrap();
        assert_eq!(a.edges(), b.edges());
        assert_eq!(&a.subj[..32], &b.subj[..32]);
        assert_eq!(a.clip, b.clip);
        assert!(generate::<i32>(2, 65, 0, "default").is_err());
        assert!(generate::<i32>(14, 8, 0, "default").is_err());
    }
}
