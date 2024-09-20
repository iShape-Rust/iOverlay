use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::i_float::point::IntPoint;

fn main() {

    let subj = [
        // Define the subject polygon (a square)
        IntPoint::new(-10, -10),
        IntPoint::new(-10, 10),
        IntPoint::new(10, 10),
        IntPoint::new(10, -10),
    ].to_vec();

    let clip = [
        // Define the clip polygon (a slightly shifted square)
        IntPoint::new(-5, -5),
        IntPoint::new(-5, 15),
        IntPoint::new(15, 15),
        IntPoint::new(15, -5),
    ].to_vec();

    let shapes = Overlay::with_paths(&[subj], &[clip])
        .into_graph(FillRule::NonZero)
        .extract_shapes(OverlayRule::Union);

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
