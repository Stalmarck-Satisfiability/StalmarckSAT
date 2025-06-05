use std::collections::HashMap;

/// Represents a formula in propositional logic
#[derive(Debug, Clone, Default)]
pub struct Formula {
    clauses: Vec<Vec<i32>>,

    /// Formula in implication form
    implication_form: Option<ImplicationFormula>,

    /// Triplet Representation of formula
    triplets: Option<TripletFormula>,
    num_vars: usize,
}

/// Represents a formula in implication form
#[derive(Debug, Clone, PartialEq)]
pub enum ImplicationFormula {
    /// A variable (positive or negative literal)
    Var(i32),

    /// Negation of an expression (NOT)
    Not(Box<ImplicationFormula>),

    /// Implication relation (p → q)
    Implies(Box<ImplicationFormula>, Box<ImplicationFormula>),

    /// Boolean constants
    Const(bool),
}

/// Represents a variable or constant in triplet form
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TripletVar {
    /// A variable (positive or negative literal)
    Var(i32),

    /// Boolean constants
    Const(bool),
}

/// Represents a formula in triplet form for Stalmarck's algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct TripletFormula {
    /// Collection of triplets (a, b, c) where each represents a logical relation
    pub triplets: Vec<(TripletVar, TripletVar, TripletVar)>,

    /// Mapping from bridge variables (b1, b2, etc.) to their corresponding triplet index
    pub bridge_vars: std::collections::HashMap<TripletVar, usize>,

    /// The next available bridge variable ID
    pub next_bridge_var: i32,

    /// The root variable representing the entire formula
    pub root_var: Option<TripletVar>,

    /// Maximum original variable ID (not including bridge variables)
    pub max_original_var: i32,
}

impl Formula {
    /// Create a new empty formula
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a clause to the formula
    pub fn add_clause(&mut self, literals: Vec<i32>) {
        // Update the number of variables based on the literals in the clause
        for &lit in &literals {
            let var = lit.abs() as usize;
            if var > self.num_vars {
                self.num_vars = var;
            }
        }
        self.clauses.push(literals);
    }

    /// Set the number of variables directly
    pub fn set_num_variables(&mut self, num_vars: usize) {
        self.num_vars = num_vars;
    }

    /// Pre-allocate space for the expected number of clauses
    pub fn reserve_clauses(&mut self, num_clauses: usize) {
        self.clauses.reserve(num_clauses);
    }

    /// Translate to implication form
    pub fn translate_to_implication_form(&mut self) {
        if self.clauses.is_empty() {
            // Empty clause is unsatisfiable (FALSE)
            self.implication_form = Some(ImplicationFormula::Not(Box::new(
                ImplicationFormula::Const(true),
            )));
            return;
        }

        let clause_exprs: Vec<ImplicationFormula> = self
            .clauses
            .iter()
            .map(|clause| self.clause_to_implication(clause))
            .collect();

        // Combine clauses using the rule: A AND B = NOT(A implies NOT B)
        // Start with the first clause
        let mut result = clause_exprs[0].clone();

        // Process remaining clauses left to right
        for clause_expr in clause_exprs.iter().skip(1) {
            // Apply the transformation: result AND clause_expr = NOT(result implies NOT clause_expr)
            result = ImplicationFormula::Not(Box::new(ImplicationFormula::Implies(
                Box::new(result),
                Box::new(ImplicationFormula::Not(Box::new(clause_expr.clone()))),
            )));
        }

        // Remove all NOTs from result
        self.remove_nots(&mut result);

        self.implication_form = Some(result);
    }

    /// Helper method to convert a single clause to implication form
    fn clause_to_implication(&self, clause: &[i32]) -> ImplicationFormula {
        if clause.is_empty() {
            // Empty clause is unsatisfiable (FALSE)
            return ImplicationFormula::Not(Box::new(ImplicationFormula::Const(true)));
        }

        if clause.len() == 1 {
            // Single literal clause
            return ImplicationFormula::Var(clause[0]);
        }

        // Convert OR clause to implication form
        // (p ∨ q ∨ r) = (¬p → (q ∨ r)) = (¬p → (¬q → r))
        // Start with the first literal
        let mut expr = ImplicationFormula::Var(clause[0]);

        // Process literals left to right
        for &lit in &clause[1..] {
            // Create implication structure
            expr = ImplicationFormula::Implies(
                Box::new(ImplicationFormula::Not(Box::new(expr))),
                Box::new(ImplicationFormula::Var(lit)),
            );
        }

        expr
    }

    /// Helper method to remove NOTs from the implication formula
    fn remove_nots(&self, formula: &mut ImplicationFormula) {
        // Perform a "take and replace" to modify the enum variant in place
        let original_node = std::mem::replace(formula, ImplicationFormula::Const(true));

        match original_node {
            ImplicationFormula::Not(mut sub_expr_box) => {
                // Recursively call remove_nots on the inner expression
                self.remove_nots(&mut *sub_expr_box);

                // Replace Not(A) with Implies(A, Const(false))
                *formula = ImplicationFormula::Implies(
                    sub_expr_box,
                    Box::new(ImplicationFormula::Const(false)),
                );
            }
            ImplicationFormula::Implies(mut left_box, mut right_box) => {
                // Recursively call remove_nots on both children
                self.remove_nots(&mut *left_box);
                self.remove_nots(&mut *right_box);

                // Reconstruct the Implies node with its children
                *formula = ImplicationFormula::Implies(left_box, right_box);
            }
            ImplicationFormula::Var(id) => {
                if id < 0 {
                    // Transform Var(-k) into Implies(Var(k), Const(false)).
                    let positive_id = id.abs();
                    *formula = ImplicationFormula::Implies(
                        Box::new(ImplicationFormula::Var(positive_id)),
                        Box::new(ImplicationFormula::Const(false)),
                    );
                    // The children of this new Implies node are Var(positive_id) and Const(false).
                    // These are already "clean" in terms of Not nodes or negative Vars,
                    // so no further recursive call on `*formula` itself is needed here.
                } else {
                    // Positive literal, restore it as is.
                    *formula = ImplicationFormula::Var(id);
                }
            }
            ImplicationFormula::Const(constant) => {
                // Constants are left as is, restore it.
                *formula = ImplicationFormula::Const(constant);
            }
        }
    }

    /// Get the stored implication form
    pub fn get_implication_form(&self) -> Option<&ImplicationFormula> {
        self.implication_form.as_ref()
    }

    /// Encode the formula into triplets
    pub fn encode_formula_to_triplets(&mut self) {
        let root_implication_node = match self.implication_form.as_ref() {
            Some(form) => form,
            None => {
                // If there's no implication form (e.g., formula was empty and translate wasn't called,
                // or explicitly set to None), then there are no triplets.
                self.triplets = None;
                return;
            }
        };

        // Create a new TripletFormula instance
        let mut triplet_formula = TripletFormula {
            triplets: Vec::new(),
            bridge_vars: HashMap::new(),
            next_bridge_var: self.num_vars as i32 + 1000,
            root_var: None,
            max_original_var: self.num_vars as i32,
        };

        // Map to store TripletVar result for each processed ImplicationFormula node
        let mut processed_results: HashMap<*const ImplicationFormula, TripletVar> = HashMap::new();

        // Stack for iterative post-order traversal
        let mut traversal_stack: Vec<&ImplicationFormula> = Vec::new();

        // Start traversal from the root of the implication form
        traversal_stack.push(root_implication_node);

        while let Some(current_node) = traversal_stack.last().cloned() {
            let current_node_ptr = current_node as *const ImplicationFormula;

            // If result for this node has already been processed, skip it
            if processed_results.contains_key(&current_node_ptr)
                && !matches!(current_node, ImplicationFormula::Implies(_, _))
            {
                traversal_stack.pop();
                continue;
            }

            match current_node {
                ImplicationFormula::Var(id) => {
                    // Leaf Node: process immediately
                    traversal_stack.pop();
                    let result = TripletVar::Var(*id);
                    processed_results.insert(current_node_ptr, result);
                }
                ImplicationFormula::Const(value) => {
                    // Leaf Node: process immediately
                    traversal_stack.pop();
                    let result = TripletVar::Const(*value);
                    processed_results.insert(current_node_ptr, result);
                }
                ImplicationFormula::Implies(left_expr, right_expr) => {
                    let left_child = &**left_expr;
                    let right_child = &**right_expr;

                    let left_child_ptr = left_child as *const ImplicationFormula;
                    let right_child_ptr = right_child as *const ImplicationFormula;

                    let left_processed = processed_results.contains_key(&left_child_ptr);
                    let right_processed = processed_results.contains_key(&right_child_ptr);

                    if left_processed && right_processed {
                        // Both children have been processed, process Implies node
                        traversal_stack.pop();

                        let var_a = processed_results
                            .get(&left_child_ptr)
                            .expect("Left child should be processed")
                            .clone();

                        let var_b = processed_results
                            .get(&right_child_ptr)
                            .expect("Right child should be processed")
                            .clone();

                        // Create a new bridge variable for this implication
                        let bridge_id = triplet_formula.next_bridge_var;
                        triplet_formula.next_bridge_var += 1;
                        let bridge_var = TripletVar::Var(bridge_id);

                        // Store the bridge variable in the map
                        triplet_formula
                            .triplets
                            .push((bridge_var.clone(), var_a, var_b));
                        triplet_formula
                            .bridge_vars
                            .insert(bridge_var.clone(), triplet_formula.triplets.len() - 1);
                        processed_results.insert(current_node_ptr, bridge_var);
                    } else {
                        // Children are not processed
                        if !left_processed {
                            // Push left child onto the stack for later processing
                            traversal_stack.push(left_child);
                        }

                        if !right_processed {
                            // Push right child onto the stack for later processing
                            traversal_stack.push(right_child);
                        }

                        // Current Implies node remains on the stack to be revisited
                    }
                }
                ImplicationFormula::Not(_) => {
                    // Should not happen in the final implication form
                    unreachable!("Not nodes should have been removed in the implication form");
                }
            }
        }

        // The result for the root of the entire implication form is its TripletVar representation
        triplet_formula.root_var = processed_results
            .get(&(root_implication_node as *const ImplicationFormula))
            .cloned();

        self.triplets = Some(triplet_formula);
    }

    /// Get the triplets representation
    pub fn get_triplets(&self) -> Option<&TripletFormula> {
        self.triplets.as_ref()
    }

    /// Get the number of variables in the formula
    pub fn num_variables(&self) -> usize {
        self.num_vars
    }

    /// Get the number of clauses in the formula
    pub fn num_clauses(&self) -> usize {
        self.clauses.len()
    }

    /// Get the clauses
    pub fn get_clauses(&self) -> &[Vec<i32>] {
        &self.clauses
    }
}
