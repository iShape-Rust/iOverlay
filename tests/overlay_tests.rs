mod data;
mod util;

#[cfg(test)]
mod tests {
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::Overlay;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::Solver;
    use crate::data::overlay::BooleanTest;
    use crate::util::overlay;
    use crate::util::overlay::JsonPrint;

    const SOLVERS: [Solver; 3] = [
        Solver::LIST,
        Solver::TREE,
        Solver::AUTO
    ];

    fn execute(index: usize) {
        let test = BooleanTest::load(index);
        let fill_rule = test.fill_rule.unwrap_or(FillRule::EvenOdd);
        for solver in SOLVERS {
            let overlay = Overlay::with_contours(&test.subj_paths, &test.clip_paths);
            let graph = overlay.into_graph_with_solver(fill_rule, solver);

            let clip = graph.extract_shapes(OverlayRule::Clip);
            let subject = graph.extract_shapes(OverlayRule::Subject);
            let difference = graph.extract_shapes(OverlayRule::Difference);
            let inverse_difference = graph.extract_shapes(OverlayRule::InverseDifference);
            let intersect = graph.extract_shapes(OverlayRule::Intersect);
            let union = graph.extract_shapes(OverlayRule::Union);
            let xor = graph.extract_shapes(OverlayRule::Xor);

            assert_eq!(true, overlay::is_group_of_shapes_one_of(&clip, &test.clip));
            assert_eq!(true, overlay::is_group_of_shapes_one_of(&subject, &test.subject));
            assert_eq!(true, overlay::is_group_of_shapes_one_of(&difference, &test.difference));
            assert_eq!(true, overlay::is_group_of_shapes_one_of(&inverse_difference, &test.inverse_difference));
            assert_eq!(true, overlay::is_group_of_shapes_one_of(&intersect, &test.intersect));
            assert_eq!(true, overlay::is_group_of_shapes_one_of(&union, &test.union));
            assert_eq!(true, overlay::is_group_of_shapes_one_of(&xor, &test.xor));
        }
    }

    fn debug_execute(index: usize, overlay_rule: OverlayRule, solver: Solver) {
        let test = BooleanTest::load(index);
        let fill_rule = test.fill_rule.unwrap_or(FillRule::NonZero);
        let overlay = Overlay::with_contours(&test.subj_paths, &test.clip_paths);
        let graph = overlay.into_graph_with_solver(fill_rule, solver);
        let result = graph.extract_shapes(overlay_rule);

        println!("result: {}", result.json_print());
        match overlay_rule {
            OverlayRule::Subject => {
                assert_eq!(true, overlay::is_group_of_shapes_one_of(&result, &test.subject));
            }
            OverlayRule::Clip => {
                assert_eq!(true, overlay::is_group_of_shapes_one_of(&result, &test.clip));
            }
            OverlayRule::Intersect => {
                assert_eq!(true, overlay::is_group_of_shapes_one_of(&result, &test.intersect));
            }
            OverlayRule::Union => {
                assert_eq!(true, overlay::is_group_of_shapes_one_of(&result, &test.union));
            }
            OverlayRule::Difference => {
                assert_eq!(true, overlay::is_group_of_shapes_one_of(&result, &test.difference));
            }
            OverlayRule::InverseDifference => {
                assert_eq!(true, overlay::is_group_of_shapes_one_of(&result, &test.inverse_difference));
            }
            OverlayRule::Xor => {
                assert_eq!(true, overlay::is_group_of_shapes_one_of(&result, &test.xor));
            }
        }
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
    fn test_12() {
        execute(12);
    }

    #[test]
    fn test_13() {
        execute(13);
    }

    #[test]
    fn test_14() {
        execute(14);
    }

    #[test]
    fn test_15() {
        execute(15);
    }

    #[test]
    fn test_16() {
        execute(16);
    }

    #[test]
    fn test_17() {
        execute(17);
    }

    #[test]
    fn test_18() {
        execute(18);
    }

    #[test]
    fn test_19() {
        execute(19);
    }

    #[test]
    fn test_20() {
        execute(20);
    }

    #[test]
    fn test_21() {
        execute(21);
    }

    #[test]
    fn test_22() {
        execute(22);
    }

    #[test]
    fn test_23() {
        execute(23);
    }

    #[test]
    fn test_24() {
        execute(24);
    }

    #[test]
    fn test_25() {
        execute(25);
    }

    #[test]
    fn test_26() {
        execute(26);
    }

    #[test]
    fn test_27() {
        execute(27);
    }

    #[test]
    fn test_28() {
        execute(28);
    }

    #[test]
    fn test_29() {
        execute(29);
    }

    #[test]
    fn test_30() {
        execute(30);
    }

    #[test]
    fn test_31() {
        execute(31);
    }

    #[test]
    fn test_32() {
        execute(32);
    }

    #[test]
    fn test_33() {
        execute(33);
    }

    #[test]
    fn test_34() {
        execute(34);
    }

    #[test]
    fn test_35() {
        execute(35);
    }

    #[test]
    fn test_36() {
        execute(36);
    }

    #[test]
    fn test_37() {
        execute(37);
    }

    #[test]
    fn test_38() {
        execute(38);
    }

    #[test]
    fn test_39() {
        execute(39);
    }

    #[test]
    fn test_40() {
        execute(40);
    }

    #[test]
    fn test_41() {
        execute(41);
    }

    #[test]
    fn test_42() {
        execute(42);
    }

    #[test]
    fn test_43() {
        execute(43);
    }

    #[test]
    fn test_44() {
        execute(44);
    }

    #[test]
    fn test_45() {
        execute(45);
    }

    #[test]
    fn test_46() {
        execute(46);
    }

    #[test]
    fn test_47() {
        execute(47);
    }

    #[test]
    fn test_48() {
        execute(48);
    }

    #[test]
    fn test_49() {
        execute(49);
    }

    #[test]
    fn test_50() {
        execute(50);
    }

    #[test]
    fn test_51() {
        execute(51);
    }

    #[test]
    fn test_52() {
        execute(52);
    }

    #[test]
    fn test_53() {
        execute(53);
    }

    #[test]
    fn test_54() {
        execute(54);
    }

    #[test]
    fn test_55() {
        execute(55);
    }

    #[test]
    fn test_56() {
        execute(56);
    }

    #[test]
    fn test_57() {
        execute(57);
    }

    #[test]
    fn test_58() {
        execute(58);
    }

    #[test]
    fn test_59() {
        execute(59);
    }

    #[test]
    fn test_60() {
        execute(60);
    }

    #[test]
    fn test_61() {
        execute(61);
    }

    #[test]
    fn test_62() {
        execute(62);
    }

    #[test]
    fn test_63() {
        execute(63);
    }

    #[test]
    fn test_64() {
        execute(64);
    }

    #[test]
    fn test_65() {
        execute(65);
    }

    #[test]
    fn test_66() {
        execute(66);
    }

    #[test]
    fn test_67() {
        execute(67);
    }

    #[test]
    fn test_68() {
        execute(68);
    }

    #[test]
    fn test_69() {
        execute(69);
    }

    #[test]
    fn test_70() {
        execute(70);
    }

    #[test]
    fn test_71() {
        execute(71);
    }

    #[test]
    fn test_72() {
        execute(72);
    }

    #[test]
    fn test_73() {
        execute(73);
    }

    #[test]
    fn test_74() {
        execute(74);
    }

    #[test]
    fn test_75() {
        execute(75);
    }

    #[test]
    fn test_76() {
        execute(76);
    }

    #[test]
    fn test_77() {
        execute(77);
    }

    #[test]
    fn test_78() {
        execute(78);
    }

    #[test]
    fn test_79() {
        execute(79);
    }

    #[test]
    fn test_80() {
        execute(80);
    }

    #[test]
    fn test_81() {
        execute(81);
    }

    #[test]
    fn test_82() {
        execute(82);
    }

    #[test]
    fn test_83() {
        execute(83);
    }

    #[test]
    fn test_84() {
        execute(84);
    }

    #[test]
    fn test_85() {
        execute(85);
    }

    #[test]
    fn test_86() {
        execute(86);
    }

    #[test]
    fn test_87() {
        execute(87);
    }

    #[test]
    fn test_88() {
        execute(88);
    }

    #[test]
    fn test_89() {
        execute(89);
    }

    #[test]
    fn test_90() {
        execute(90);
    }

    #[test]
    fn test_91() {
        execute(91);
    }

    #[test]
    fn test_92() {
        execute(92);
    }

    #[test]
    fn test_93() {
        execute(93);
    }

    #[test]
    fn test_94() {
        execute(94);
    }

    #[test]
    fn test_95() {
        execute(95);
    }

    #[test]
    fn test_96() {
        execute(96);
    }

    #[test]
    fn test_97() {
        execute(97);
    }

    #[test]
    fn test_98() {
        execute(98);
    }

    #[test]
    fn test_99() {
        execute(99);
    }

    #[test]
    fn test_100() {
        execute(100);
    }

    #[test]
    fn test_101() {
        execute(101);
    }

    #[test]
    fn test_102() {
        execute(102);
    }

    #[test]
    fn test_103() {
        execute(103);
    }

    #[test]
    fn test_104() {
        execute(104);
    }

    #[test]
    fn test_105() {
        execute(105);
    }

    #[test]
    fn test_106() {
        execute(106);
    }

    #[test]
    fn test_107() {
        execute(107);
    }

    #[test]
    fn test_108() {
        execute(108);
    }

    #[test]
    fn test_109() {
        execute(109);
    }

    #[test]
    fn test_110() {
        execute(110);
    }

    #[test]
    fn test_111() {
        execute(111);
    }

    #[test]
    fn test_112() {
        execute(112);
    }

    #[test]
    fn test_113() {
        execute(113);
    }

    #[test]
    fn test_114() {
        execute(114);
    }

    #[test]
    fn test_115() {
        execute(115);
    }

    #[test]
    fn test_116() {
        execute(116);
    }

    #[test]
    fn test_117() {
        execute(117);
    }

    #[test]
    fn test_118() {
        execute(118);
    }

    #[test]
    fn test_119() {
        execute(119);
    }

    #[test]
    fn test_120() {
        execute(120);
    }

    #[test]
    fn test_121() {
        execute(121);
    }

    #[test]
    fn test_122() {
        execute(122);
    }

    #[test]
    fn test_123() {
        execute(123);
    }

    #[test]
    fn test_124() {
        execute(124);
    }

    #[test]
    fn test_125() {
        execute(125);
    }

    #[test]
    fn test_126() {
        execute(126);
    }

    #[test]
    fn test_127() {
        execute(127);
    }

    #[test]
    fn test_128() {
        execute(128);
    }

    #[test]
    fn test_129() {
        execute(129);
    }

    #[test]
    fn test_130() {
        execute(130);
    }

    #[test]
    fn test_131() {
        execute(131);
    }

    #[test]
    fn test_132() {
        execute(132);
    }

    #[test]
    fn test_133() {
        execute(133);
    }

    #[test]
    fn test_134() {
        execute(134);
    }

    #[test]
    fn test_135() {
        execute(135);
    }

    // #[test]
    // fn test_debug() {
    //     debug_execute(138, OverlayRule::Subject, Solver::LIST);
    // }
}