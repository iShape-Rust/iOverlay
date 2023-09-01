use i_float::fix_vec::FixVec;
use i_overlay::{layout::overlay::Overlay, fill::shape_type::ShapeType, bool::fill_rule::FillRule};

fn main() {
    let mut overlay = Overlay::new(1);
        
    let subj = [
        FixVec::new_number(-10, -10),
        FixVec::new_number(-10,  10),
        FixVec::new_number( 10,  10),
        FixVec::new_number( 10, -10)
    ];

    let clip = [
        FixVec::new_number(-5, -5),
        FixVec::new_number(-5, 15),
        FixVec::new_number(15, 15),
        FixVec::new_number(15, -5)
    ];


    overlay.add_path(subj.to_vec(), ShapeType::SUBJECT);
    overlay.add_path(clip.to_vec(), ShapeType::CLIP);
    let graph = overlay.build_graph();

    let shapes = graph.extract_shapes(FillRule::Union);

    println!("shapes count: {}", shapes.len());

    if shapes.len() > 0 {
        let contour = shapes[0].contour();
        println!("shape 0 contour: ");
        for p in contour {
            let x = p.x.float();
            let y = p.x.float();
            println!("({}, {})", x, y);
        }
        
    }
}
