use crate::extension::rule::ExtRule;
use crate::extension::unstable_graph::UnstableGraph;
use crate::segm::segment::{CLIP_BOTH, SUBJ_BOTH};

impl UnstableGraph {
    #[inline(always)]
    pub(super) fn filter(&self, ext_rule: ExtRule) -> Vec<u8> {
        match ext_rule {
            ExtRule::Slice => {
                self.filter_slice()
            }
        }
    }

    fn filter_slice(&self) -> Vec<u8> {
        [].to_vec()
        /*
        self.links.iter().enumerate().map(|(index, link)| {
            if !self.node(link.a.id).is_contain(index) {
                // this link is a leaf and it is removed
                return 0;
            }
            let fill = link.fill;
            let subj = fill & SUBJ_BOTH;
            let one_side_subj = subj != 0 && subj != SUBJ_BOTH;

            if one_side_subj {
                1
            } else if fill & CLIP_BOTH != 0 && subj == SUBJ_BOTH {
                // ony edges inside subj
                // slice edge, we must visit it twice
                2
            } else {
                0
            }
        }).collect()
        */
    }
}