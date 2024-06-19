#[cfg(feature = "allow_multithreading")]
use rayon::prelude::*;
use crate::pre_split::line_mark::LineMark;
use crate::pre_split::solver::PreSplitSolver;
use crate::split::cross_solver::ScanCrossSolver;
use crate::split::shape_edge::ShapeEdge;

struct Chunk {
    marks: Vec<LineMark>,
    need_to_fix: bool,
}

impl PreSplitSolver {
    pub(super) fn multiple_split(max_repeat_count: usize, edges: &mut Vec<ShapeEdge>) -> bool {
        let n = edges.len();
        let chunks_cnt = 256;
        let chunk_size = n / chunks_cnt;
        let chunks_count = if chunk_size * chunks_cnt == n {
            chunks_cnt
        } else {
            chunks_cnt + 1
        };

        let mut chunks: Vec<Chunk> = (0..chunks_count).map(|_| Chunk { marks: vec![], need_to_fix: false }).collect();

        let mut need_to_fix = true;
        let mut split_count = 0;

        let mut total_marks = Vec::new();

        while need_to_fix && split_count < max_repeat_count {
            split_count += 1;

            chunks.par_iter_mut().enumerate().for_each(|(index, chunk)| {
                let i0 = index * chunk_size;
                let i1 = (i0 + chunk_size).min(n);

                for i in i0..i1 {
                    let ei = &edges[i].x_segment;
                    for j in i + 1..n {
                        let ej = &edges[j].x_segment;
                        if ei.b <= ej.a {
                            break;
                        }

                        if ScanCrossSolver::test_y(ei, ej) {
                            continue;
                        }

                        Self::cross(i, j, ei, ej, &mut chunk.need_to_fix, &mut chunk.marks);
                    }
                }
            });

            need_to_fix = false;
            for chunk in chunks.iter_mut() {
                if chunk.need_to_fix {
                    need_to_fix = true;
                    chunk.need_to_fix = false;
                }
                if !chunk.marks.is_empty() {
                    total_marks.append(&mut chunk.marks);
                }
            }

            if total_marks.is_empty() {
                return false;
            }

            Self::apply(need_to_fix, &mut total_marks, edges);

            total_marks.clear();
        }

        need_to_fix
    }
}