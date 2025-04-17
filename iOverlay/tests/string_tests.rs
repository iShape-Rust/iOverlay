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

        let clip_direct = test.body.clip_paths(&test.string, fill_rule, ClipRule { invert: false, boundary_included: false });
        assert_eq!(true, overlay::is_paths_one_of(&clip_direct, &test.clip_direct));

        let clip_invert = test.body.clip_paths(&test.string, fill_rule, ClipRule { invert: true, boundary_included: false });
        assert_eq!(true, overlay::is_paths_one_of(&clip_invert, &test.clip_invert));
    }

    fn debug_execute_slice(index: usize) {
        let test = StringTest::load(index);
        let fill_rule = test.fill_rule.unwrap_or(FillRule::EvenOdd);
        let slice = test.body.slice_by_paths(&test.string, fill_rule);

        println!("slice: {}", slice.json_print());
    }

    fn debug_execute_clip(index: usize, invert: bool) {
        let test = StringTest::load(index);
        let fill_rule = test.fill_rule.unwrap_or(FillRule::EvenOdd);

        let clip = test.body.clip_paths(&test.string, fill_rule, ClipRule { invert, boundary_included: false });

        println!("clip {}: {}", invert, clip.json_print());
    }

    #[test]
    fn test_0() {
        execute(0);
    }

    #[test]
    fn test_1() {
        execute(1);
    }

    #[test]
    fn test_2() {
        execute(2);
    }

    #[test]
    fn test_3() {
        execute(3);
    }

    #[test]
    fn test_4() {
        execute(4);
    }

    #[test]
    fn test_5() {
        execute(5);
    }

    #[test]
    fn test_6() {
        execute(6);
    }

    #[test]
    fn test_7() {
        execute(7);
    }

    #[test]
    fn test_8() {
        execute(8);
    }

    #[test]
    fn test_9() {
        execute(9);
    }

    #[test]
    fn test_10() {
        execute(10);
    }

    #[test]
    fn test_11() {
        execute(11);
    }

    #[test]
    fn test_debug() {
        let index = 11;
        debug_execute_slice(index);
        debug_execute_clip(index, false);
        debug_execute_clip(index, true);
    }
}