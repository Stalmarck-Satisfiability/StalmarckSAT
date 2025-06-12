/// Statistics for tracking solver performance and progress
#[derive(Debug, Default, Clone)]
pub struct SolverStatistics {
    /// Number of times solve_recursive has been called
    pub recursive_calls: usize,
    /// Number of times simple rules have been applied
    pub simple_rule_applications: usize,
    /// Number of subproblems created in the search tree
    pub subproblems_explored: usize,
    /// Maximum depth reached during recursive solving
    pub max_depth: usize,
    /// Number of times dilemma rule was applied (branching)
    pub dilemma_rule_applications: usize,
}

impl SolverStatistics {
    /// Reset all statistics to zero
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Increment the recursive calls counter
    pub fn increment_recursive_calls(&mut self) {
        self.recursive_calls += 1;
    }

    /// Increment the simple rule applications counter
    pub fn increment_simple_rule_applications(&mut self) {
        self.simple_rule_applications += 1;
    }

    /// Increment the subproblems explored counter
    pub fn increment_subproblems_explored(&mut self) {
        self.subproblems_explored += 1;
    }

    /// Update the maximum depth if current depth is greater
    pub fn update_max_depth(&mut self, depth: usize) {
        if depth > self.max_depth {
            self.max_depth = depth;
        }
    }

    /// Increment the dilemma rule applications counter
    pub fn increment_dilemma_rule_applications(&mut self) {
        self.dilemma_rule_applications += 1;
    }

    /// Print progress information with specified verbosity level
    pub fn print_progress(&self, _depth: usize, verbosity: i32) {
        if verbosity >= 2 {
            println!("Iteration number: {}", self.recursive_calls);
            println!(
                "Simple Rule Applications: {}, Dilemma Rule Applications: {}, Subproblems Explored: {}\n",
                self.simple_rule_applications, self.dilemma_rule_applications, self.subproblems_explored
            );
            println!("=========================\n");
        }
    }

    /// Print final statistics summary
    pub fn print_summary(&self, verbosity: i32) {
        if verbosity >= 1 {
            println!("\n=== Solver Statistics ===");
            println!("Total iterations: {}", self.recursive_calls);
            println!(
                "Total simple rule applications: {}",
                self.simple_rule_applications
            );
            println!(
                "Total dilemma rule applications: {}",
                self.dilemma_rule_applications
            );
            println!("Total subproblems explored: {}", self.subproblems_explored);
            println!("Maximum depth reached: {}", self.max_depth);
            println!("========================\n");
        }
    }
}
