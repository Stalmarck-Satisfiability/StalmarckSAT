use crate::core::formula::Formula;
use crate::parser::dimacs::Parser;
use crate::solver::solver::Solver;
use crate::Result;

/// Main solver class for Stålmarck's method
#[derive(Debug, Default)]
pub struct StalmarckSolver {
    solver: Solver,
    parser: Parser,
    is_tautology_result: bool,
    timeout: f64,
    verbosity: i32,
}

impl StalmarckSolver {
    /// Create a new Stalmarck solver
    pub fn new() -> Self {
        Self::default()
    }

    /// Solve from a file path
    pub fn solve_from_file(&mut self, filename: &str) -> Result<bool> {
        // Parse the DIMACS file
        let mut formula = self.parser.parse_dimacs(filename)?;

        if self.verbosity > 0 {
            println!(
                "Parsed formula with {} variables and {} clauses",
                formula.num_variables(),
                formula.num_clauses()
            );
        }

        // Solve the formula
        self.solve(&mut formula)
    }

    /// Solve from a formula
    pub fn solve(&mut self, formula: &mut Formula) -> Result<bool> {
        if self.verbosity > 0 {
            println!("Starting the Stalmarck Procedure");
        }

        // Use the solver to determine if -F is a tautology
        let is_negated_tautology = self.solver.solve(&mut formula.clone());

        // Store the result
        self.is_tautology_result = is_negated_tautology;

        if self.verbosity > 0 {
            println!(
                "Negated formula is{} a tautology",
                if is_negated_tautology { "" } else { " not" }
            );
        }

        // Return true if the original formula is satisfiable
        Ok(!is_negated_tautology)
    }

    /// Check if the formula is a tautology
    pub fn is_tautology(&self) -> bool {
        self.is_tautology_result
    }

    /// Set the timeout value in seconds
    pub fn set_timeout(&mut self, seconds: f64) {
        self.timeout = seconds;
    }

    /// Set the verbosity level
    pub fn set_verbosity(&mut self, level: i32) {
        self.verbosity = level;
    }
}
