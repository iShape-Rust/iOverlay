use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::string::overlay::StringOverlay;
use i_overlay::string::rule::StringRule;

fn area_two(path: &[IntPoint]) -> i64 {
    path.iter()
        .zip(path.iter().cycle().skip(1))
        .map(|(a, b)| i64::from(a.x) * i64::from(b.y) - i64::from(a.y) * i64::from(b.x))
        .sum()
}

// Independent ray-crossing oracle. Samples have odd coordinates; cut edges
// lie on even grid lines, so no sample lies on an input boundary.
fn contains(path: &[IntPoint], p: IntPoint) -> bool {
    let mut inside = false;
    for (a, b) in path.iter().zip(path.iter().cycle().skip(1)) {
        if (a.y > p.y) != (b.y > p.y) {
            let cross =
                i64::from(b.x - a.x) * i64::from(p.y - a.y) - i64::from(b.y - a.y) * i64::from(p.x - a.x);
            if (cross > 0) == (b.y > a.y) {
                inside = !inside;
            }
        }
    }
    inside
}

#[test]
fn grid_cuts_partition_subject_without_gaps_or_overlaps() {
    let subject = [
        IntPoint::new(0, 0),
        IntPoint::new(20, 0),
        IntPoint::new(20, 20),
        IntPoint::new(0, 20),
    ];
    let mut seed = 0x7e59_1928_f17a_57b3_u64;
    for case in 0..512 {
        let mut cuts = Vec::new();
        for x in (0..=20).step_by(2) {
            for y in (0..=20).step_by(2) {
                for (dx, dy) in [(2, 0), (0, 2)] {
                    if x + dx > 20 || y + dy > 20 {
                        continue;
                    }
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    if (seed >> 32) % 100 < 30 + case % 60 {
                        cuts.push([IntPoint::new(x, y), IntPoint::new(x + dx, y + dy)]);
                    }
                }
            }
        }
        let mut overlay = StringOverlay::from_shape(&subject);
        overlay.add_string_lines(&cuts);
        let graph = overlay.build_graph_view(FillRule::NonZero).unwrap();
        let result = graph.extract_shapes(StringRule::Slice);
        let total: i64 = result.iter().flatten().map(|path| area_two(path)).sum();
        assert_eq!(total, 800, "case={case}, cuts={cuts:?}, result={result:?}");
        for x in (1..20).step_by(2) {
            for y in (1..20).step_by(2) {
                let p = IntPoint::new(x, y);
                let count = result
                    .iter()
                    .filter(|shape| {
                        contains(&shape[0], p) && !shape[1..].iter().any(|hole| contains(hole, p))
                    })
                    .count();
                assert_eq!(
                    count, 1,
                    "case={case}, sample={p:?}, cuts={cuts:?}, result={result:?}"
                );
            }
        }
    }
}

#[test]
fn interior_self_crossing_cuts_preserve_subject_area() {
    let subject = [
        IntPoint::new(0, 0),
        IntPoint::new(100, 0),
        IntPoint::new(100, 100),
        IntPoint::new(0, 100),
    ];
    let mut seed = 0x61f4_2ce8_290b_5b83_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 91) as i32 + 5
    };
    for case in 0..3000 {
        let mut cut: Vec<_> = (0..3 + case % 25)
            .map(|_| IntPoint::new(next(), next()))
            .collect();
        if case % 2 == 0 {
            cut.push(cut[0]);
        }
        let mut overlay = StringOverlay::from_shape(&subject);
        overlay.add_string_path(&cut);
        let graph = overlay.build_graph_view(FillRule::NonZero).unwrap();
        let result = graph.extract_shapes(StringRule::Slice);
        let total: i64 = result.iter().flatten().map(|path| area_two(path)).sum();
        assert_eq!(total, 20000, "case={case}, cut={cut:?}, result={result:?}");
        for shape in &result {
            assert!(
                area_two(&shape[0]) > 0,
                "invalid outer: case={case}, cut={cut:?}, shape={shape:?}"
            );
            for hole in &shape[1..] {
                assert!(
                    area_two(hole) < 0,
                    "invalid hole: case={case}, cut={cut:?}, hole={hole:?}"
                );
            }
        }
    }
}
