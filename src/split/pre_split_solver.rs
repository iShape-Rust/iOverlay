use crate::core::solver::Solver;
use crate::split::pre_split_simple_solver::SimplePreSplitSolver;
use crate::split::shape_edge::ShapeEdge;

pub(crate) struct PreSplitSolver;

impl PreSplitSolver {
    pub(crate) fn split(solver: Solver, edges: &mut Vec<ShapeEdge>) -> bool {
        if edges.len() < solver.chunk_list_max_size {
            return SimplePreSplitSolver::split(solver.pre_split_max_count, edges);
        }

        true
    }
}