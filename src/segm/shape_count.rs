use crate::core::overlay::ShapeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeCountBoolean {
    pub subj: i32,
    pub clip: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeCountString {
    pub subj: i32,
    pub clip: u8,
}

pub(crate) trait ShapeCount
where
    Self: Clone + Copy + Send,
{
    fn is_not_empty(&self) -> bool;
    fn new(subj: i32, clip: i32) -> Self;
    fn with_shape_type(shape_type: ShapeType) -> (Self, Self);
    fn add(self, count: Self) -> Self;
    fn apply(&mut self, count: Self);
    fn invert(self) -> Self;
}

impl ShapeCount for ShapeCountBoolean {
    #[inline(always)]
    fn is_not_empty(&self) -> bool { self.subj != 0 || self.clip != 0 }

    #[inline(always)]
    fn new(subj: i32, clip: i32) -> Self { Self { subj, clip } }

    #[inline(always)]
    fn with_shape_type(shape_type: ShapeType) -> (Self, Self) {
        match shape_type {
            ShapeType::Subject => (Self { subj: 1, clip: 0 }, Self { subj: -1, clip: 0 }),
            ShapeType::Clip => (Self { subj: 0, clip: 1 }, Self { subj: 0, clip: -1 })
        }
    }

    #[inline(always)]
    fn add(self, count: Self) -> Self {
        let subj = self.subj + count.subj;
        let clip = self.clip + count.clip;

        Self { subj, clip }
    }

    #[inline(always)]
    fn apply(&mut self, count: Self) {
        self.subj += count.subj;
        self.clip += count.clip;
    }

    #[inline(always)]
    fn invert(self) -> Self {
        Self { subj: -self.subj, clip: -self.clip }
    }
}

impl ShapeCount for ShapeCountString {
    #[inline(always)]
    fn is_not_empty(&self) -> bool { self.subj != 0 || self.clip != 0 }

    #[inline(always)]
    fn new(subj: i32, clip: i32) -> Self {
        let mask = if clip > 0 {
            0b10
        } else if clip < 0 {
            0b01
        } else {
            0
        };

        // 0 - bit - back
        // 1 - bit - forward
        Self { subj, clip: mask }
    }

    #[inline(always)]
    fn with_shape_type(shape_type: ShapeType) -> (Self, Self) {
        match shape_type {
            ShapeType::Subject => (Self { subj: 1, clip: 0 }, Self { subj: -1, clip: 0 }),
            ShapeType::Clip => (Self { subj: 0, clip: 0b10 }, Self { subj: 0, clip: 0b01 })
        }
    }

    #[inline(always)]
    fn add(self, count: Self) -> Self {
        let subj = self.subj + count.subj;
        let clip = self.clip | count.clip;

        Self { subj, clip }
    }

    #[inline(always)]
    fn apply(&mut self, count: Self) {
        self.subj += count.subj;
        self.clip = self.clip | count.clip;
    }

    #[inline(always)]
    fn invert(self) -> Self {
        let b0 = self.clip & 0b01;
        let b1 = self.clip & 0b10;
        let clip = (b0 << 1) | (b1 >> 1);

        Self { subj: -self.subj, clip }
    }
}