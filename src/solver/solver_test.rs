use crate::core::formula::Formula;
use crate::core::formula::TripletVar;
use crate::solver::solver::Solver;

#[cfg(test)]
mod solver_test {
    use super::*;

    #[test]
    fn test_solver_initialization() {
        // Test that a new solver starts in a clean state
        let solver = Solver::new();

        // Verify initial state
        assert!(
            !solver.has_contradiction(),
            "New solver should not have contradictions"
        );
        assert!(
            !solver.has_complete_assignment(),
            "New solver should not have complete assignment"
        );
    }

    #[test]
    fn test_solver_reset() {
        // Test that reset properly clears solver state
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
    fn test_simple_rule_1() {
        // Test Rule 1: (0, y, z) => y=1, z=0
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

        // Apply simple rules to test the propagation
        solver.apply_simple_rules();

        // Verify Rule 1: when first element is false, y=1 and z=0
        assert_eq!(
            solver.assignments.get(&var_y_id),
            Some(&true),
            "Variable y (id {}) should be assigned true",
            var_y_id
        );

        assert_eq!(
            solver.assignments.get(&var_z_id),
            Some(&false),
            "Variable z (id {}) should be assigned false",
            var_z_id
        );

        assert!(
            !solver.has_contradiction(),
            "No contradiction should be found for this rule application"
        );
    }

    #[test]
    fn test_simple_rule_2() {
        // Test Rule 2: (x, y, 1) => x=1
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_y_id = 2;

        // Setup: triplet with third element as true constant
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_y_id),
            TripletVar::Const(true),
        ));

        // Apply rules and verify x is assigned true
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
    fn test_simple_rule_3() {
        // Test Rule 3: (x, 0, z) => x=1
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_z_id = 2;

        // Setup: triplet with second element as false constant
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Const(false),
            TripletVar::Var(var_z_id),
        ));

        // Apply rules and verify x is assigned true
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
    fn test_simple_rule_4a() {
        // Test Rule 4: (x, 1, z) => x=z, case where z is known
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_z_id = 2;

        // Setup: triplet with second element as true, pre-assign z=false
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Const(true),
            TripletVar::Var(var_z_id),
        ));
        solver.assignments.insert(var_z_id, false);

        // Apply rules and verify x equals z
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
    fn test_simple_rule_4b() {
        // Test Rule 4: (x, 1, z) => x=z, case where x is known
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_z_id = 2;

        // Setup: triplet with second element as true, pre-assign x=true
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Const(true),
            TripletVar::Var(var_z_id),
        ));
        solver.assignments.insert(var_x_id, true);

        // Apply rules and verify z equals x
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
    fn test_simple_rule_5a() {
        // Test Rule 5: (x, y, 0) => x=!y, case where y is known
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_y_id = 2;

        // Setup: triplet with third element as false, pre-assign y=true
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_y_id),
            TripletVar::Const(false),
        ));
        solver.assignments.insert(var_y_id, true);

        // Apply rules and verify x equals !y
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
    fn test_simple_rule_5b() {
        // Test Rule 5: (x, y, 0) => x=!y, case where x is known
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_y_id = 2;

        // Setup: triplet with third element as false, pre-assign x=false
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_y_id),
            TripletVar::Const(false),
        ));
        solver.assignments.insert(var_x_id, false);

        // Apply rules and verify y equals !x
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
    fn test_simple_rule_6() {
        // Test Rule 6: (x, x, z) => x=1, z=1
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_z_id = 2;

        // Setup: triplet where first two elements are the same variable
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_z_id),
        ));

        // Apply rules and verify both x and z are true
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
    fn test_simple_rule_7() {
        // Test Rule 7: (x, y, y) => x=1
        let mut solver = Solver::new();
        let var_x_id = 1;
        let var_y_id = 2;

        // Setup: triplet where last two elements are the same variable
        solver.current_triplets.push((
            TripletVar::Var(var_x_id),
            TripletVar::Var(var_y_id),
            TripletVar::Var(var_y_id),
        ));

        // Apply rules and verify x is true
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
    fn test_branch_and_solve() {
        // Test branching behavior when both branches lead to contradiction
        let mut solver = Solver::new();
        let mut formula = Formula::new();
        let v_id = 1;

        // Set up the formula with one variable
        formula.set_num_variables(1);
        solver.set_current_num_variables(1);

        // Add contradictory triplets that will force both branches to fail
        // First triplet: if v_id=true, then false=true (contradiction)
        solver.current_triplets.push((
            TripletVar::Var(v_id),
            TripletVar::Const(false),
            TripletVar::Const(true),
        ));

        // Second triplet: if v_id=false, then true=false (contradiction)
        solver.current_triplets.push((
            TripletVar::Var(v_id),
            TripletVar::Const(true),
            TripletVar::Const(false),
        ));

        // Verify initial state
        assert!(
            solver.assignments.is_empty(),
            "Assignments should be empty initially"
        );
        assert!(
            !solver.has_contradiction(),
            "Solver should not have a contradiction before branching"
        );
        assert!(
            !solver.has_complete_assignment(),
            "Solver should not have a complete assignment before branching"
        );

        // Call branch_and_solve - should find contradiction in both branches
        solver.branch_and_solve(&formula);

        // Verify final state after branching
        assert!(
            solver.has_contradiction(),
            "Solver should have a contradiction after branching (both branches contradictory)"
        );
        assert!(
            !solver.has_complete_assignment(),
            "Solver should not have a complete assignment when contradiction is found"
        );
        assert!(
            solver.assignments.is_empty(),
            "Assignments should be empty after branching finds contradictions in both branches"
        );
    }

    #[test]
    fn test_simple_tautology() {
        // Test detection of unsatisfiable formula (p AND -p)
        let mut solver = Solver::new();
        let mut formula = Formula::new();

        // Formula: (p AND -p) - this is unsatisfiable
        formula.add_clause(vec![1]);
        formula.add_clause(vec![-1]);
        formula.set_num_variables(1);

        // Solve returns true if formula is unsatisfiable, meaning its negation is a tautology
        let is_negation_tautology = solver.solve(&mut formula);

        // Verify the formula is detected as unsatisfiable
        assert!(
            is_negation_tautology,
            "Negation of formula (p AND -p) should be a tautology"
        );
        assert!(
            solver.has_contradiction(),
            "Solver should have found a contradiction for (p AND -p)"
        );
        assert!(
            !solver.has_complete_assignment(),
            "Solver should not have a complete assignment if a contradiction is found"
        );
    }

    #[test]
    fn test_simple_not_tautology() {
        // Test detection of satisfiable formula (p OR -p)
        let mut solver = Solver::new();
        let mut formula = Formula::new();

        // Formula: (p OR -p) - this is a tautology
        formula.add_clause(vec![1, -1]);
        formula.set_num_variables(1);

        // Solve returns false if formula is satisfiable, meaning its negation is not a tautology
        let is_negation_tautology = solver.solve(&mut formula);

        // Verify the formula is detected as satisfiable
        assert!(
            !is_negation_tautology,
            "Negation of formula (p OR -p) should not be a tautology"
        );
        assert!(
            !solver.has_contradiction(),
            "Solver should not have found a contradiction for (p OR -p)"
        );
        assert!(
            solver.has_complete_assignment(),
            "Solver should have found a complete assignment for (p OR -p)"
        );
    }

    #[test]
    fn test_pq_nq_tautology() {
        // Formula: (p) AND (-p OR q) AND (-q) - Unsatisfiable
        let mut solver = Solver::new();
        let mut formula = Formula::new();

        formula.add_clause(vec![1]);
        formula.add_clause(vec![-1, 2]);
        formula.add_clause(vec![-2]);
        formula.set_num_variables(2);

        let is_negation_tautology = solver.solve(&mut formula);

        assert!(
            is_negation_tautology,
            "Negation of (p) AND (-p OR q) AND (-q) should be a tautology"
        );
        assert!(
            solver.has_contradiction(),
            "Solver should find a contradiction for (p) AND (-p OR q) AND (-q)"
        );
        assert!(
            !solver.has_complete_assignment(),
            "Solver should not have a complete assignment if a contradiction is found"
        );
    }

    #[test]
    fn test_forcing_a_and_not_a_tautology() {
        // Formula: (a OR b) AND (a OR -b) AND (-a OR c) AND (-a OR -c) - Unsatisfiable
        let mut solver = Solver::new();
        let mut formula = Formula::new();

        formula.add_clause(vec![1, 2]);
        formula.add_clause(vec![1, -2]);
        formula.add_clause(vec![-1, 3]);
        formula.add_clause(vec![-1, -3]);
        formula.set_num_variables(3);

        let is_negation_tautology = solver.solve(&mut formula);

        assert!(
            is_negation_tautology,
            "Negation of (a OR b) AND (a OR -b) AND (-a OR c) AND (-a OR -c) should be a tautology"
        );
        assert!(
            solver.has_contradiction(),
            "Solver should find a contradiction for this formula"
        );
        assert!(
            !solver.has_complete_assignment(),
            "Solver should not have a complete assignment if a contradiction is found"
        );
    }

    #[test]
    fn test_implies_y_and_not_y_tautology() {
        // Renamed from test_unsatisfiable_implies_y_and_not_y
        // Formula: (x OR y) AND (-x OR y) AND (x OR -y) AND (-x OR -y) - Unsatisfiable
        // Its negation IS a tautology.
        let mut solver = Solver::new();
        let mut formula = Formula::new();

        formula.add_clause(vec![1, 2]);
        formula.add_clause(vec![-1, 2]);
        formula.add_clause(vec![1, -2]);
        formula.add_clause(vec![-1, -2]);
        formula.set_num_variables(2);

        let is_negation_tautology = solver.solve(&mut formula); // Renamed variable

        assert!(
            is_negation_tautology, // Check if negation is a tautology
            "Negation of (x OR y) AND (-x OR y) AND (x OR -y) AND (-x OR -y) should be a tautology"
        );
        assert!(
            solver.has_contradiction(),
            "Solver should find a contradiction for this formula"
        );
        assert!(
            !solver.has_complete_assignment(),
            "Solver should not have a complete assignment if a contradiction is found"
        );
    }
}
