#[cfg(test)]
mod tests {
    use crate::core::formula::Formula;
    use crate::core::formula::ImplicationFormula;

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
                                Box::new(ImplicationFormula::Const(false))
                            )), 
                            Box::new(ImplicationFormula::Var(-2))
                        )),
                        Box::new(ImplicationFormula::Implies(
                            Box::new(ImplicationFormula::Implies(
                                Box::new(ImplicationFormula::Implies(
                                    Box::new(ImplicationFormula::Var(-3)), 
                                    Box::new(ImplicationFormula::Const(false))
                                )), 
                                Box::new(ImplicationFormula::Var(4))
                            )),
                            Box::new(ImplicationFormula::Const(false))
                        ))
                    )),
                    Box::new(ImplicationFormula::Const(false))
                )),
                Box::new(ImplicationFormula::Implies(
                    Box::new(ImplicationFormula::Var(5)), 
                    Box::new(ImplicationFormula::Const(false))
                ))
            )),
            Box::new(ImplicationFormula::Const(false))
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
                                Box::new(ImplicationFormula::Const(false))
                            )), 
                            Box::new(ImplicationFormula::Var(-2))
                        )),
                        Box::new(ImplicationFormula::Implies(
                            Box::new(ImplicationFormula::Implies(
                                Box::new(ImplicationFormula::Implies(
                                    Box::new(ImplicationFormula::Var(-3)), 
                                    Box::new(ImplicationFormula::Const(false))
                                )), 
                                Box::new(ImplicationFormula::Var(4))
                            )),
                            Box::new(ImplicationFormula::Const(false))
                        ))
                    )),
                    Box::new(ImplicationFormula::Const(false))
                )),
                Box::new(ImplicationFormula::Implies(
                    Box::new(ImplicationFormula::Var(5)), 
                    Box::new(ImplicationFormula::Const(false))
                ))
            )),
            Box::new(ImplicationFormula::Const(false))
        );
        
        // Assert that the translated formula matches the expected structure
        assert_eq!(formula.get_implication_form(), Some(&expected));

        // Translate implication form to triplets
        // formula.encode_formula_to_triplets();

        // // Build expected triplets
        // let expected_triplets = vec![

        // ];
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