use super::runner::Engine;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::i_shape::int::path::IntPaths;
pub type Point = (i64, i64);
pub type Path = Vec<Point>;

#[derive(Clone, Debug)]
pub struct Fixture {
    pub subj: Vec<Path>,
    pub clip: Vec<Path>,
    pub rule: OverlayRule,
}

pub fn rect(x: i64, y: i64, w: i64, h: i64) -> Path {
    vec![(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
}

pub fn rotate(paths: &mut [Path]) {
    for path in paths {
        for (x, y) in path {
            (*x, *y) = (-*y, *x);
        }
    }
}

impl Fixture {
    pub fn edges(&self) -> usize {
        self.subj.iter().chain(&self.clip).map(Vec::len).sum()
    }

    pub fn bounds(&self) -> (i64, i64, i64, i64) {
        let mut b = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
        for &(x, y) in self.subj.iter().chain(&self.clip).flatten() {
            b = (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y));
        }
        b
    }

    // FNV-1a over domain-separated little-endian u64 lengths and i64 coordinates.
    // Contour order is intentionally significant for the INPUT fingerprint.
    pub fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for paths in [&self.subj, &self.clip] {
            hash_word(&mut hash, paths.len() as u64);
            for path in paths {
                hash_word(&mut hash, path.len() as u64);
                for &(x, y) in path {
                    hash_word(&mut hash, x as u64);
                    hash_word(&mut hash, y as u64);
                }
            }
        }
        hash
    }
}

fn hash_word(hash: &mut u64, word: u64) {
    for byte in word.to_le_bytes() {
        *hash = (*hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
    }
}

/// SplitMix64, including zero seed. All arithmetic explicitly wraps modulo 2^64.
pub struct Prng(pub u64);
impl Prng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

impl Fixture {
    pub fn from_int(subj: IntPaths<i64>, clip: IntPaths<i64>, rule: OverlayRule) -> Self {
        let cvt = |paths: IntPaths<i64>| {
            paths
                .into_iter()
                .map(|p| p.into_iter().map(|p| (p.x, p.y)).collect())
                .collect()
        };
        Self {
            subj: cvt(subj),
            clip: cvt(clip),
            rule,
        }
    }
    pub fn from_float<I: Engine>(
        subj: Vec<Vec<[f64; 2]>>,
        clip: Vec<Vec<[f64; 2]>>,
        rule: OverlayRule,
    ) -> Self {
        use i_overlay::i_float::adapter::FloatPointAdapter;
        let adapter = FloatPointAdapter::<[f64; 2], I>::with_iter_conservative(
            subj.iter().chain(&clip).flatten(),
        );
        let cvt = |paths: Vec<Vec<[f64; 2]>>| {
            paths
                .into_iter()
                .map(|p| {
                    p.iter()
                        .map(|p| {
                            let q = adapter.float_to_int(p);
                            (q.x.to_i64(), q.y.to_i64())
                        })
                        .collect()
                })
                .collect()
        };
        Self {
            subj: cvt(subj),
            clip: cvt(clip),
            rule,
        }
    }
}
#[cfg(test)]
mod checks {
    use super::*;
    #[test]
    fn splitmix64_reference_vector() {
        let mut r = Prng(0);
        assert_eq!(
            (0..5).map(|_| r.next()).collect::<Vec<_>>(),
            vec![
                0xe220a8397b1dcdaf,
                0x6e789e6aa1b965f4,
                0x06c45d188009454f,
                0xf88bb8a8724c81ec,
                0x1b39896a51a8749b
            ]
        );
    }
}
