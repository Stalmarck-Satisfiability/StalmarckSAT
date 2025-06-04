use crate::core::formula::{Formula, ImplicationFormula, TripletVar};
use std::collections::HashMap;

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

    /// Set the number of current variables (for testing)
    pub fn set_current_num_variables(&mut self, num_vars: usize) {
        self.current_num_variables = num_vars;
    }

    /// Solve a formula. Returns `true` if UNSATISFIABLE, `false` if SATISFIABLE.
    pub fn solve(&mut self, formula: &mut Formula) -> bool {
        self.reset();
        formula.translate_to_implication_form();
        formula.encode_formula_to_triplets();

        if let Some(triplet_formula_container) = formula.get_triplets() {
            self.current_triplets = triplet_formula_container.triplets.clone();

            if let Some(root_var) = &triplet_formula_container.root_var {
                self.assign_value(root_var, true);

                if self.has_contradiction_flag {
                    return true;
                }
            } else {
                if !self.current_triplets.is_empty() {
                    eprintln!("Error: Triplets exist but no root variable found in TripletFormula. Cannot reliably solve.");
                    return false;
                }
                return self.handle_trivial_formula(formula);
            }
        } else {
            return self.handle_trivial_formula(formula);
        }

        self.current_num_variables = formula.num_variables();

        // Apply initial propagation rules after asserting the root variable
        self.apply_simple_rules();

        if self.has_contradiction_flag {
            return true;
        }

        // Check if initial rules were sufficient to assign all variables
        if self.check_all_original_variables_assigned(formula) {
            self.has_complete_assignment_flag = true;
            return false;
        }

        // Main solving loop with branching
        let mut iteration = 0;
        let max_iterations = formula.num_variables() * 2 + 10;

        while !self.has_complete_assignment_flag && !self.has_contradiction_flag {
            iteration += 1;
            if iteration > max_iterations {
                break;
            }

            // Save state before branching to detect if progress was made
            let assignments_before_branch = self.assignments.clone();
            let contradiction_before_branch = self.has_contradiction_flag;
            let complete_before_branch = self.has_complete_assignment_flag;

            // Attempt to make progress through branching
            self.branch_and_solve(formula);

            if self.has_contradiction_flag {
                break;
            }
            if self.has_complete_assignment_flag {
                break;
            }

            // Check if branching made progress
            if self.assignments == assignments_before_branch
                && self.has_contradiction_flag == contradiction_before_branch
                && self.has_complete_assignment_flag == complete_before_branch
            {
                // If no progress from branching, try applying rules again
                let mut made_change_by_rules = false;
                let assignments_before_rules = self.assignments.clone();

                self.apply_simple_rules();

                if self.assignments != assignments_before_rules {
                    made_change_by_rules = true;
                }

                // If no progress from rules either, check for completion
                if !made_change_by_rules && !self.has_contradiction_flag {
                    if self.check_all_original_variables_assigned(formula) {
                        self.has_complete_assignment_flag = true;
                        break;
                    }
                }
            } else {
                // Branching made progress, apply rules to propagate changes
                self.apply_simple_rules();
            }

            if self.has_contradiction_flag {
                break;
            }
            // Final check for completion after rule application
            if self.check_all_original_variables_assigned(formula) && !self.has_contradiction_flag {
                self.has_complete_assignment_flag = true;
                break;
            }
        }

        self.has_contradiction_flag
    }

    fn handle_trivial_formula(&mut self, formula: &Formula) -> bool {
        if let Some(imp_form) = formula.get_implication_form() {
            match imp_form {
                ImplicationFormula::Const(false) => {
                    self.has_contradiction_flag = true;
                }
                ImplicationFormula::Const(true) => {
                    self.has_contradiction_flag = false;
                    self.has_complete_assignment_flag = true;
                }
                _ => {}
            }
        } else {
            if formula.get_clauses().is_empty() && formula.num_variables() == 0 {
                self.has_complete_assignment_flag = true;
            }
        }
        self.has_contradiction_flag
    }

    /// Checks if all original variables (not bridge variables) have an assignment.
    /// Returns true if the formula is completely solved for all original variables.
    fn check_all_original_variables_assigned(&self, _formula: &Formula) -> bool {
        // Special case: formula with no original variables but has bridge variables
        // This can happen with formulas like `Var(1000) -> Var(1000)` that are tautologies
        if self.current_num_variables == 0
            && self.assignments.is_empty()
            && !self.current_triplets.is_empty()
        {
            let mut all_triplet_vars_assigned = true;
            let mut vars_in_triplets = std::collections::HashSet::new();

            // Collect all variable IDs used in triplets
            for (a, b, c) in &self.current_triplets {
                for tv in [a, b, c] {
                    if let TripletVar::Var(id) = tv {
                        vars_in_triplets.insert(*id);
                    }
                }
            }

            // Check if all triplet variables are assigned
            for tv_id in vars_in_triplets {
                if !self.assignments.contains_key(&tv_id) {
                    all_triplet_vars_assigned = false;
                    break;
                }
            }
            return all_triplet_vars_assigned;
        }

        // Check all original variables (numbered 1 to current_num_variables)
        for i in 1..=self.current_num_variables {
            if !self.assignments.contains_key(&(i as i32)) {
                // Also check for negative literal representation (if used)
                if !self.assignments.contains_key(&(-(i as i32))) {
                    return false;
                }
            }
        }

        // If we have original variables and reached here, all are assigned
        if self.current_num_variables > 0 {
            return true;
        }
        false
    }

    /// Apply simple rules to the formula
    pub fn apply_simple_rules(&mut self) {
        let mut rule_pass = 0;
        let max_rule_passes = self.current_triplets.len() + 5;

        loop {
            rule_pass += 1;
            if rule_pass > max_rule_passes {
                break;
            }
            let mut made_change_in_pass = false;

            let triplets_to_process = self.current_triplets.clone();

            for (_i, (trip_a, trip_b, trip_c)) in triplets_to_process.iter().enumerate() {
                let _initial_assignments_snapshot = self.assignments.clone();

                // Rule 1: (0, y, z) => y=1, z=0
                if let Some(false) = self.get_triplet_var_value(trip_a) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(trip_b, true) {
                        made_change_in_pass = true;
                    }
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(trip_c, false) {
                        made_change_in_pass = true;
                    }
                }
                // Rule 2: (x, y, 1) => x=1
                else if let Some(true) = self.get_triplet_var_value(trip_c) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(trip_a, true) {
                        made_change_in_pass = true;
                    }
                }
                // Rule 3: (x, 0, z) => x=1
                else if let Some(false) = self.get_triplet_var_value(trip_b) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(trip_a, true) {
                        made_change_in_pass = true;
                    }
                }
                // Rule 4: (x, 1, z) => x=z
                else if let Some(true) = self.get_triplet_var_value(trip_b) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if let Some(val_c) = self.get_triplet_var_value(trip_c) {
                        if self.assign_value(trip_a, val_c) {
                            made_change_in_pass = true;
                        }
                    } else if let Some(val_a) = self.get_triplet_var_value(trip_a) {
                        if self.assign_value(trip_c, val_a) {
                            made_change_in_pass = true;
                        }
                    }
                }
                // Rule 5: (x, y, 0) => x=-y
                else if let Some(false) = self.get_triplet_var_value(trip_c) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if let Some(val_b) = self.get_triplet_var_value(trip_b) {
                        if self.assign_value(trip_a, !val_b) {
                            made_change_in_pass = true;
                        }
                    } else if let Some(val_a) = self.get_triplet_var_value(trip_a) {
                        if self.assign_value(trip_b, !val_a) {
                            made_change_in_pass = true;
                        }
                    }
                }
                // Rule 6: (x, x, z) => x=1, z=1
                else if trip_a == trip_b {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(trip_a, true) {
                        made_change_in_pass = true;
                    }
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(trip_c, true) {
                        made_change_in_pass = true;
                    }
                }
                // Rule 7: (x, y, y) => x=1
                else if trip_b == trip_c {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(trip_a, true) {
                        made_change_in_pass = true;
                    }
                }

                if self.has_contradiction_flag {
                    break;
                }
            }

            if !made_change_in_pass || self.has_contradiction_flag {
                break;
            }
        }
    }

    /// Helper function to propagate an assignment
    fn assign_value(&mut self, tv: &TripletVar, value: bool) -> bool {
        match tv {
            // If the variable is already assigned, check for contradiction
            TripletVar::Var(id) => {
                if let Some(&current_value) = self.assignments.get(id) {
                    // If the current value contradicts the new value, set contradiction flag
                    if current_value != value {
                        self.has_contradiction_flag = true;
                        return false;
                    }
                    return false;
                } else {
                    // If not assigned, insert the new value
                    self.assignments.insert(*id, value);
                    return true;
                }
            }
            // If the variable is a constant, check for contradiction
            TripletVar::Const(const_val) => {
                if *const_val != value {
                    // If the constant value contradicts the new value, set contradiction flag
                    self.has_contradiction_flag = true;
                    return false;
                }
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
    /// This implements Stålmarck's dilemma rule by trying both truth values
    /// for an unassigned variable and taking the intersection of valid assignments
    pub fn branch_and_solve(&mut self, formula: &Formula) {
        let mut unassigned_var_id_opt: Option<i32> = None;

        // Strategy 1: Prioritize branching on original, unassigned variables first
        for i in 1..=self.current_num_variables {
            let var_id = i as i32;
            if !self.assignments.contains_key(&var_id) {
                unassigned_var_id_opt = Some(var_id);
                break;
            }
        }

        // Strategy 2: If all original variables are assigned, check bridge variables in triplets
        if unassigned_var_id_opt.is_none() {
            let mut vars_in_triplets = std::collections::HashSet::new();

            // Collect all variable IDs used in current triplets
            for (ta, tb, tc) in &self.current_triplets {
                for tv_ref in [ta, tb, tc] {
                    if let TripletVar::Var(id) = tv_ref {
                        vars_in_triplets.insert(*id);
                    }
                }
            }

            // Find the first unassigned bridge variable
            for var_id_in_triplet in vars_in_triplets {
                if !self.assignments.contains_key(&var_id_in_triplet) {
                    unassigned_var_id_opt = Some(var_id_in_triplet);
                    break;
                }
            }
        }

        // If no unassigned variables found, check if we have a complete assignment
        if unassigned_var_id_opt.is_none() {
            if !self.has_contradiction_flag && self.check_all_original_variables_assigned(formula) {
                self.has_complete_assignment_flag = true;
            }
            return;
        }

        let v_id = unassigned_var_id_opt.unwrap();

        // Save current solver state before branching
        let original_assignments = self.assignments.clone();
        let original_contradiction_flag = self.has_contradiction_flag;

        // Branch 1: Try assigning variable to true
        self.has_contradiction_flag = false;
        self.assign_value(&TripletVar::Var(v_id), true);
        if !self.has_contradiction_flag {
            self.apply_simple_rules();
        }
        let contradiction_on_true = self.has_contradiction_flag;
        let assignments_after_true = self.assignments.clone();

        // Restore state for second branch
        self.assignments = original_assignments.clone();
        self.has_contradiction_flag = original_contradiction_flag;

        // Branch 2: Try assigning variable to false
        self.has_contradiction_flag = false;
        self.assign_value(&TripletVar::Var(v_id), false);
        if !self.has_contradiction_flag {
            self.apply_simple_rules();
        }
        let contradiction_on_false = self.has_contradiction_flag;
        let assignments_after_false = self.assignments.clone();

        // Restore original state before making final decision
        self.assignments = original_assignments.clone();
        self.has_contradiction_flag = original_contradiction_flag;

        // Apply dilemma rule based on branch results
        if contradiction_on_true && contradiction_on_false {
            // Both branches contradict - the formula is unsatisfiable
            self.has_contradiction_flag = true;
        } else if contradiction_on_true {
            // True branch contradicts - commit to false branch
            self.assignments = assignments_after_false;
        } else if contradiction_on_false {
            // False branch contradicts - commit to true branch
            self.assignments = assignments_after_true;
        } else {
            // Neither branch contradicts - choose true branch
            self.assignments = assignments_after_true;
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
}
