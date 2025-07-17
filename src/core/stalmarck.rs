use crate::core::formula::Formula;
use crate::parser::dimacs::Parser;
use crate::solver::solver::{Dilemma, SimpleRuleStrategy, Solver};
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

        // Solve the formula
        self.solve(&mut formula)
    }

    /// Solve from a formula
    pub fn solve(&mut self, formula: &mut Formula) -> Result<bool> {
        // Set verbosity in the solver
        self.solver.set_verbosity(self.verbosity);

        // Set timeout in the solver
        self.solver.set_timeout(self.timeout);

        // Use the solver to determine if -F is a tautology
        let is_negated_tautology = self.solver.solve(&mut formula.clone());

        // Check if a timeout occurred
        if self.solver.timeout_occurred() {
            println!("UNKNOWN (Timeout)");
            // Exit or handle the timeout case appropriately
            // For now, we'll exit the process
            std::process::exit(1);
        }

        // Store the result
        self.is_tautology_result = is_negated_tautology;

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
        self.solver.set_timeout(seconds);
    }

    /// Set the verbosity level
    pub fn set_verbosity(&mut self, level: i32) {
        self.verbosity = level;
    }

    /// Set the dilemma strategy
    pub fn set_dilemma_strategy(&mut self, strategy: Dilemma) {
        self.solver.set_dilemma_strategy(strategy);
    }

    /// Set the simple rule strategy
    pub fn set_simple_rule_strategy(&mut self, strategy: SimpleRuleStrategy) {
        self.solver.set_simple_rule_strategy(strategy);
    }
}
