use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::hierarchy::{ChildLink, FlatShapeHierarchy};
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;
use i_shape::int::path::ContourExtension;
use std::panic::{AssertUnwindSafe, catch_unwind};

#[test]
fn randomized_boolean_hierarchy_matches_containment() {
    for iteration in 0..4_096 {
        let seed = next_stress_seed(iteration as u64);
        let result = catch_unwind(AssertUnwindSafe(|| run_stress_case(seed)));

        if let Err(payload) = result {
            panic!(
                "hierarchy stress failed: iteration={iteration} seed={seed} panic={}",
                panic_message(payload)
            );
        }
    }
}

fn run_stress_case(seed: u64) {
    let mut rng = StressRng::new(seed);
    let mut contours = Vec::new();
    let root_count = rng.range_usize(1, 4);

    for root_index in 0..root_count {
        let x = (root_index % 2) as i32 * 2_000_000 + rng.range_i32(0, 100_000);
        let y = (root_index / 2) as i32 * 2_000_000 + rng.range_i32(-100_000, 100_000);
        let width = rng.range_i32(800_000, 1_200_000);
        let height = rng.range_i32(800_000, 1_200_000);
        let depth = rng.range_usize(0, 3);
        append_shape_tree(
            Rect::new(x, y, x + width, y + height),
            depth,
            &mut rng,
            &mut contours,
        );
    }

    let (subject, clip, overlay_rule) = if seed & 1 == 0 {
        (contours, Vec::new(), OverlayRule::Subject)
    } else {
        let mut subject = Vec::new();
        let mut clip = Vec::new();
        for contour in contours {
            if rng.next_u32() & 1 == 0 {
                subject.push(contour);
            } else {
                clip.push(contour);
            }
        }
        (subject, clip, OverlayRule::Xor)
    };

    let mut overlay = Overlay::with_contours(&subject, &clip);
    if seed & 2 != 0 {
        overlay.options.output_direction = ContourDirection::Clockwise;
    }

    let hierarchy = overlay.overlay_hierarchy(overlay_rule, FillRule::EvenOdd);
    assert_hierarchy_matches_containment(&hierarchy, seed);
    assert_full_forest_iteration(&hierarchy, seed);
}

fn append_shape_tree(hull: Rect, depth: usize, rng: &mut StressRng, contours: &mut Vec<Vec<IntPoint<i32>>>) {
    contours.push(hull.to_contour(rng.next_u32() & 1 == 0));
    if depth == 0 || hull.width() < 1_000 || hull.height() < 1_000 {
        return;
    }

    let x_inset = hull.width() / rng.range_i32(8, 14);
    let y_inset = hull.height() / rng.range_i32(8, 14);
    let inner = hull.inset(x_inset, y_inset);
    let hole_count = rng.range_usize(1, 2);
    let hole_gap = inner.height() / 25;
    let hole_height = (inner.height() - hole_gap * (hole_count as i32 - 1)) / hole_count as i32;

    for hole_index in 0..hole_count {
        let y0 = inner.y0 + hole_index as i32 * (hole_height + hole_gap);
        let hole = Rect::new(inner.x0, y0, inner.x1, y0 + hole_height);
        contours.push(hole.to_contour(rng.next_u32() & 1 == 0));

        let child_count = rng.range_usize(0, 2);
        if child_count == 0 {
            continue;
        }

        let x_padding = hole.width() / 16;
        let y_padding = hole.height() / 10;
        let child_space = hole.inset(x_padding, y_padding);
        let child_gap = child_space.width() / 30;
        let child_width = (child_space.width() - child_gap * (child_count as i32 - 1)) / child_count as i32;

        for child_index in 0..child_count {
            let x0 = child_space.x0 + child_index as i32 * (child_width + child_gap);
            let child = Rect::new(x0, child_space.y0, x0 + child_width, child_space.y1);
            append_shape_tree(child, depth - 1, rng, contours);
        }
    }
}

fn assert_hierarchy_matches_containment(hierarchy: &FlatShapeHierarchy<i32>, seed: u64) {
    let shapes = &hierarchy.shapes;
    let mut expected = Vec::new();

    for (child_shape_index, shape_range) in shapes.shape_ranges.iter().enumerate() {
        let hull_range = &shapes.contour_ranges[shape_range.start];
        let sample = shapes.points[hull_range.start];
        let mut parent: Option<(u64, usize, usize)> = None;

        for (parent_shape_index, parent_shape_range) in shapes.shape_ranges.iter().enumerate() {
            for parent_contour_index in parent_shape_range.start + 1..parent_shape_range.end {
                let contour_range = &shapes.contour_ranges[parent_contour_index];
                let contour = &shapes.points[contour_range.clone()];
                if !contour.contains_point(sample) {
                    continue;
                }

                let area = contour.unsafe_area().unsigned_abs();
                if parent.is_none_or(|candidate| area < candidate.0) {
                    parent = Some((area, parent_shape_index, parent_contour_index));
                }
            }
        }

        if let Some((_, parent_shape_index, parent_contour_index)) = parent {
            expected.push(ChildLink {
                parent_shape_index,
                parent_contour_index,
                child_shape_index,
            });
        }
    }

    expected.sort_unstable();
    assert_eq!(hierarchy.links, expected, "seed={seed}");

    for pair in hierarchy.links.windows(2) {
        assert!(pair[0] <= pair[1], "links are not sorted: seed={seed}");
    }
}

fn assert_full_forest_iteration(hierarchy: &FlatShapeHierarchy<i32>, seed: u64) {
    let shape_count = hierarchy.shapes.shape_ranges.len();
    let mut children = vec![Vec::new(); shape_count];
    let mut incoming = vec![0usize; shape_count];
    let mut linked = vec![false; shape_count];

    for link in &hierarchy.links {
        assert!(link.parent_shape_index < shape_count, "seed={seed}");
        assert!(link.child_shape_index < shape_count, "seed={seed}");
        let parent_range = &hierarchy.shapes.shape_ranges[link.parent_shape_index];
        assert!(
            parent_range.start < link.parent_contour_index && link.parent_contour_index < parent_range.end,
            "seed={seed} link={link:?}"
        );

        children[link.parent_shape_index].push(link.child_shape_index);
        incoming[link.child_shape_index] += 1;
        linked[link.parent_shape_index] = true;
        linked[link.child_shape_index] = true;
    }

    assert!(incoming.iter().all(|&count| count <= 1), "seed={seed}");

    let mut visited = vec![false; shape_count];
    let mut stack = Vec::new();
    for shape_index in 0..shape_count {
        if incoming[shape_index] == 0 && !children[shape_index].is_empty() {
            stack.push(shape_index);
        }
    }

    while let Some(shape_index) = stack.pop() {
        assert!(!visited[shape_index], "cycle or duplicate visit: seed={seed}");
        visited[shape_index] = true;
        stack.extend(children[shape_index].iter().copied());
    }

    for shape_index in 0..shape_count {
        if !visited[shape_index] {
            assert!(!linked[shape_index], "unreachable linked shape: seed={seed}");
            visited[shape_index] = true;
        }
    }

    assert!(visited.into_iter().all(|value| value), "seed={seed}");
}

#[derive(Clone, Copy)]
struct Rect {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Rect {
    fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    fn width(self) -> i32 {
        self.x1 - self.x0
    }

    fn height(self) -> i32 {
        self.y1 - self.y0
    }

    fn inset(self, x: i32, y: i32) -> Self {
        Self::new(self.x0 + x, self.y0 + y, self.x1 - x, self.y1 - y)
    }

    fn to_contour(self, reversed: bool) -> Vec<IntPoint<i32>> {
        let mut contour = vec![
            IntPoint::new(self.x0, self.y0),
            IntPoint::new(self.x1, self.y0),
            IntPoint::new(self.x1, self.y1),
            IntPoint::new(self.x0, self.y1),
        ];
        if reversed {
            contour.reverse();
        }
        contour
    }
}

struct StressRng {
    state: u64,
}

impl StressRng {
    fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0xa076_1d64_78bd_642f,
        }
    }

    fn range_usize(&mut self, min: usize, max: usize) -> usize {
        min + self.next_u32() as usize % (max - min + 1)
    }

    fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        min + (self.next_u32() % (max - min + 1) as u32) as i32
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(0xe703_7ed1_a0b4_28db)
            .wrapping_add(0x8ebc_6af0_9c88_c6e3);
        (self.state >> 32) as u32
    }
}

fn next_stress_seed(seed: u64) -> u64 {
    seed.wrapping_mul(0xe703_7ed1_a0b4_28db)
        .wrapping_add(0x8ebc_6af0_9c88_c6e3)
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_owned()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "non-string panic payload".to_owned()
    }
}
