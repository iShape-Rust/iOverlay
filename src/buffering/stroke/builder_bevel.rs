use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::buffering::stroke::builder::SectionBuilder;
use crate::buffering::stroke::builder_cap::CapBuilder;
use crate::buffering::stroke::builder_round::RoundBuilder;
use crate::buffering::stroke::section::Section;
use crate::segm::segment::Segment;
use crate::segm::winding_count::{ShapeCountBoolean, WindingCount};
use crate::buffering::stroke::style::StrokeStyle;

pub(crate) struct BevelBuilder<T, P, C> {
    pub(super) radius: T,
    pub(super) cap_builder: C,
}

impl<T: FloatNumber, P: FloatPointCompatible<T>, C: CapBuilder<T, P>> BevelBuilder<T, P, C> {
    fn new(style: StrokeStyle<T>) -> Self {
        let radius = T::from_float(0.5) * style.width;
        let round_builder = RoundBuilder::new(style);

        Self {
            radius,
            cap_builder
        }
    }

    fn append_segments(
        &self,
        iter: impl Iterator<Item=&'_ [P]>,
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        if is_closed_path {
            for path in iter {
                let n = path.len();
                if n < 2 { continue; }
                let start = Section::section(self.radius, &path[n - 1], &path[0]);

                let mut s0 = start.clone();
                segments.add_section(&s0, adapter);
                for [a, b] in path.windows(2) {
                    let s1 = Section::section(self.radius, a, b);
                    segments.add_bevel_join(&s0, &s1, adapter);
                    segments.add_section(&s1, adapter);
                    s0 = s1;
                }

                segments.add_bevel_join(&s0, &start, adapter);
            }
        } else {
            for path in iter {
                let mut s0 = Section::section(self.radius, path[0], path[1]);


                segments.add_section(&s0, adapter);

                for [a, b] in path.windows(2).skip(1).take(path.len() - 2) {
                    let s1 = Section::section(self.radius, a, b);
                    segments.add_bevel_join(&s0, &s1, adapter);
                    segments.add_section(&s1, adapter);
                    s0 = s1;
                }

                segments.add_bevel_join(&s0, &start, adapter);
            }
        }
    }
}


