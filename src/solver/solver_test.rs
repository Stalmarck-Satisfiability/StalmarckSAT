use crate::core::formula::TripletVar;
use crate::solver::solver::Solver;
use crate::core::formula::Formula;

#[cfg(test)]
mod solver_test {
    use super::*;

    #[test]
    fn test_solver_initialization() {
        let solver = Solver::new();

        // Assert that the solver starts in a clean state
        assert!(
            !solver.has_contradiction(),
            "New solver should not have contradictions"
        );
        assert!(
            !solver.has_complete_assignment(),
            "New solver should not have complete assignment"
        );
        assert!(
            solver.verify_assignment(),
            "New solver should have valid empty assignment"
        );
    }

    #[test]
    fn test_solver_reset() {
        let mut solver = Solver::new();

        // Reset should work on a fresh solver without panicking
        solver.reset();

        // Verify state remains clean after reset
        assert!(
            !solver.has_contradiction(),
            "Reset solver should not have contradictions"
        );
        assert!(
            !solver.has_complete_assignment(),
            "Reset solver should not have complete assignment"
        );
    }

    #[test]
    fn test_simple_rule_1_propagation() {
        let mut solver = Solver::new();

        // Setup: triplet (Const(false), Var(1), Var(2))
        let var_y_id = 1;
        let var_z_id = 2;
        let trip_a = TripletVar::Const(false);
        let trip_b = TripletVar::Var(var_y_id);
        let trip_c = TripletVar::Var(var_z_id);

        solver
            .current_triplets
            .push((trip_a, trip_b.clone(), trip_c.clone()));

        // Call apply_simple_rules to test the propagation.
        solver.apply_simple_rules();

        // Assertions based on the current implementation: (0, y, z) => y=1, z=0
        assert_eq!(
            solver.assignments.get(&var_y_id),
            Some(&true),
            "Variable y (id {}) should be assigned true",
            var_y_id
        );

        assert_eq!(
            solver.assignments.get(&var_z_id),
            Some(&false),
            "Variable z (id {}) should be assigned true (as per current code)",
            var_z_id
        );

        // If the rule was strictly (0, y, z) / y=1, z=0
        assert!(
            !solver.has_contradiction(),
            "No contradiction should be found for this rule application"
        );
    }

    #[test]
    fn test_simple_rule_2_propagation() {
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_y_id = 2;
        // Rule 2: (x, y, 1) => x=1
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_y_id),
            TripletVar::Const(true),
        ));
        solver.apply_simple_rules();

        assert_eq!(
            solver.assignments.get(&var_x_id),
            Some(&true),
            "Rule 2: x should be true"
        );
        assert!(
            !solver.has_contradiction(),
            "Rule 2: No contradiction expected"
        );
    }

    #[test]
    fn test_simple_rule_3_propagation() {
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_z_id = 2;
        // Rule 3: (x, 0, z) => x=1
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Const(false),
            TripletVar::Var(var_z_id),
        ));
        solver.apply_simple_rules();

        assert_eq!(
            solver.assignments.get(&var_x_id),
            Some(&true),
            "Rule 3: x should be true"
        );
        assert!(
            !solver.has_contradiction(),
            "Rule 3: No contradiction expected"
        );
    }

    #[test]
    fn test_simple_rule_4_propagation_x_equals_known_z() {
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_z_id = 2;
        // Rule 4: (x, 1, z) => x=z. Case: z is known (e.g., true)
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Const(true),
            TripletVar::Var(var_z_id),
        ));
        solver.assignments.insert(var_z_id, false); // Pre-assign z to false
        solver.apply_simple_rules();

        assert_eq!(
            solver.assignments.get(&var_x_id),
            Some(&false),
            "Rule 4: x should be equal to z (false)"
        );
        assert!(
            !solver.has_contradiction(),
            "Rule 4 (x=z): No contradiction expected"
        );
    }

    #[test]
    fn test_simple_rule_4_propagation_z_equals_known_x() {
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_z_id = 2;
        // Rule 4: (x, 1, z) => x=z. Case: x is known (e.g., true)
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Const(true),
            TripletVar::Var(var_z_id),
        ));
        solver.assignments.insert(var_x_id, true); // Pre-assign x to true
        solver.apply_simple_rules();

        assert_eq!(
            solver.assignments.get(&var_z_id),
            Some(&true),
            "Rule 4: z should be equal to x (true)"
        );
        assert!(
            !solver.has_contradiction(),
            "Rule 4 (z=x): No contradiction expected"
        );
    }

    #[test]
    fn test_simple_rule_5_propagation_x_equals_not_known_y() {
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_y_id = 2;
        // Rule 5: (x, y, 0) => x=-y. Case: y is known (e.g., true)
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_y_id),
            TripletVar::Const(false),
        ));
        solver.assignments.insert(var_y_id, true); // Pre-assign y to true
        solver.apply_simple_rules();

        assert_eq!(
            solver.assignments.get(&var_x_id),
            Some(&false),
            "Rule 5: x should be !y (false)"
        );
        assert!(
            !solver.has_contradiction(),
            "Rule 5 (x=!y): No contradiction expected"
        );
    }

    #[test]
    fn test_simple_rule_5_propagation_y_equals_not_known_x() {
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_y_id = 2;
        // Rule 5: (x, y, 0) => x=-y. Case: x is known (e.g., false)
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_y_id),
            TripletVar::Const(false),
        ));
        solver.assignments.insert(var_x_id, false);
        solver.apply_simple_rules();

        assert_eq!(
            solver.assignments.get(&var_y_id),
            Some(&true),
            "Rule 5: y should be !x (true)"
        );
        assert!(
            !solver.has_contradiction(),
            "Rule 5 (y=!x): No contradiction expected"
        );
    }

    #[test]
    fn test_simple_rule_6_propagation() {
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_z_id = 2;

        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_z_id),
        ));
        solver.apply_simple_rules();

        assert_eq!(
            solver.assignments.get(&var_x_id),
            Some(&true),
            "Rule 6: x should be true"
        );
        assert_eq!(
            solver.assignments.get(&var_z_id),
            Some(&true),
            "Rule 6: z should be true"
        );
        assert!(
            !solver.has_contradiction(),
            "Rule 6: No contradiction expected"
        );
    }

    #[test]
    fn test_simple_rule_7_propagation() {
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_y_id = 2;

        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_y_id),
            TripletVar::Var(var_y_id), // trip_b and trip_c are the same variable
        ));
        solver.apply_simple_rules();

        assert_eq!(
            solver.assignments.get(&var_x_id),
            Some(&true),
            "Rule 7: x should be true"
        );
        assert!(
            !solver.has_contradiction(),
            "Rule 7: No contradiction expected"
        );
    }

    #[test]
    fn test_branch_an_solve() {
        let mut solver = Solver::new();
        let v_id = 1;

        solver.current_triplets.push((
            TripletVar::Var(v_id),
            TripletVar::Const(false),
            TripletVar::Const(true),
        ));

        solver.current_triplets.push((
            TripletVar::Var(v_id),
            TripletVar::Const(true),
            TripletVar::Const(false),
        ));

        assert!(
            solver.assignments.is_empty(),
            "Assignments should be empty initially."
        );
        assert!(
            !solver.has_contradiction(),
            "Solver should not have a contradiction before branching."
        );
        assert!(
            !solver.has_complete_assignment(),
            "Solver should not have a complete assignment before branching."
        );

        solver.branch_and_solve();

        assert!(
            solver.has_contradiction(),
            "Solver should not have a contradiction after branching."
        );
        assert!(
            !solver.has_complete_assignment(),
            "Solver should have a complete assignment after branching."
        );
        assert!(
            solver.assignments.is_empty(),
            "Assignments should be empty after branching and solving."
        );
    }

    #[test]
    fn test_tautology() {
        let mut solver = Solver::new();
        let mut formula = Formula::new();

        // Formula: (p AND -p) - this is unsatisfiable
        formula.add_clause(vec![1]);
        formula.add_clause(vec![-1]);
        formula.set_num_variables(1);

        let is_negation_tautology = solver.solve(&mut formula);

        // The negation of (p AND -p) is (p OR -p), which IS a tautology
        assert!(is_negation_tautology, "Negation of formula (p AND -p) should be a tautology.");
        assert!(solver.has_contradiction(), "Solver should have found a contradiction for (p AND -p).");
        assert!(!solver.has_complete_assignment(), "Solver should not have a complete assignment if a contradiction is found.");
    }

    #[test]
    fn test_not_tautology() {
        let mut solver = Solver::new();
        let mut formula = Formula::new();

        // Formula: (p OR -p) - this is a tautology
        formula.add_clause(vec![1, -1]);
        formula.set_num_variables(1);

        let is_negation_tautology = solver.solve(&mut formula);

        // The negation of (p OR -p) is (p AND -p), which is NOT a tautology
        assert!(!is_negation_tautology, "Negation of formula (p OR -p) should not be a tautology.");
        assert!(!solver.has_contradiction(), "Solver should not have found a contradiction for (p OR -p).");
        assert!(solver.has_complete_assignment(), "Solver should have found a complete assignment for (p OR -p).");
    }
}
