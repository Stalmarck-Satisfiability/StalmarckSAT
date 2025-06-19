use crate::core::formula::TripletVar;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct VariableFrequency {
    /// Count of how many triplets each variable appears in
    frequency_map: HashMap<i32, usize>,

    /// Sorted list of variables by frequency
    sorted_variables: Vec<(i32, usize)>,
}

impl VariableFrequency {
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyze triplets and build frequency map
    pub fn analyze_triplets(&mut self, triplets: &[(TripletVar, TripletVar, TripletVar)]) {
        self.frequency_map.clear();

        for (a, b, c) in triplets {
            for triplet_var in [a, b, c] {
                if let TripletVar::Var(id) = triplet_var {
                    *self.frequency_map.entry(*id).or_insert(0) += 1;
                }
            }
        }

        // Sort variables by frequency (descending)
        self.sorted_variables = self
            .frequency_map
            .iter()
            .map(|(&var, &freq)| (var, freq))
            .collect();
        self.sorted_variables.sort_by(|a, b| b.1.cmp(&a.1));
    }

    /// Get the most frequent unassigned variable
    pub fn get_most_frequent_unassigned(
        &self,
        assignments: &HashMap<i32, bool>,
        original_vars_only: bool,
        max_original_var: i32,
    ) -> Option<i32> {
        for &(var_id, _freq) in &self.sorted_variables {
            if !assignments.contains_key(&var_id) {
                // If we want original variables only, filter by range
                if original_vars_only && var_id > max_original_var {
                    continue;
                }
                return Some(var_id);
            }
        }
        None
    }
}
