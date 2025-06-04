// Core solver implementation
use std::collections::HashMap;

use crate::core::formula::{Formula, TripletVar};

/// Core solver for Stalmarck's method
#[derive(Debug, Default)]
pub struct Solver {
    pub(crate) assignments: HashMap<i32, bool>,
    has_contradiction_flag: bool,
    has_complete_assignment_flag: bool,
    pub(crate) current_triplets: Vec<(TripletVar, TripletVar, TripletVar)>,
    current_num_variables: usize,
}

impl Solver {
    /// Create a new solver instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Solve a formula
    pub fn solve(&mut self, formula: &mut Formula) -> bool {
        // Reset state
        self.reset();

        // Translate formula to implication form
        formula.translate_to_implication_form();

        // Encode formula to triplets
        formula.encode_formula_to_triplets();

        // Get the formula triplets
        if let Some(triplet_formula) = formula.get_triplets() {
            self.current_triplets = triplet_formula.triplets.clone();
        }

        while !self.has_complete_assignment_flag && !self.has_contradiction_flag {
            // Repeatedly apply simple rules
            self.apply_simple_rules();

            if self.has_contradiction_flag || self.has_complete_assignment_flag {
                break;
            }

            // Apply branching
            self.branch_and_solve()
        }

        !self.has_contradiction_flag
    }

    /// Apply simple rules to the formula
    pub fn apply_simple_rules(&mut self) {
        // Apply simple rules until no changes are made
        loop {
            let mut made_change_in_pass = false;

            // Iterate over current triplets
            let triplets_to_process = self.current_triplets.clone();

            for (trip_a, trip_b, trip_c) in triplets_to_process {
                // Check rule 1: (0, y, z) / y=1, z=0
                if let Some(false) = self.get_triplet_var_value(&trip_a) {
                    // Assign trip_b to true and propagate
                    if self.assign_value(&trip_b, true) {
                        made_change_in_pass = true;
                    }

                    // Assign trip_c to true and propagate
                    if self.assign_value(&trip_c, false) {
                        made_change_in_pass = true;
                    }
                }
                // Check rule 2: (x, y, 1) / x=1
                else if let Some(true) = self.get_triplet_var_value(&trip_c) {
                    // Assign trip_a to true and propagate
                    if self.assign_value(&trip_a, true) {
                        made_change_in_pass = true;
                    }
                }
                // Check rule 3: (x, 0, z) / x=1
                else if let Some(false) = self.get_triplet_var_value(&trip_b) {
                    // Assign trip_b to false and propagate
                    if self.assign_value(&trip_a, true) {
                        made_change_in_pass = true;
                    }
                }
                // Check rule 4: (x, 1, z) / x=z
                else if let Some(true) = self.get_triplet_var_value(&trip_b) {
                    if let Some(val_c) = self.get_triplet_var_value(&trip_c) {
                        if self.assign_value(&trip_a, val_c) {
                            made_change_in_pass = true;
                        }
                    } else if let Some(val_a) = self.get_triplet_var_value(&trip_a) {
                        if self.assign_value(&trip_c, val_a) {
                            made_change_in_pass = true;
                        }
                    }
                }
                // Check rule 5: (x, y, 0) / x=-y
                else if let Some(false) = self.get_triplet_var_value(&trip_c) {
                    if let Some(val_b) = self.get_triplet_var_value(&trip_b) {
                        if self.assign_value(&trip_a, !val_b) {
                            made_change_in_pass = true;
                        }
                    } else if let Some(val_a) = self.get_triplet_var_value(&trip_a) {
                        if self.assign_value(&trip_b, !val_a) {
                            made_change_in_pass = true;
                        }
                    }
                }
                // Check rule 6: (x, x, z) / x=1, z=1
                else if trip_a == trip_b {
                    if self.assign_value(&trip_a, true) {
                        made_change_in_pass = true;
                    }
                    if self.assign_value(&trip_c, true) {
                        made_change_in_pass = true;
                    }
                }
                // Check rule 7: (x, y, y) / x=1
                else if trip_b == trip_c {
                    if self.assign_value(&trip_a, true) {
                        made_change_in_pass = true;
                    }
                }

                // If a contradiction was found during any assignment, stop processing rules for this pass
                if self.has_contradiction_flag {
                    break;
                }
            }

            // If no changes were made in this pass, or a contradiction was found, exit the loop
            if !made_change_in_pass || self.has_contradiction_flag {
                break;
            }
        }
    }

    /// Helper function to propagate an assignment
    fn assign_value(&mut self, tv: &TripletVar, value: bool) -> bool {
        match tv {
            TripletVar::Var(id) => {
                if let Some(&current_value) = self.assignments.get(id) {
                    if current_value != value {
                        // Contradiction: trying to assign a different value
                        self.has_contradiction_flag = true;
                        return false;
                    }
                    // Already assigned the same value, no change
                    return false;
                } else {
                    // New assignment
                    self.assignments.insert(*id, value);

                    // A change was made, a new variable was assigned
                    return true;
                }
            }
            TripletVar::Const(const_val) => {
                // Check if the assignment to a constant is contradictory
                if *const_val != value {
                    self.has_contradiction_flag = true;
                    return false;
                }

                // Assignment is consistent with the constant
                return false;
            }
        }
    }

    // Helper function to get the evaluated value of a TripletVar
    fn get_triplet_var_value(&self, tv: &TripletVar) -> Option<bool> {
        match tv {
            TripletVar::Const(val) => Some(*val),
            TripletVar::Var(id) => self.assignments.get(id).cloned(),
        }
    }

    /// Branch on a variable with the dilemma rule and attempt to solve
    pub fn branch_and_solve(&mut self) {
        // Find unassigned variable present in current_triplets
        let mut unassigned_var_id_opt: Option<i32> = None;
        let mut vars_in_triplets = std::collections::HashSet::new();

        for (triplet_a, triplet_b, triplet_c) in &self.current_triplets {
            for tv in [triplet_a, triplet_b, triplet_c] {
                if let TripletVar::Var(id) = tv {
                    vars_in_triplets.insert(*id);
                }
            }
        }

        for var_id in vars_in_triplets {
            if !self.assignments.contains_key(&var_id) {
                unassigned_var_id_opt = Some(var_id);
                break;
            }
        }

        // If no unassigned variable relevant to triplets, current assignment is complete
        if unassigned_var_id_opt.is_none() {
            self.has_complete_assignment_flag = true;
            return;
        }

        let v_id = unassigned_var_id_opt.unwrap();

        // Store current assignment state
        let original_assignments = self.assignments.clone();

        // Branch on v_id = true
        self.assignments.insert(v_id, true); // Changed from assert to insert
        self.has_contradiction_flag = false;
        self.apply_simple_rules();
        let contradiction_ont_true = self.has_contradiction_flag;
        let assignments_after_true = self.assignments.clone();

        // Restore state
        self.assignments = original_assignments.clone();

        // Branch on v_id = false
        self.assignments.insert(v_id, false); // Changed from assert to insert
        self.has_contradiction_flag = false;
        self.apply_simple_rules();
        let contradiction_ont_false = self.has_contradiction_flag;
        let assignments_after_false = self.assignments.clone();

        // Analyze results and update solver state
        if contradiction_ont_true && contradiction_ont_false {
            // Corrected variable names
            // Both branches lead to contradictions
            self.assignments = original_assignments;
            self.has_contradiction_flag = true;
        } else if contradiction_ont_true && !contradiction_ont_false {
            // Corrected variable names
            // Commit to false branch
            self.assignments = assignments_after_false;
            self.has_contradiction_flag = false;
        } else if !contradiction_ont_true && contradiction_ont_false {
            // Corrected variable names
            // Commit to true branch
            self.assignments = assignments_after_true;
            self.has_contradiction_flag = false;
        } else {
            // Neither branch leads to a contradiction
            // Keep assignments from one of the successful branches, e.g., true branch
            self.assignments = assignments_after_true;
            self.has_complete_assignment_flag = true; // Corrected to assign to the flag field
            self.has_contradiction_flag = false;
        }
    }

    /// Check if a contradiction was found
    pub fn has_contradiction(&self) -> bool {
        self.has_contradiction_flag
    }

    /// Check if a complete assignment was found
    pub fn has_complete_assignment(&self) -> bool {
        self.has_complete_assignment_flag
    }

    /// Reset the solver state
    pub fn reset(&mut self) {
        self.assignments.clear();
        self.has_contradiction_flag = false;
        self.has_complete_assignment_flag = false;
        self.current_triplets.clear();
        self.current_num_variables = 0;
    }

    /// Verify that the current assignment satisfies the formula
    pub fn verify_assignment(&self) -> bool {
        // Placeholder implementation
        true
    }

    /// Evaluate a literal with the current assignment
    pub fn eval_literal(&self, _literal: i32) -> bool {
        // Placeholder implementation
        false
    }
}
