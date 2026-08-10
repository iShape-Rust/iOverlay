use crate::mesh::variable_stroke::builder_cap::RoundFanBuilder;
use crate::mesh::variable_stroke::builder_join::RoundJoinBuilder;
use crate::mesh::variable_stroke::section::{Section, SectionKind};
use crate::mesh::variable_stroke::style::{StrokeVertex, VariableStrokeStyle};
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_shape::int::area::Area;

pub(super) struct VariableStrokeBuilder<T: FloatNumber> {
    fan: RoundFanBuilder<T>,
    join: RoundJoinBuilder<T>,
}

impl<T: FloatNumber> VariableStrokeBuilder<T> {
    pub(super) fn new(style: VariableStrokeStyle<T>) -> Self {
        let style = style.normalized();
        Self {
            fan: RoundFanBuilder::new(style.round_angle),
            join: RoundJoinBuilder::new(style.round_angle),
        }
    }

    pub(super) fn build<P, I>(
        &self,
        path: &[StrokeVertex<P>],
        is_closed: bool,
        adapter: &FloatPointAdapter<P, I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        if path.len() < 2 {
            return;
        }

        let edge_count = if is_closed { path.len() } else { path.len() - 1 };
        let mut sections = Vec::with_capacity(edge_count);
        for i in 0..edge_count {
            let j = if i + 1 == path.len() { 0 } else { i + 1 };
            sections.push(Section::classify(&path[i], &path[j]));
        }

        let mut output = PolygonBuilder { adapter, segments };

        for section in sections.iter() {
            match section {
                SectionKind::Regular(section) => output.add_body(section),
                SectionKind::Covered(section) => self.fan.add_covered_cap(section, &mut output),
                SectionKind::Coincident | SectionKind::Empty => {}
            }
        }

        for i in 0..edge_count {
            let SectionKind::Regular(section) = &sections[i] else {
                continue;
            };

            let previous = if i > 0 {
                sections.get(i - 1)
            } else if is_closed {
                sections.last()
            } else {
                None
            };

            match previous {
                Some(SectionKind::Regular(previous)) => {
                    self.join.add(previous, section, &self.fan, &mut output)
                }
                Some(SectionKind::Covered(_)) => {}
                Some(SectionKind::Coincident | SectionKind::Empty) | None => {
                    self.fan.add_start_cap(section, &mut output)
                }
            }

            let next = if i + 1 < edge_count {
                sections.get(i + 1)
            } else if is_closed {
                sections.first()
            } else {
                None
            };

            match next {
                Some(SectionKind::Regular(_) | SectionKind::Covered(_)) => {}
                Some(SectionKind::Coincident | SectionKind::Empty) | None => {
                    self.fan.add_end_cap(section, &mut output)
                }
            }
        }
    }

    pub(super) fn capacity(&self, paths_count: usize, points_count: usize, is_closed: bool) -> usize {
        let body = points_count.saturating_sub(paths_count) * 6;
        let caps = if is_closed { 0 } else { 2 * paths_count };
        body + self.fan.capacity() * (points_count + caps)
    }

    pub(super) fn additional_offset(&self, max_radius: T) -> T {
        T::from_float(1.1) * max_radius
    }
}

pub(super) struct PolygonBuilder<'a, P: FloatPointCompatible, I: IntNumber> {
    adapter: &'a FloatPointAdapter<P, I>,
    segments: &'a mut Vec<Segment<ShapeCountBoolean, I>>,
}

impl<P: FloatPointCompatible, I: IntNumber> PolygonBuilder<'_, P, I> {
    #[inline]
    pub(super) fn add_body(&mut self, section: &Section<P>) {
        self.add_polygon(&[
            section.a,
            section.a_left,
            section.b_left,
            section.b,
            section.b_right,
            section.a_right,
        ]);
    }

    #[inline]
    pub(super) fn add_triangle(&mut self, center: P, a: P, b: P) {
        self.add_polygon(&[center, a, b]);
    }

    pub(super) fn add_polygon(&mut self, points: &[P]) {
        let mut contour = Vec::with_capacity(points.len());
        for point in points {
            let int_point = self.adapter.float_to_int(point);
            if contour.last() != Some(&int_point) {
                contour.push(int_point);
            }
        }

        if contour.len() > 1 && contour.first() == contour.last() {
            contour.pop();
        }
        if contour.len() < 3 {
            return;
        }

        let area = contour.as_slice().area_two();
        if area == I::Wide::ZERO {
            return;
        }
        if area < I::Wide::ZERO {
            contour.reverse();
        }

        let mut a = *contour.last().unwrap();
        for &b in contour.iter() {
            if a != b {
                self.segments.push(Segment::subject(a, b));
            }
            a = b;
        }
    }
}
