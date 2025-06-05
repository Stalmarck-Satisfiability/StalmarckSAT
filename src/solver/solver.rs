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

/// Helper struct to save/restore solver state
#[derive(Debug, Clone)]
struct SolverState {
    assignments: HashMap<i32, bool>,
    has_contradiction_flag: bool,
    has_complete_assignment_flag: bool,
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

    /// Solving loop
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
                    return false;
                }
                return self.handle_trivial_formula(formula);
            }
        } else {
            return self.handle_trivial_formula(formula);
        }

        self.current_num_variables = formula.num_variables();

        // Start the recursive search
        let result = self.solve_recursive(formula, 0);

        result
    }

    /// Recursive solver
    fn solve_recursive(&mut self, formula: &Formula, depth: usize) -> bool {
        // Apply simple rules (0-saturation)
        self.apply_simple_rules();

        if self.has_contradiction_flag {
            return true;
        }

        // Check if all original variables are assigned
        if self.check_all_original_variables_assigned(formula) {
            return false;
        }

        // Find a variable to branch on
        if let Some(v_id) = self.find_unassigned_variable() {
            // Save current state
            let saved_state = self.save_state();

            // Branch 1: v_id = true
            self.assign_value(&TripletVar::Var(v_id), true);
            let unsat_on_true = if self.has_contradiction_flag {
                true
            } else {
                self.solve_recursive(formula, depth + 1)
            };

            // Restore state for second branch
            self.restore_state(&saved_state);

            // Branch 2: v_id = false
            self.assign_value(&TripletVar::Var(v_id), false);
            let unsat_on_false = if self.has_contradiction_flag {
                true
            } else {
                self.solve_recursive(formula, depth + 1)
            };

            // Restore original state
            self.restore_state(&saved_state);

            // Apply dilemma rule
            if unsat_on_true && unsat_on_false {
                self.has_contradiction_flag = true;
                return true;
            } else if unsat_on_true {
                self.assign_value(&TripletVar::Var(v_id), false);
                return self.solve_recursive(formula, depth + 1);
            } else if unsat_on_false {
                self.assign_value(&TripletVar::Var(v_id), true);
                return self.solve_recursive(formula, depth + 1);
            } else {
                // Commit to satisfying model (pick first branch)
                self.assign_value(&TripletVar::Var(v_id), true);
                // Re-run rules for bridge variable consistency
                self.apply_simple_rules();
                self.has_complete_assignment_flag =
                    self.check_all_original_variables_assigned(formula);
                return false; // Return SAT
            }
        } else {
            return false;
        }
    }

    /// Find an unassigned variable to branch on
    fn find_unassigned_variable(&self) -> Option<i32> {
        // Strategy 1: Prioritize original variables first
        for i in 1..=self.current_num_variables {
            let var_id = i as i32;
            if !self.assignments.contains_key(&var_id) {
                return Some(var_id);
            }
        }

        // Strategy 2: Check bridge variables in triplets
        let mut vars_in_triplets = std::collections::HashSet::new();
        for (ta, tb, tc) in &self.current_triplets {
            for tv_ref in [ta, tb, tc] {
                if let TripletVar::Var(id) = tv_ref {
                    vars_in_triplets.insert(*id);
                }
            }
        }

        for var_id in vars_in_triplets {
            if !self.assignments.contains_key(&var_id) {
                return Some(var_id);
            }
        }

        None
    }

    /// Save the current solver state
    fn save_state(&self) -> SolverState {
        SolverState {
            assignments: self.assignments.clone(),
            has_contradiction_flag: self.has_contradiction_flag,
            has_complete_assignment_flag: self.has_complete_assignment_flag,
        }
    }

    /// Restore a previously saved solver state
    fn restore_state(&mut self, state: &SolverState) {
        self.assignments = state.assignments.clone();
        self.has_contradiction_flag = state.has_contradiction_flag;
        self.has_complete_assignment_flag = state.has_complete_assignment_flag;
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
        // Special case: formula with no original variables but has bridge variables (e.g., tautologies)
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
                // Also check for negative literal representation
                if !self.assignments.contains_key(&(-(i as i32))) {
                    return false;
                }
            }
        }

        // Original variables are assigned if we reach here
        if self.current_num_variables > 0 {
            return true;
        }
        false
    }

    /// Apply simple rules to the formula
    pub fn apply_simple_rules(&mut self) {
        loop {
            let mut made_change_in_pass = false;

            let triplets_to_process = self.current_triplets.clone();

            for (_trip_a, _trip_b, _trip_c) in triplets_to_process.iter() {
                let _initial_assignments_snapshot = self.assignments.clone();

                // Rule 1: (0, y, z) => y=1, z=0
                if let Some(false) = self.get_triplet_var_value(_trip_a) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(_trip_b, true) {
                        made_change_in_pass = true;
                    }
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(_trip_c, false) {
                        made_change_in_pass = true;
                    }
                }
                // Rule 2: (x, y, 1) => x=1
                else if let Some(true) = self.get_triplet_var_value(_trip_c) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(_trip_a, true) {
                        made_change_in_pass = true;
                    }
                }
                // Rule 3: (x, 0, z) => x=1
                else if let Some(false) = self.get_triplet_var_value(_trip_b) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(_trip_a, true) {
                        made_change_in_pass = true;
                    }
                }
                // Rule 4: (x, 1, z) => x=z
                else if let Some(true) = self.get_triplet_var_value(_trip_b) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if let Some(val_c) = self.get_triplet_var_value(_trip_c) {
                        if self.assign_value(_trip_a, val_c) {
                            made_change_in_pass = true;
                        }
                    } else if let Some(val_a) = self.get_triplet_var_value(_trip_a) {
                        if self.assign_value(_trip_c, val_a) {
                            made_change_in_pass = true;
                        }
                    }
                }
                // Rule 5: (x, y, 0) => x=-y
                else if let Some(false) = self.get_triplet_var_value(_trip_c) {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if let Some(val_b) = self.get_triplet_var_value(_trip_b) {
                        if self.assign_value(_trip_a, !val_b) {
                            made_change_in_pass = true;
                        }
                    } else if let Some(val_a) = self.get_triplet_var_value(_trip_a) {
                        if self.assign_value(_trip_b, !val_a) {
                            made_change_in_pass = true;
                        }
                    }
                }
                // Rule 6: (x, x, z) => x=1, z=1
                else if _trip_a == _trip_b {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(_trip_a, true) {
                        made_change_in_pass = true;
                    }
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(_trip_c, true) {
                        made_change_in_pass = true;
                    }
                }
                // Rule 7: (x, y, y) => x=1
                else if _trip_b == _trip_c {
                    if self.has_contradiction_flag {
                        break;
                    }
                    if self.assign_value(_trip_a, true) {
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
            // Check for contradiction if already assigned
            TripletVar::Var(id) => {
                if let Some(&current_value) = self.assignments.get(id) {
                    // Set contradiction flag if values differ
                    if current_value != value {
                        self.has_contradiction_flag = true;
                        return false;
                    }
                    return false;
                } else {
                    // Insert new value if not assigned
                    self.assignments.insert(*id, value);
                    return true;
                }
            }
            // Check for contradiction with constant
            TripletVar::Const(const_val) => {
                if *const_val != value {
                    // Set contradiction flag if constant differs
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
    /// Branch on a variable with the dilemma rule - implements Stålmarck's dilemma rule by trying both truth values
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
            // Neither branch contradicts - take the intersection of assignments
            let mut intersection_assignments = HashMap::new();
            for (var_id_s1, val_s1) in assignments_after_true.iter() {
                if let Some(val_s2) = assignments_after_false.get(var_id_s1) {
                    if val_s1 == val_s2 {
                        intersection_assignments.insert(*var_id_s1, *val_s1);
                    }
                }
            }
            self.assignments = intersection_assignments;
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
