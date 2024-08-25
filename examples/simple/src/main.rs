use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::ShapeType;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::f64::overlay::F64Overlay;
use i_overlay::i_float::f64_point::F64Point;

fn main() {

    let subj = [
        F64Point::new(-10.0, -10.0),
        F64Point::new(-10.0, 10.0),
        F64Point::new(10.0, 10.0),
        F64Point::new(10.0, -10.0),
    ].to_vec();

    let clip = [
        F64Point::new(-5.0, -5.0),
        F64Point::new(-5.0, 15.0),
        F64Point::new(15.0, 15.0),
        F64Point::new(15.0, -5.0),
    ].to_vec();

    let mut overlay = F64Overlay::new();

    overlay.add_path(subj, ShapeType::Subject);
    overlay.add_path(clip, ShapeType::Clip);

    let graph = overlay.into_graph(FillRule::NonZero);
    let shapes = graph.extract_shapes(OverlayRule::Union);

    println!("shapes count: {}", shapes.len());

    if shapes.len() > 0 {
        let contour = &shapes[0][0];
        println!("shape 0 contour: ");
        for p in contour {
            let x = p.x;
            let y = p.y;
            println!("({}, {})", x, y);
        }
    }
}
