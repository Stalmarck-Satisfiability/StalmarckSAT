// Main library file for stalmarck_sat

pub mod core;
pub mod parser;
pub mod solver;
pub mod error;

pub use crate::core::formula::Formula;
pub use crate::core::stalmarck::StalmarckSolver;
pub use crate::parser::dimacs::Parser;
pub use crate::solver::solver::Solver;
pub use crate::error::Error;

// Re-export the Result type with our own Error type
pub type Result<T> = std::result::Result<T, Error>;
