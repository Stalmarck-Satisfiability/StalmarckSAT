use crate::core::formula::TripletVar;
use crate::solver::variable_frequency::VariableFrequency;
use std::collections::HashMap;

#[cfg(test)]
mod variable_frequency_test {
    use super::*;

    #[test]
    fn test_new() {
        let vf = VariableFrequency::new();
        let assignments = HashMap::new();
        assert_eq!(
            vf.get_most_frequent_unassigned(&assignments, false, 0),
            None
        );
    }

    #[test]
    fn test_analyze_triplets() {
        let mut vf = VariableFrequency::new();
        let triplets = vec![
            (TripletVar::Var(1), TripletVar::Var(2), TripletVar::Var(3)),
            (
                TripletVar::Var(1),
                TripletVar::Var(4),
                TripletVar::Const(true),
            ),
            (TripletVar::Var(2), TripletVar::Var(1), TripletVar::Var(5)),
        ];
        vf.analyze_triplets(&triplets);

        let mut assignments = HashMap::new();
        assert_eq!(
            vf.get_most_frequent_unassigned(&assignments, false, 5),
            Some(1)
        );

        assignments.insert(1, true);
        assert_eq!(
            vf.get_most_frequent_unassigned(&assignments, false, 5),
            Some(2)
        );

        assignments.insert(2, true);
        let next_var = vf
            .get_most_frequent_unassigned(&assignments, false, 5)
            .unwrap();
        assert!([3, 4, 5].contains(&next_var));
    }

    #[test]
    fn test_get_most_frequent_unassigned() {
        let mut vf = VariableFrequency::new();
        let triplets = vec![
            (TripletVar::Var(1), TripletVar::Var(10), TripletVar::Var(2)),
            (TripletVar::Var(2), TripletVar::Var(10), TripletVar::Var(3)),
            (TripletVar::Var(3), TripletVar::Var(10), TripletVar::Var(4)),
        ];
        vf.analyze_triplets(&triplets);

        let mut assignments = HashMap::new();
        // Var 10 appears most
        assert_eq!(
            vf.get_most_frequent_unassigned(&assignments, false, 4),
            Some(10)
        );
        // With original_vars_only, 10 is skipped. 2 and 3 are next most frequent.
        let next_var = vf
            .get_most_frequent_unassigned(&assignments, true, 4)
            .unwrap();
        assert!([2, 3].contains(&next_var));

        assignments.insert(10, false);
        // Now 2, 3 are candidates. Sorted list is not guaranteed for same-frequency variables.
        let next_var = vf
            .get_most_frequent_unassigned(&assignments, false, 4)
            .unwrap();
        assert!([2, 3].contains(&next_var));

        assignments.insert(1, true);
        assignments.insert(2, true);
        assignments.insert(3, true);
        assignments.insert(4, true);
        assert_eq!(
            vf.get_most_frequent_unassigned(&assignments, false, 4),
            None
        );
    }

    #[test]
    fn test_get_triplet_frequency() {
        let mut vf = VariableFrequency::new();
        let triplets = vec![
            (TripletVar::Var(1), TripletVar::Var(2), TripletVar::Var(3)),
            (
                TripletVar::Var(1),
                TripletVar::Var(4),
                TripletVar::Const(true),
            ),
        ];
        vf.analyze_triplets(&triplets);

        let triplet1 = (TripletVar::Var(1), TripletVar::Var(2), TripletVar::Var(3));
        // Frequencies: 1->2, 2->1, 3->1. Total = 4
        assert_eq!(vf.get_triplet_frequency(&triplet1), 4);

        let triplet2 = (
            TripletVar::Var(1),
            TripletVar::Var(4),
            TripletVar::Const(true),
        );
        // Frequencies: 1->2, 4->1. Total = 3
        assert_eq!(vf.get_triplet_frequency(&triplet2), 3);

        let triplet3 = (
            TripletVar::Var(5),
            TripletVar::Var(6),
            TripletVar::Const(false),
        );
        // Frequencies: 5->0, 6->0. Total = 0
        assert_eq!(vf.get_triplet_frequency(&triplet3), 0);
    }

    #[test]
    fn test_get_potential_deduction_score() {
        let mut vf = VariableFrequency::new();
        let triplets = vec![
            (TripletVar::Var(1), TripletVar::Var(2), TripletVar::Var(3)),
            (TripletVar::Var(4), TripletVar::Var(4), TripletVar::Var(5)),
            (TripletVar::Var(6), TripletVar::Var(7), TripletVar::Var(7)),
        ];
        vf.analyze_triplets(&triplets);

        let triplet1 = (TripletVar::Var(1), TripletVar::Var(2), TripletVar::Var(3));
        // No deduction, score = 1 * base_freq = 3
        assert_eq!(vf.get_potential_deduction_score(&triplet1), 3);

        let triplet2 = (TripletVar::Var(4), TripletVar::Var(4), TripletVar::Var(5));
        // Rule (x,x,z), potential_deductions = 2. Score = (1+2) * 5 = 15
        assert_eq!(vf.get_potential_deduction_score(&triplet2), 15);

        let triplet3 = (TripletVar::Var(6), TripletVar::Var(7), TripletVar::Var(7));
        // Rule (x,y,y), potential_deductions = 1. Score = (1+1) * 5 = 10
        assert_eq!(vf.get_potential_deduction_score(&triplet3), 10);
    }
}
