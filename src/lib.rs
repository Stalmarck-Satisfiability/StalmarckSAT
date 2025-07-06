// Main library file for stalmarck_sat

pub mod core;
pub mod error;
pub mod parser;
pub mod solver;

pub use crate::core::formula::Formula;
pub use crate::core::stalmarck::StalmarckSolver;
pub use crate::error::Error;
pub use crate::parser::dimacs::Parser;
pub use crate::solver::solver::Solver;

// Re-export the Result type with our own Error type
pub type Result<T> = std::result::Result<T, Error>;
