// use std::iter;
// use i_float::float::compatible::FloatPointCompatible;
// use i_float::float::number::FloatNumber;
//
// pub trait ContourSource<P, T>
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'a>: Iterator<Item=&'a [P]>
//     where
//         P: 'a,
//         Self: 'a;
//     fn iter_contours(&self) -> Self::ContourIter<'_>;
// }
//
// // Contour
//
// // Slice
// impl<'a, P, T> ContourSource<P, T> for &'a [P]
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'b> = iter::Once<&'b [P]>
//     where
//         P: 'b,
//         Self: 'b;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         iter::once(self)
//     }
// }
//
// // Array
// impl<P, T> ContourSource<P, T> for [P]
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'a> = iter::Once<&'a [P]>
//     where
//         P: 'a;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         iter::once(self.as_ref())
//     }
// }
//
// // Vec
// impl<P, T> ContourSource<P, T> for Vec<P>
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'a> = iter::Once<&'a [P]>
//     where
//         P: 'a;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         iter::once(self)
//     }
// }
// // ____________
//
// // Contours
// impl<'a, P, T> ContourSource<P, T> for &'a [Vec<P>]
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'b> = iter::Map<std::slice::Iter<'b, Vec<P>>, fn(&'b Vec<P>) -> &'b [P]>
//     where
//         P: 'b,
//         Self: 'b;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         self.iter().map(|v| &v[..])
//     }
// }
//
// impl<P, T> ContourSource<P, T> for [Vec<P>]
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'a> = iter::Map<std::slice::Iter<'a, Vec<P>>, fn(&'a Vec<P>) -> &'a [P]>
//     where
//         P: 'a;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         self.iter().map(|v| &v[..])
//     }
// }
//
// impl<P, T> ContourSource<P, T> for Vec<Vec<P>>
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'a> = iter::Map<std::slice::Iter<'a, Vec<P>>, fn(&'a Vec<P>) -> &'a [P]>
//     where
//         P: 'a;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         self.iter().map(|v| &v[..])
//     }
// }
//
// // ____________
//
// // Shapes
//
// impl<'a, P, T> ContourSource<P, T> for &'a [Vec<Vec<P>>]
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'b> = iter::FlatMap<
//         std::slice::Iter<'b, Vec<Vec<P>>>,
//         iter::Map<std::slice::Iter<'b, Vec<P>>, fn(&'b Vec<P>) -> &'b [P]>,
//         fn(&'b Vec<Vec<P>>) -> iter::Map<std::slice::Iter<'b, Vec<P>>, fn(&'b Vec<P>) -> &'b [P]>
//     >
//     where
//         P: 'b,
//         Self: 'b;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         self.iter().flat_map(|inner| inner.iter().map(|v| &v[..]))
//     }
// }
//
// impl<P, T> ContourSource<P, T> for [Vec<Vec<P>>]
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'a> = iter::FlatMap<
//         std::slice::Iter<'a, Vec<Vec<P>>>,
//         iter::Map<std::slice::Iter<'a, Vec<P>>, fn(&'a Vec<P>) -> &'a [P]>,
//         fn(&'a Vec<Vec<P>>) -> iter::Map<std::slice::Iter<'a, Vec<P>>, fn(&'a Vec<P>) -> &'a [P]>
//     >
//     where
//         P: 'a;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         self.iter().flat_map(|inner| inner.iter().map(|v| &v[..]))
//     }
// }
//
// impl<P, T> ContourSource<P, T> for Vec<Vec<Vec<P>>>
// where
//     P: FloatPointCompatible<T>,
//     T: FloatNumber,
// {
//     type ContourIter<'a> = iter::FlatMap<
//         std::slice::Iter<'a, Vec<Vec<P>>>,
//         iter::Map<std::slice::Iter<'a, Vec<P>>, fn(&'a Vec<P>) -> &'a [P]>,
//         fn(&'a Vec<Vec<P>>) -> iter::Map<std::slice::Iter<'a, Vec<P>>, fn(&'a Vec<P>) -> &'a [P]>
//     >
//     where
//         P: 'a;
//
//     fn iter_contours(&self) -> Self::ContourIter<'_> {
//         self.iter().flat_map(|inner| inner.iter().map(|v| &v[..]))
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use i_float::float::compatible::FloatPointCompatible;
//     use crate::float::old_source::ContourSource;
//
//     #[test]
//     fn test_contour_array() {
//         let contour = [
//             [0.0, 0.0],
//             [0.0, 1.0],
//             [1.0, 1.0],
//             [1.0, 0.0]
//         ];
//
//         let count = contour.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 4);
//     }
//
//     #[test]
//     fn test_contour_slice() {
//         let contour = vec![
//             [0.0, 0.0],
//             [0.0, 1.0],
//             [1.0, 1.0],
//             [1.0, 0.0]
//         ];
//
//         let count = contour.as_slice().iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 4);
//     }
//
//     #[test]
//     fn test_contour_vec() {
//         let contour = vec![
//             [0.0, 0.0],
//             [0.0, 1.0],
//             [1.0, 1.0],
//             [1.0, 0.0]
//         ];
//
//         let count = contour.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 4);
//     }
//
//     #[test]
//     fn test_contours_array() {
//         let contours = [
//             [
//                 [0.0, 0.0],
//                 [0.0, 1.0],
//                 [1.0, 1.0],
//                 [1.0, 0.0]
//             ].to_vec(),
//             [
//                 [0.0, 0.0],
//                 [0.0, 1.0],
//                 [1.0, 1.0],
//                 [1.0, 0.0]
//             ].to_vec()
//         ];
//
//         let count = contours.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 8);
//     }
//
//     #[test]
//     fn test_contours_slice() {
//         let contours = [
//             [
//                 [0.0, 0.0],
//                 [0.0, 1.0],
//                 [1.0, 1.0],
//                 [1.0, 0.0]
//             ].to_vec(),
//             [
//                 [0.0, 0.0],
//                 [0.0, 1.0],
//                 [1.0, 1.0],
//                 [1.0, 0.0]
//             ].to_vec()
//         ];
//
//         let count = contours.as_slice().iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 8);
//     }
//
//     #[test]
//     fn test_contours_vec() {
//         let contours = vec![
//             vec![
//                 [0.0, 0.0],
//                 [0.0, 1.0],
//                 [1.0, 1.0],
//                 [1.0, 0.0]
//             ],
//             vec![
//                 [0.0, 0.0],
//                 [0.0, 1.0],
//                 [1.0, 1.0],
//                 [1.0, 0.0]
//             ]
//         ];
//
//         let count = contours.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 8);
//     }
//
//     #[test]
//     fn test_shapes_array() {
//         let shapes = [
//             [
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec(),
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec()
//             ].to_vec(),
//             [
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec()
//             ].to_vec()
//         ];
//
//         let count = shapes.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 12);
//     }
//
//     #[test]
//     fn test_shapes_slice() {
//         let shapes = [
//             [
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec(),
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec()
//             ].to_vec(),
//             [
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec()
//             ].to_vec()
//         ];
//
//         let count = shapes.as_slice().iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 12);
//     }
//
//     #[test]
//     fn test_shapes_vec() {
//         let shapes = [
//             [
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec(),
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec()
//             ].to_vec(),
//             [
//                 [
//                     [0.0, 0.0],
//                     [0.0, 1.0],
//                     [1.0, 1.0],
//                     [1.0, 0.0]
//                 ].to_vec()
//             ].to_vec()
//         ];
//
//         let count = shapes.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 12);
//     }
//
//     #[derive(Copy, Clone)]
//     struct CustomPoint {
//         x: f64,
//         y: f64,
//     }
//
//     impl FloatPointCompatible<f64> for CustomPoint {
//         fn from_xy(x: f64, y: f64) -> Self {
//             Self { x, y }
//         }
//
//         fn x(&self) -> f64 {
//             self.x
//         }
//
//         fn y(&self) -> f64 {
//             self.y
//         }
//     }
//
//     #[test]
//     fn test_contour_custom() {
//         let contour = [
//             CustomPoint { x: 0.0, y: 0.0 },
//             CustomPoint { x: 0.0, y: 1.0 },
//             CustomPoint { x: 1.0, y: 1.0 },
//             CustomPoint { x: 1.0, y: 0.0 },
//         ].as_slice();
//
//         let count = contour.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 4);
//     }
//
//     #[test]
//     fn test_contours_custom() {
//         let contours = [
//             vec![
//                 CustomPoint { x: 0.0, y: 0.0 },
//                 CustomPoint { x: 0.0, y: 1.0 },
//                 CustomPoint { x: 1.0, y: 1.0 },
//                 CustomPoint { x: 1.0, y: 0.0 },
//             ],
//             vec![
//                 CustomPoint { x: 0.0, y: 0.0 },
//                 CustomPoint { x: 0.0, y: 1.0 },
//                 CustomPoint { x: 1.0, y: 1.0 },
//                 CustomPoint { x: 1.0, y: 0.0 },
//             ],
//         ];
//
//         let count = contours.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 8);
//     }
//
//     #[test]
//     fn test_shapes_custom() {
//         let shapes = [
//             vec![
//                 vec![
//                     CustomPoint { x: 0.0, y: 0.0 },
//                     CustomPoint { x: 0.0, y: 1.0 },
//                     CustomPoint { x: 1.0, y: 1.0 },
//                     CustomPoint { x: 1.0, y: 0.0 },
//                 ]
//             ],
//             vec![
//                 vec![
//                     CustomPoint { x: 0.0, y: 0.0 },
//                     CustomPoint { x: 0.0, y: 1.0 },
//                     CustomPoint { x: 1.0, y: 1.0 },
//                     CustomPoint { x: 1.0, y: 0.0 },
//                 ],
//                 vec![
//                     CustomPoint { x: 0.0, y: 0.0 },
//                     CustomPoint { x: 0.0, y: 1.0 },
//                     CustomPoint { x: 1.0, y: 1.0 },
//                     CustomPoint { x: 1.0, y: 0.0 },
//                 ],
//             ]
//         ];
//
//         let count = shapes.iter_contours().fold(0, |s, c| s + c.len());
//
//         assert_eq!(count, 12);
//     }
// }