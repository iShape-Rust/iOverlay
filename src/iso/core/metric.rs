use i_shape::int::shape::IntContour;

pub(super) struct Metric {
    pub(super) min: i32,
    pub(super) max: i32,
    pub(super) vr_count: usize,
    pub(super) hz_count: usize,
    pub(super) dg_pos_count: usize,
    pub(super) dg_neg_count: usize,
}

impl Metric {

    pub(super) fn new() -> Self {
        Self {
            min: i32::MAX,
            max: i32::MIN,
            vr_count: 0,
            hz_count: 0,
            dg_pos_count: 0,
            dg_neg_count: 0,
        }
    }

    pub(super) fn add(&mut self, contours: &[IntContour]) {
        for contour in contours {
            if contour.len() < 3 { continue; }

            let mut p0 = contour.last().unwrap().clone();

            for &p1 in contour {
                self.min = self.min.min(p1.x);
                self.max = self.max.max(p1.x);

                if p0 == p1 {
                    p0 = p1;
                    continue;
                }
                if p0.x == p1.x {
                    self.vr_count += 1
                } else if p0.y == p1.y {
                    self.hz_count += 1
                } else {
                    let dp = p1 - p0;
                    let is_diagonal_pos = dp.x > 0 && dp.y > 0 || dp.x < 0 && dp.y < 0;

                    if is_diagonal_pos {
                        self.dg_pos_count += 1
                    } else {
                        self.dg_neg_count += 1
                    }
                }
                p0 = p1;
            }
        }
    }
}