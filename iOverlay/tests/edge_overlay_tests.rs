use i_float::int::point::IntPoint;
use i_overlay::core::edge_data::{EdgeDataMerge, OverlayEdgeData};
use i_overlay::core::edge_overlay::{EdgeOverlay, InputEdge};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::ShapeType;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::segm::boolean::ShapeCountBoolean;
use i_overlay::vector::edge::DataVectorEdge;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Undefined,
}

impl OverlayEdgeData for Color {
    fn merge(ctx: EdgeDataMerge<ShapeCountBoolean, Self>) -> Self {
        match (ctx.lhs_data, ctx.rhs_data) {
            (Color::Red, Color::Red) => Color::Red,
            (Color::Green, Color::Green) => Color::Green,
            _ => Color::Undefined,
        }
    }
}

#[test]
fn union_keeps_source_edge_data_and_marks_shared_runs_undefined() {
    let edges = overlay_edges(OverlayRule::Union);
    assert_eq!(edges.len(), 8);
    assert_counts(&edges, 3, 3, 2);
}

#[test]
fn intersect_uses_cut_edges_and_marks_shared_runs_undefined() {
    let edges = overlay_edges(OverlayRule::Intersect);
    assert_eq!(edges.len(), 4);
    assert_counts(&edges, 1, 1, 2);
}

#[test]
fn difference_keeps_subject_edges_and_uses_clip_cut_edge() {
    let edges = overlay_edges(OverlayRule::Difference);
    assert_eq!(edges.len(), 4);
    assert_counts(&edges, 3, 1, 0);
}

#[test]
fn inverse_difference_keeps_clip_edges_and_uses_subject_cut_edge() {
    let edges = overlay_edges(OverlayRule::InverseDifference);
    assert_eq!(edges.len(), 4);
    assert_counts(&edges, 1, 3, 0);
}

fn overlay_edges(rule: OverlayRule) -> Vec<DataVectorEdge<Color>> {
    let subj = square(0, 0, 4, 4, Color::Red);
    let clip = square(2, 0, 6, 4, Color::Green);

    let mut overlay = EdgeOverlay::new(subj.len() + clip.len());
    overlay.add_edges(subj, ShapeType::Subject);
    overlay.add_edges(clip, ShapeType::Clip);
    overlay.build_separate_vectors(rule, FillRule::NonZero)
}

fn square(x0: i32, y0: i32, x1: i32, y1: i32, data: Color) -> Vec<InputEdge<Color>> {
    let points = [
        IntPoint::new(x0, y0),
        IntPoint::new(x1, y0),
        IntPoint::new(x1, y1),
        IntPoint::new(x0, y1),
    ];

    points
        .iter()
        .copied()
        .zip(points.iter().copied().cycle().skip(1))
        .take(points.len())
        .map(|(a, b)| InputEdge { a, b, data })
        .collect()
}

fn assert_counts(edges: &[DataVectorEdge<Color>], red: usize, green: usize, undefined: usize) {
    assert_eq!(edges.iter().filter(|e| e.data == Color::Red).count(), red);
    assert_eq!(edges.iter().filter(|e| e.data == Color::Green).count(), green);
    assert_eq!(
        edges.iter().filter(|e| e.data == Color::Undefined).count(),
        undefined
    );
}
