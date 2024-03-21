use i_float::{fix_float::FixMath, fix_vec::FixVec};
use i_overlay::{
    bool::{fill_rule::FillRule, overlay_rule::OverlayRule},
    layout::overlay::{Overlay, ShapeType},
};

fn main() {
    let mut overlay = Overlay::new(1);

    let subj = [
        FixVec::new_number(-10, -10),
        FixVec::new_number(-10, 10),
        FixVec::new_number(10, 10),
        FixVec::new_number(10, -10),
    ];

    let clip = [
        FixVec::new_number(-5, -5),
        FixVec::new_number(-5, 15),
        FixVec::new_number(15, 15),
        FixVec::new_number(15, -5),
    ];

    overlay.add_path(&subj.to_vec(), ShapeType::Subject);
    overlay.add_path(&clip.to_vec(), ShapeType::Clip);
    let graph = overlay.build_graph(FillRule::NonZero);

    let shapes = graph.extract_shapes(OverlayRule::Union);

    println!("shapes count: {}", shapes.len());

    if shapes.len() > 0 {
        let contour = shapes[0].contour();
        println!("shape 0 contour: ");
        for p in contour {
            let x = p.x.f32();
            let y = p.x.f32();
            println!("({}, {})", x, y);
        }
    }
}
