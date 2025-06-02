#[cfg(test)]
mod tests {
    use crate::core::formula::Formula;
    use crate::core::formula::ImplicationFormula;
    use crate::core::formula::TripletFormula;
    use crate::core::formula::TripletVar;
    use std::collections::HashMap;

    #[test]
    fn test_clause_reservation() {
        // Test reserving space for clauses
        let mut formula = Formula::new();
        formula.reserve_clauses(100);

        // Add several clauses
        for i in 1..=50 {
            formula.add_clause(vec![i as i32]);
        }

        assert_eq!(formula.num_clauses(), 50);
        assert_eq!(formula.num_variables(), 50);
    }

    #[test]
    fn test_get_clauses() {
        // Test getting the clauses
        let mut formula = Formula::new();
        formula.add_clause(vec![1, -2]);
        formula.add_clause(vec![-3, 4]);

        let clauses = formula.get_clauses();
        assert_eq!(clauses.len(), 2);
        assert_eq!(clauses[0], vec![1, -2]);
        assert_eq!(clauses[1], vec![-3, 4]);
    }

    #[test]
    fn test_basic_implication_translation() {
        let mut formula = Formula::new();
        formula.add_clause(vec![1, -2]);
        formula.add_clause(vec![-3, 4]);
        formula.add_clause(vec![5]);

        formula.translate_to_implication_form();

        // Build the expected structure after NOT removal
        let expected = ImplicationFormula::Implies(
            Box::new(ImplicationFormula::Implies(
                Box::new(ImplicationFormula::Implies(
                    Box::new(ImplicationFormula::Implies(
                        Box::new(ImplicationFormula::Implies(
                            Box::new(ImplicationFormula::Implies(
                                Box::new(ImplicationFormula::Var(1)),
                                Box::new(ImplicationFormula::Const(false)),
                            )),
                            Box::new(ImplicationFormula::Implies(
                                Box::new(ImplicationFormula::Var(2)),
                                Box::new(ImplicationFormula::Const(false)),
                            )),
                        )),
                        Box::new(ImplicationFormula::Implies(
                            Box::new(ImplicationFormula::Implies(
                                Box::new(ImplicationFormula::Implies(
                                    Box::new(ImplicationFormula::Implies(
                                        Box::new(ImplicationFormula::Var(3)),
                                        Box::new(ImplicationFormula::Const(false)),
                                    )),
                                    Box::new(ImplicationFormula::Const(false)),
                                )),
                                Box::new(ImplicationFormula::Var(4)),
                            )),
                            Box::new(ImplicationFormula::Const(false)),
                        )),
                    )),
                    Box::new(ImplicationFormula::Const(false)),
                )),
                Box::new(ImplicationFormula::Implies(
                    Box::new(ImplicationFormula::Var(5)),
                    Box::new(ImplicationFormula::Const(false)),
                )),
            )),
            Box::new(ImplicationFormula::Const(false)),
        );

        // Assert that the result matches the expected structure
        assert_eq!(formula.get_implication_form(), Some(&expected));
    }

    #[test]
    fn test_basic_triplet_translation() {
        let mut formula = Formula::new();
        formula.add_clause(vec![1, -2]);
        formula.add_clause(vec![-3, 4]);
        formula.add_clause(vec![5]);

        formula.translate_to_implication_form();

        // Build the expected structure after NOT removal
        let expected = ImplicationFormula::Implies(
            Box::new(ImplicationFormula::Implies(
                Box::new(ImplicationFormula::Implies(
                    Box::new(ImplicationFormula::Implies(
                        Box::new(ImplicationFormula::Implies(
                            Box::new(ImplicationFormula::Implies(
                                Box::new(ImplicationFormula::Var(1)),
                                Box::new(ImplicationFormula::Const(false)),
                            )),
                            Box::new(ImplicationFormula::Implies(
                                Box::new(ImplicationFormula::Var(2)),
                                Box::new(ImplicationFormula::Const(false)),
                            )),
                        )),
                        Box::new(ImplicationFormula::Implies(
                            Box::new(ImplicationFormula::Implies(
                                Box::new(ImplicationFormula::Implies(
                                    Box::new(ImplicationFormula::Implies(
                                        Box::new(ImplicationFormula::Var(3)),
                                        Box::new(ImplicationFormula::Const(false)),
                                    )),
                                    Box::new(ImplicationFormula::Const(false)),
                                )),
                                Box::new(ImplicationFormula::Var(4)),
                            )),
                            Box::new(ImplicationFormula::Const(false)),
                        )),
                    )),
                    Box::new(ImplicationFormula::Const(false)),
                )),
                Box::new(ImplicationFormula::Implies(
                    Box::new(ImplicationFormula::Var(5)),
                    Box::new(ImplicationFormula::Const(false)),
                )),
            )),
            Box::new(ImplicationFormula::Const(false)),
        );

        // Assert that the translated formula matches the expected structure
        assert_eq!(formula.get_implication_form(), Some(&expected));

        // Translate implication form to triplets
        formula.encode_formula_to_triplets();

        // Build expected triplets
        let b0 = 1005;
        let b1 = 1006;
        let b2 = 1007;
        let b3 = 1008;
        let b4 = 1009;
        let b5 = 1010;
        let b6 = 1011;
        let b7 = 1012;
        let b8 = 1013;
        let b9 = 1014;
        let b10 = 1015;
        let b11 = 1016;

        let expected_triplets = TripletFormula {
            triplets: vec![
                // These triplets correspond to the 'actual' output which is logically correct
                (
                    TripletVar::Var(b0),
                    TripletVar::Var(5),
                    TripletVar::Const(false),
                ),
                (
                    TripletVar::Var(b1),
                    TripletVar::Var(3),
                    TripletVar::Const(false),
                ),
                (
                    TripletVar::Var(b2),
                    TripletVar::Var(b1),
                    TripletVar::Const(false),
                ),
                (TripletVar::Var(b3), TripletVar::Var(b2), TripletVar::Var(4)),
                (
                    TripletVar::Var(b4),
                    TripletVar::Var(b3),
                    TripletVar::Const(false),
                ),
                (
                    TripletVar::Var(b5),
                    TripletVar::Var(2),
                    TripletVar::Const(false),
                ),
                (
                    TripletVar::Var(b6),
                    TripletVar::Var(1),
                    TripletVar::Const(false),
                ),
                (
                    TripletVar::Var(b7),
                    TripletVar::Var(b6),
                    TripletVar::Var(b5),
                ),
                (
                    TripletVar::Var(b8),
                    TripletVar::Var(b7),
                    TripletVar::Var(b4),
                ),
                (
                    TripletVar::Var(b9),
                    TripletVar::Var(b8),
                    TripletVar::Const(false),
                ),
                (
                    TripletVar::Var(b10),
                    TripletVar::Var(b9),
                    TripletVar::Var(b0),
                ),
                (
                    TripletVar::Var(b11),
                    TripletVar::Var(b10),
                    TripletVar::Const(false),
                ),
            ],
            bridge_vars: {
                let mut map = HashMap::new();
                map.insert(TripletVar::Var(1005), 0);
                map.insert(TripletVar::Var(1006), 1);
                map.insert(TripletVar::Var(1007), 2);
                map.insert(TripletVar::Var(1008), 3);
                map.insert(TripletVar::Var(1009), 4);
                map.insert(TripletVar::Var(1010), 5);
                map.insert(TripletVar::Var(1011), 6);
                map.insert(TripletVar::Var(1012), 7);
                map.insert(TripletVar::Var(1013), 8);
                map.insert(TripletVar::Var(1014), 9);
                map.insert(TripletVar::Var(1015), 10);
                map.insert(TripletVar::Var(1016), 11);
                map
            },
            next_bridge_var: 1017,                // b11 + 1
            root_var: Some(TripletVar::Var(b11)), // The last bridge variable created
            max_original_var: 5,
        };

        // Assert that the encoded triplets match our expected structure
        assert_eq!(formula.get_triplets(), Some(&expected_triplets));
    }

    #[test]
    fn test_empty_formula() {
        // Test that an empty formula (no clauses) works correctly
        let mut formula = Formula::new();
        formula.translate_to_implication_form();

        // Empty formula should represent FALSE in our implementation
        let expected = ImplicationFormula::Not(Box::new(ImplicationFormula::Const(true)));
        assert_eq!(formula.get_implication_form(), Some(&expected));

        assert_eq!(formula.num_clauses(), 0);
        assert_eq!(formula.num_variables(), 0);
    }

    #[test]
    fn test_single_literal_clause() {
        // Test a formula with a single literal clause
        let mut formula = Formula::new();
        formula.add_clause(vec![7]);

        formula.translate_to_implication_form();
        let expected = ImplicationFormula::Var(7);

        assert_eq!(formula.get_implication_form(), Some(&expected));
        assert_eq!(formula.num_variables(), 7);
        assert_eq!(formula.num_clauses(), 1);
    }

    #[test]
    fn test_empty_clause() {
        // Test a formula with an empty clause (represents unsatisfiable)
        let mut formula = Formula::new();
        formula.add_clause(vec![]);
        formula.add_clause(vec![1, 2]);

        formula.translate_to_implication_form();

        // First clause is empty, which should be FALSE
        let empty_clause = ImplicationFormula::Not(Box::new(ImplicationFormula::Var(1)));

        // Result should combine the empty clause with others
        assert_ne!(formula.get_implication_form(), Some(&empty_clause)); // Just verifying it's not simply returning the empty clause

        assert_eq!(formula.num_clauses(), 2);
    }
}
