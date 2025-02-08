// use std::f64::consts::PI;
// use i_float::float::compatible::FloatPointCompatible;
// use i_float::float::number::FloatNumber;
// use crate::buffering::rotator::Rotator;
// use crate::buffering::stroke::style::{LineCap, StrokeStyle};
//
// pub(crate) struct RoundBuilder<T, P> {
//     pub(super) min_dot_product: T,
//     pub(super) count_for_pi: usize,
//     pub(super) rotator: Rotator<T, P>,
// }
//
// impl<T: FloatNumber, P: FloatPointCompatible<T>> RoundBuilder<T, P> {
//     pub(super) fn new(style: StrokeStyle<T>) -> Option<Self> {
//         if style.begin_cap == LineCap::Round || style.end_cap == LineCap::Round {
//             return None
//         }
//
//         let count_for_pi = ((style.round_limit / style.width).to_f64().round() as usize).min(2);
//         let delta_angle = PI / count_for_pi as f64;
//         let (sn, cs) = delta_angle.to_f64().sin_cos();
//         let sin = T::from_float(sn);
//         let cos = T::from_float(cs);
//         let rotator = Rotator::new(sin, cos);
//
//         Some(Self {
//             min_dot_product: cos,
//             count_for_pi,
//             rotator,
//         })
//     }
// }