use i_overlay::bool::fill_rule::FillRule;
use i_overlay::layout::overlay::Overlay;


#[cfg(test)]
mod tests {
    

    use i_float::fix_vec::FixVec;
    use i_overlay::fill::shape_type::ShapeType;

    use super::*;

    #[test]
    fn test_brute_0() {
        
        let clip = create_star(100.0, 200.0, 5, 0.0);
        
        let mut angle = 0.0;

        for i in 0..20_000 {
            if i % 1000 == 0 {
                println!("test index: {}", i);
            }

            let subj = create_star(100.0, 200.0, 7, angle);

            let mut overlay = Overlay::new(1);
    
            overlay.add_path(subj.to_vec(), ShapeType::SUBJECT);
            overlay.add_path(clip.to_vec(), ShapeType::CLIP);
    
            let graph = overlay.build_graph();
            let shapes = graph.extract_shapes(FillRule::Union);

            assert_eq!(shapes.len() != 0, true);

            angle += 0.0025;
        }
    }

    #[test]
    fn test_single_0() {
        
        let clip = create_star(100.0, 200.0, 5, 0.0);
        let angle = 41.634999999998378;
        let subj = create_star(100.0, 200.0, 7, angle);

        let mut overlay = Overlay::new(1);

        overlay.add_path(subj.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(clip.to_vec(), ShapeType::CLIP);

        let graph = overlay.build_graph();
        let shapes = graph.extract_shapes(FillRule::Union);

        assert_eq!(shapes.len() != 0, true);
    }

    fn create_star(r0: f64, r1: f64, count: usize, angle: f64) -> Vec<FixVec> {
        let da = std::f64::consts::PI / (count as f64);
        let mut a = angle;
        let x0: f64 = 400.0;
        let y0: f64 = 400.0;
      
        let mut points = Vec::new();
        for _ in 0..count {
            let sc0 = a.sin_cos();

            let xr0 = r0 * sc0.1 + x0;
            let yr0 = r0 * sc0.0 + y0;
            a += da;

            let sc1 = a.sin_cos();

            let xr1 = r1 * sc1.1 + x0;
            let yr1 = r1 * sc1.0 + y0;
        
            a += da;

            let p0 = FixVec::new_f64(xr0, yr0);
            let p1 = FixVec::new_f64(xr1, yr1);


            points.push(p0);
            points.push(p1);
        }
      
        points
      }

}