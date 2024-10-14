use crate::string::rule::StringRule;
use crate::string::graph::StringGraph;
use crate::geom::segment::{CLIP_BOTH, SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};

impl StringGraph {
    #[inline(always)]
    pub(super) fn filter(&self, ext_rule: StringRule) -> Vec<u8> {
        match ext_rule {
            StringRule::Slice => {
                self.filter_slice()
            }
        }
    }

    #[inline]
    fn filter_slice(&self) -> Vec<u8> {
        self.links.iter().map(|link| {
            let fill = link.fill;
            let subj = fill & SUBJ_BOTH;
            let one_side_subj = subj == SUBJ_TOP || subj == SUBJ_BOTTOM;

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
    }
}