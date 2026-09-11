use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;
use i_overlay::core::edge_data::{EdgeDataMerge, EdgeDataSplit, OverlayEdgeData};
use i_overlay::core::edge_overlay::{EdgeOverlay, InputEdge};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::segm::boolean::ShapeCountBoolean;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Endpoints {
    a: [i64; 2],
    b: [i64; 2],
}

#[derive(Default)]
struct Calls {
    split: usize,
    merge: usize,
    reverse: usize,
}

fn point<I: IntNumber>(p: IntPoint<I>) -> [i64; 2] {
    // Test coordinates are small integers, exactly representable in f64.
    [p.x.to_f64() as i64, p.y.to_f64() as i64]
}

impl OverlayEdgeData for Endpoints {
    type Store = Calls;

    fn reversed(self, store: &mut Calls) -> Self {
        store.reverse += 1;
        Self { a: self.b, b: self.a }
    }

    fn split<I: IntNumber>(self, ctx: EdgeDataSplit<I>, store: &mut Calls) -> (Self, Self) {
        store.split += 1;
        let (a, p, b) = (point(ctx.a), point(ctx.p), point(ctx.b));
        assert_eq!(
            self,
            Self { a, b },
            "split context must match the current edge data"
        );
        assert!(a != p && p != b);
        (Self { a, b: p }, Self { a: p, b })
    }

    fn merge(ctx: EdgeDataMerge<ShapeCountBoolean, Self>, store: &mut Calls) -> Self {
        store.merge += 1;
        assert_eq!(
            ctx.lhs_data, ctx.rhs_data,
            "merged edges must share directed endpoints"
        );
        ctx.lhs_data
    }
}

fn edges(path: &[IntPoint]) -> Vec<InputEdge<i32, Endpoints>> {
    path.iter()
        .zip(path.iter().cycle().skip(1))
        .map(|(&a, &b)| InputEdge {
            a,
            b,
            data: Endpoints {
                a: point(a),
                b: point(b),
            },
        })
        .collect()
}

#[test]
fn edge_data_tracks_splits_merges_and_output_direction() {
    let subject = [[0, 0], [10, 0], [10, 10], [0, 10]].map(|p| IntPoint::new(p[0], p[1]));
    let clip = [[5, 0], [15, 0], [15, 10], [5, 10]].map(|p| IntPoint::new(p[0], p[1]));
    let mut seed = 0x1359_8472_fa99_217b_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 41) as i32 - 20
    };
    for case in 0..501 {
        let (a, b) = if case == 0 {
            (subject.to_vec(), clip.to_vec())
        } else {
            (
                (0..3 + case % 8).map(|_| IntPoint::new(next(), next())).collect(),
                (0..3 + case % 9).map(|_| IntPoint::new(next(), next())).collect(),
            )
        };
        let mut overlay = EdgeOverlay::new(a.len() + b.len());
        overlay.solver = [Solver::LIST, Solver::TREE, Solver::FRAG][case % 3];
        overlay.add_edges(edges(&a), ShapeType::Subject);
        overlay.add_edges(edges(&b), ShapeType::Clip);
        for clockwise in [false, true] {
            overlay.options.output_direction = if clockwise {
                ContourDirection::Clockwise
            } else {
                ContourDirection::CounterClockwise
            };
            for rule in [
                OverlayRule::Union,
                OverlayRule::Intersect,
                OverlayRule::Difference,
                OverlayRule::Xor,
            ] {
                let shapes = overlay.build_vector_shapes(rule, FillRule::EvenOdd);
                for edge in shapes.iter().flatten().flatten() {
                    assert_eq!(
                        edge.data,
                        Endpoints {
                            a: point(edge.a),
                            b: point(edge.b)
                        },
                        "case={case}, clockwise={clockwise}, rule={rule:?}, edge={edge:?}"
                    );
                }
                for edge in overlay.build_vectors(rule, FillRule::EvenOdd) {
                    assert_eq!(
                        edge.data,
                        Endpoints {
                            a: point(edge.a),
                            b: point(edge.b)
                        },
                        "flat vector: case={case}, edge={edge:?}"
                    );
                }
            }
        }
        if case == 0 {
            let store = overlay.data_store();
            assert!(store.split > 0 && store.merge > 0 && store.reverse > 0);
        }
    }
}
