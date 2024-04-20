pub mod dimacs;
mod dpll;
pub mod sat_solver;
pub mod utilities;

pub type SATSolverCore = dpll::DPLL;
