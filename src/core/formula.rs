// Formula implementation for Stalmarck's algorithm

/// Represents a formula in propositional logic
#[derive(Debug, Clone, Default)]
pub struct Formula {
    clauses: Vec<Vec<i32>>,

    /// Formula in implication form
    implication_form: Option<ImplicationFormula>, 

    /// Triplet Representation of formula
    triplets: Vec<(i32, i32, i32)>,
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
            self.implication_form = Some(ImplicationFormula::Not(Box::new(ImplicationFormula::Const(true))));
            return;
        }

        let clause_exprs: Vec<ImplicationFormula> = self.clauses.iter()
            .map(|clause| self.clause_to_implication(clause))
            .collect();

        // Combine clauses using the rule: A AND B = NOT(A implies NOT B)
        // Start with the first clause
        let mut result = clause_exprs[0].clone();

        // Process remaining clauses left to right
        for clause_expr in clause_exprs.iter().skip(1) {
            // Apply the transformation: result AND clause_expr = NOT(result implies NOT clause_expr)
            result = ImplicationFormula::Not(
                Box::new(ImplicationFormula::Implies(
                    Box::new(result),
                    Box::new(ImplicationFormula::Not(
                        Box::new(clause_expr.clone())
                    ))
                ))
            );
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
        // reconstruct formula based on its previous content without borrow checker issues
        let original_node = std::mem::replace(formula, ImplicationFormula::Const(true));

        match original_node {
            // Transform Not(A)
            ImplicationFormula::Not(mut sub_expr_box) => {
                // Recursively call remove_nots on the inner expression A
                self.remove_nots(&mut *sub_expr_box);
                
                // Replace the original formula with Implies(A, Const(false))
                *formula = ImplicationFormula::Implies(
                    sub_expr_box, // This is Box<A'> where A' is A with its own Not's removed
                    Box::new(ImplicationFormula::Const(false)),
                );
            }
            ImplicationFormula::Implies(mut left_box, mut right_box) => {
                // Recursively call remove_nots on both children
                self.remove_nots(&mut *left_box);
                self.remove_nots(&mut *right_box);

                // Reconstruct Implies with its children
                *formula = ImplicationFormula::Implies(left_box, right_box);
            }
            ImplicationFormula::Var(variable) => {
                // Base case, a variable is left as is
                *formula = ImplicationFormula::Var(variable);
            }
            ImplicationFormula::Const(constant) => {
                // Base case, a constant is left as is
                *formula = ImplicationFormula::Const(constant);
            }
        }
    }

    /// Get the stored implication form
    pub fn get_implication_form(&self) -> Option<&ImplicationFormula> {
        self.implication_form.as_ref()
    }

    /// Encode to triplets
    pub fn encode_formula_to_triplets(&mut self) {
        // Actual implementation will go here in the future
    }

    /// Get the number of variables in the formula
    pub fn num_variables(&self) -> usize {
        self.num_vars
    }

    /// Get the number of clauses in the formula
    pub fn num_clauses(&self) -> usize {
        self.clauses.len()
    }

    /// Get the triplets representation
    pub fn get_triplets(&self) -> &[(i32, i32, i32)] {
        &self.triplets
    }

    /// Get the clauses
    pub fn get_clauses(&self) -> &[Vec<i32>] {
        &self.clauses
    }
}
