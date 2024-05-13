use crate::core::solver::{Solver, Strategy};
use crate::fill::segment::Segment;
use crate::line_range::LineRange;
use crate::split::scan_tree::ScanSplitTree;
use crate::split::shape_edge::ShapeEdge;
use crate::split::solver_list::SplitSolverList;
use crate::split::solver_tree::SplitSolverTree;
use crate::split::store_list::StoreList;
use crate::split::store_tree::StoreTree;

pub(crate) struct SplitSolver;

impl SplitSolver {
    pub(crate) fn split(edges: Vec<ShapeEdge>, solver: Solver, range: LineRange) -> (Vec<Segment>, bool) {
        let count = edges.len();
        match solver.strategy {
            Strategy::List => {
                let store = StoreList::new(edges, solver.chunk_start_length);
                let mut solver = SplitSolverList::new(store);
                _ = solver.split(usize::MAX);
                (solver.store.segments(), true)
            }
            Strategy::Tree => {
                if range.width() < solver.chunk_list_max_size as i64 {
                    let store = StoreList::new(edges, solver.chunk_start_length);
                    let mut solver = SplitSolverList::new(store);
                    _ = solver.split(usize::MAX);
                    (solver.store.segments(), true)
                } else {
                    let store = StoreTree::new(edges, solver.chunk_start_length);
                    let mut solver = SplitSolverTree::new(store, ScanSplitTree::new(range, count));
                    solver.split();
                    (solver.store.segments(), false)
                }
            }
            Strategy::Auto => {
                let list_store = StoreList::new(edges, solver.chunk_start_length);
                if list_store.is_tree_conversion_required(solver.chunk_list_max_size) {
                    let mut solver = SplitSolverTree::new(list_store.convert_to_tree(), ScanSplitTree::new(range, count));
                    solver.split();
                    (solver.store.segments(), false)
                } else {
                    let mut list_solver = SplitSolverList::new(list_store);
                    let finished = list_solver.split(solver.chunk_list_max_size);
                    if finished {
                        (list_solver.store.segments(), true)
                    } else {
                        let mut tree_solver = SplitSolverTree::new(
                            list_solver.store.convert_to_tree(),
                            ScanSplitTree::new(range, count << 1)
                        );
                        tree_solver.split();
                        (tree_solver.store.segments(), false)
                    }
                }
            }
        }
    }
}