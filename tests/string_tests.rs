mod data;
mod util;

#[cfg(test)]
mod tests {
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::string::clip::{ClipRule, IntClip};
    use i_overlay::string::slice::IntSlice;
    use crate::data::overlay::StringTest;
    use crate::util::overlay;
    use crate::util::overlay::JsonPrint;

    fn execute(index: usize) {
        let test = StringTest::load(index);
        let fill_rule = test.fill_rule.unwrap_or(FillRule::EvenOdd);

        let slice = test.body.slice_by_paths(&test.string, fill_rule);
        assert_eq!(true, overlay::is_group_of_shapes_one_of(&slice, &test.slice));

        // let graph = overlay.into_graph_with_solver(fill_rule, Default);
        //
        // let clip = graph.extract_shapes(OverlayRule::Clip);
        // let subject = graph.extract_shapes(OverlayRule::Subject);
        // let difference = graph.extract_shapes(OverlayRule::Difference);
        // let inverse_difference = graph.extract_shapes(OverlayRule::InverseDifference);
        // let intersect = graph.extract_shapes(OverlayRule::Intersect);
        // let union = graph.extract_shapes(OverlayRule::Union);
        // let xor = graph.extract_shapes(OverlayRule::Xor);
        //
        // assert_eq!(true, test_result(&clip, &test.clip));
        // assert_eq!(true, test_result(&subject, &test.subject));
        // assert_eq!(true, test_result(&difference, &test.difference));
        // assert_eq!(true, test_result(&inverse_difference, &test.inverse_difference));
        // assert_eq!(true, test_result(&intersect, &test.intersect));
        // assert_eq!(true, test_result(&union, &test.union));
        // assert_eq!(true, test_result(&xor, &test.xor));
    }

    fn debug_execute_slice(index: usize) {
        let test = StringTest::load(index);
        let fill_rule = test.fill_rule.unwrap_or(FillRule::EvenOdd);
        let slice = test.body.slice_by_paths(&test.string, fill_rule);

        print!("slice: {}", slice.json_print());
    }

    fn debug_execute_clip(index: usize, invert: bool) {
        let test = StringTest::load(index);
        let fill_rule = test.fill_rule.unwrap_or(FillRule::EvenOdd);

        let clip = test.body.clip_paths(&test.string, fill_rule, ClipRule { invert, boundary_included: false });

        print!("clip: {}", clip.json_print());
    }

    #[test]
    fn test_0() {
        execute(0);
    }
    
    #[test]
    fn test_debug() {
        // debug_execute_slice(0);
        debug_execute_clip(0, false);
    }

    #[test]
    fn test_debug_2() {
        // debug_execute_slice(0);
        debug_execute_clip(1, false);
    }
}