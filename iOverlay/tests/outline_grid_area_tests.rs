use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::mesh::float::outline::offset::OutlineOffset;
use i_overlay::mesh::float::style::OutlineStyle;

fn area(shapes: &[Vec<Vec<[f64; 2]>>]) -> f64 {
    shapes
        .iter()
        .flatten()
        .map(|path| {
            path.iter()
                .zip(path.iter().cycle().skip(1))
                .map(|(a, b)| 0.5 * (a[0] * b[1] - a[1] * b[0]))
                .sum::<f64>()
        })
        .sum()
}

#[test]
fn zero_outline_preserves_half_grid_cell_hole() {
    // Symmetric bounds put the adapter origin at (0,0), so every vertex is
    // exactly on the fixed grid. The clockwise triangle has double area -1.
    let shape = vec![
        vec![[-10.0, -10.0], [10.0, -10.0], [10.0, 10.0], [-10.0, 10.0]],
        vec![[0.0, 0.0], [0.0, 1.0], [1.0, 0.0]],
    ];
    let result = shape.outline_fixed_scale(&OutlineStyle::new(0.0), 1.0).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0].len(),
        2,
        "a nonzero-area hole must survive zero offset: {result:?}"
    );
    assert_eq!(area(&result), 399.5);
}

#[test]
fn zero_outline_preserves_half_grid_cell_components() {
    let shape = vec![
        vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
        vec![[0.0, 0.0], [-1.0, 0.0], [0.0, -1.0]],
    ];
    let result = shape.outline_fixed_scale(&OutlineStyle::new(0.0), 1.0).unwrap();
    assert_eq!(
        area(&result),
        1.0,
        "two nondegenerate triangles must survive: {result:?}"
    );
}

#[test]
fn outward_outline_expands_half_grid_cell_components() {
    let shape = vec![
        vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
        vec![[0.0, 0.0], [-1.0, 0.0], [0.0, -1.0]],
    ];
    let result = shape.outline_fixed_scale(&OutlineStyle::new(2.0), 1.0).unwrap();
    assert!(
        area(&result) > 1.0,
        "outward offset must expand the triangles, not discard them: {result:?}"
    );
}

#[test]
fn integer_overlay_preserves_half_grid_cell_contours() {
    let triangle = vec![IntPoint::new(0, 0), IntPoint::new(1, 0), IntPoint::new(0, 1)];
    let result = Overlay::with_contour(&triangle, &[]).overlay(OverlayRule::Subject, FillRule::Positive);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0][0].len(), 3);
    let mut hole = triangle;
    hole.reverse();
    let shape = vec![
        vec![
            IntPoint::new(-10, -10),
            IntPoint::new(10, -10),
            IntPoint::new(10, 10),
            IntPoint::new(-10, 10),
        ],
        hole,
    ];
    let result = Overlay::with_contours(&shape, &[]).overlay(OverlayRule::Subject, FillRule::Positive);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].len(), 2);
}
