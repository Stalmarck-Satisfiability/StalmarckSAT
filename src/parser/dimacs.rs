use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::core::formula::Formula;
use crate::Result;

/// Parser for DIMACS CNF format
#[derive(Debug, Default)]
pub struct Parser {
    error_message: String,
    has_error_flag: bool,
}

impl Parser {
    /// Create a new parser instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a DIMACS CNF file
    pub fn parse_dimacs<P: AsRef<Path>>(&mut self, _path: P) -> Result<Formula> {
        // Reset error state
        self.error_message.clear();
        self.has_error_flag = false;

        // Open the file
        let file = match File::open(_path) {
            Ok(file) => file,
            Err(e) => {
                self.error_message = format!("Failed to open file: {}", e);
                self.has_error_flag = true;
                return Err(crate::Error::Io(e));
            }
        };

        // Read the file
        let reader = BufReader::new(file);
        let mut formula = Formula::new();

        // Process each line
        for (line_num, line_result) in reader.lines().enumerate() {
            // Read the line
            let line = match line_result {
                Ok(line) => line,
                Err(e) => {
                    self.error_message = format!("Error reading line {}: {}", line_num + 1, e);
                    self.has_error_flag = true;
                    return Err(crate::Error::Io(e));
                }
            };

            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('c') {
                continue;
            }

            // Parse the problem line
            if line.starts_with('p') {
                let tokens: Vec<&str> = line.split_whitespace().collect();
                if tokens.len() >= 4 && tokens[1] == "cnf" {
                    if let Ok(num_vars) = tokens[2].parse::<usize>() {
                        formula.set_num_variables(num_vars);

                        // Also parse the number of clauses and reserve space
                        if let Ok(num_clauses) = tokens[3].parse::<usize>() {
                            formula.reserve_clauses(num_clauses);
                        } else {
                            self.error_message =
                                format!("Invalid number of clauses in line {}", line_num + 1);
                            self.has_error_flag = true;
                            return Err(crate::Error::Parse(self.error_message.clone()));
                        }
                    } else {
                        self.error_message =
                            format!("Invalid number of variables in line {}", line_num + 1);
                        self.has_error_flag = true;
                        return Err(crate::Error::Parse(self.error_message.clone()));
                    }
                }
                continue;
            }

            // Add clause to formula
            let mut clause = Vec::new();

            for token in line.split_whitespace() {
                let literal = match token.parse::<i32>() {
                    Ok(literal) => literal,
                    Err(e) => {
                        self.error_message =
                            format!("Invalid literal in line {}: {}", line_num + 1, e);
                        self.has_error_flag = true;
                        return Err(crate::Error::Parse(self.error_message.clone()));
                    }
                };

                // Stop parsing if we encounter 0
                if literal == 0 {
                    break;
                }

                // Push literal to clause
                clause.push(literal);
            }

            // Push clause to formula
            if !clause.is_empty() {
                formula.add_clause(clause);
            }
        }

        // Just return the empty formula for now
        Ok(formula)
    }

    /// Check if the parser encountered an error
    pub fn has_error(&self) -> bool {
        self.has_error_flag
    }

    /// Get the error message if there was an error
    pub fn get_error(&self) -> &str {
        &self.error_message
    }
}
