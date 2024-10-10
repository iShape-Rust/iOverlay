use i_shape::int::shape::IntShapes;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::Overlay;
use crate::core::overlay_node::OverlayNode;
use crate::segm::segment::{CLIP_BOTH, CLIP_BOTTOM, CLIP_TOP, SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};
use crate::string::graph::StringGraph;
use crate::string::rule::StringRule;

impl Overlay {

    pub fn extra_union(self, fill_rule: FillRule) -> IntShapes {
        let overlay_graph = self.into_graph(fill_rule);
        let nodes: Vec<_> = overlay_graph.nodes.into_iter().map(|node|
            match node {
                OverlayNode::Bridge([a,b]) => {vec![a, b]}
                OverlayNode::Cross(indices) => {indices}
            }
        ).collect();

        let mut links = overlay_graph.links;

        let fill_0 = SUBJ_TOP | CLIP_BOTTOM;
        let fill_1 = SUBJ_BOTTOM | CLIP_TOP;

        for link in links.iter_mut() {
            let fill = link.fill;

            if fill == fill_0 || fill == fill_1 {
                // for touching borders
                link.fill = CLIP_BOTH | SUBJ_BOTH // it's string line mask for string graph!
            } else {
                // convert clip to subject
                let subj = fill & SUBJ_BOTH;
                let clip = (fill & CLIP_BOTH) >> 2;
                link.fill = subj | clip;
            }
        }

        let string_graph = StringGraph {
            solver: Default::default(),
            nodes,
            links,
        };

        string_graph.extract_shapes(StringRule::Slice)
    }
}

#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay::Overlay;

    #[test]
    fn test_2_boxes() {
        let left = vec![
            IntPoint::new(0, 0),
            IntPoint::new(0, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, 0),
        ];

        let right = vec![
            IntPoint::new(10, 0),
            IntPoint::new(10, 10),
            IntPoint::new(20, 10),
            IntPoint::new(20, 0),
        ];

        let overlay = Overlay::with_paths(&[left.clone()], &[right.clone()]);
        let result = overlay.extra_union(FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[0][0], left);
        assert_eq!(result[1][0], right);
    }
}