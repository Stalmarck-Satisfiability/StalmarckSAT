#[cfg(test)]
mod tests {
    use crate::core::formula::Formula;
    use crate::parser::dimacs::Parser;
    use std::fs::File;
    use std::io::{Read, Write};
    use tempfile::NamedTempFile;

    // Helper function to create a temporary CNF file
    fn create_temp_cnf_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    // Helper function to print file content and formula for debugging
    fn print_debug_info(file_path: &std::path::Path, formula: &Formula) {
        println!("==== FILE CONTENT ====");
        let mut file = File::open(file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        println!("{}", content);

        println!("==== FORMULA INFO ====");
        println!("Number of variables: {}", formula.num_variables());
        println!("Number of clauses: {}", formula.num_clauses());

        println!("Clauses:");
        for (i, clause) in formula.get_clauses().iter().enumerate() {
            println!("  Clause {}: {:?}", i + 1, clause);
        }
        println!("=====================");
    }

    #[test]
    fn test_empty_file() {
        let file = create_temp_cnf_file("");
        let mut parser = Parser::new();

        let result = parser.parse_dimacs(file.path());
        assert!(result.is_ok(), "Should not error on empty file");

        let formula = result.unwrap();
        assert_eq!(
            formula.num_clauses(),
            0,
            "Empty file should have zero clauses"
        );
    }

    #[test]
    fn test_comments_only() {
        let content = "c This is a comment\nc Another comment\n";
        let file = create_temp_cnf_file(content);
        let mut parser = Parser::new();

        let result = parser.parse_dimacs(file.path());
        assert!(
            result.is_ok(),
            "Should not error on file with only comments"
        );

        let formula = result.unwrap();
        assert_eq!(
            formula.num_clauses(),
            0,
            "File with only comments should have zero clauses"
        );
    }

    #[test]
    fn test_simple_cnf() {
        let content = "c Example CNF\np cnf 3 2\n1 2 -3 0\n-1 2 3 0\n";
        let file = create_temp_cnf_file(content);
        let mut parser = Parser::new();

        let result = parser.parse_dimacs(file.path());
        assert!(result.is_ok(), "Should not error on valid CNF file");

        let formula = result.unwrap();

        // Print debug information
        print_debug_info(file.path(), &formula);

        assert_eq!(formula.num_clauses(), 2, "Should have two clauses");
        assert_eq!(formula.num_variables(), 3, "Should have three variables");

        let clauses = formula.get_clauses();
        assert_eq!(
            clauses[0],
            vec![1, 2, -3],
            "First clause should be [1, 2, -3]"
        );
        assert_eq!(
            clauses[1],
            vec![-1, 2, 3],
            "Second clause should be [-1, 2, 3]"
        );
    }

    #[test]
    fn test_complicated_cnf() {
        let content = "p cnf 5 3\n1 -2 3 0\n-1 2 -3 4 0\n-4 -5 0\n";
        let file = create_temp_cnf_file(content);
        let mut parser = Parser::new();

        let result = parser.parse_dimacs(file.path());
        assert!(result.is_ok(), "Should not error on valid CNF file");

        let formula = result.unwrap();

        // Print debug information
        print_debug_info(file.path(), &formula);

        assert_eq!(formula.num_clauses(), 3, "Should have three clauses");
        assert_eq!(formula.num_variables(), 5, "Should have five variables");

        let clauses = formula.get_clauses();
        assert_eq!(
            clauses[0],
            vec![1, -2, 3],
            "First clause should be [1, -2, 3]"
        );
        assert_eq!(
            clauses[1],
            vec![-1, 2, -3, 4],
            "Second clause should be [-1, 2, -3, 4]"
        );
        assert_eq!(clauses[2], vec![-4, -5], "Third clause should be [-4, -5]");
    }

    #[test]
    fn test_invalid_literal() {
        let content = "p cnf 3 1\n1 2 xyz 0\n";
        let file = create_temp_cnf_file(content);
        let mut parser = Parser::new();

        let result = parser.parse_dimacs(file.path());
        assert!(result.is_err(), "Should error on invalid literal");
        assert!(parser.has_error(), "Parser should have error flag set");
        assert!(
            parser.get_error().contains("Invalid literal"),
            "Error message should mention invalid literal"
        );
    }

    #[test]
    fn test_nonexistent_file() {
        let mut parser = Parser::new();
        let result = parser.parse_dimacs("/this/file/does/not/exist.cnf");

        assert!(result.is_err(), "Should error on nonexistent file");
        assert!(parser.has_error(), "Parser should have error flag set");
        assert!(
            parser.get_error().contains("Failed to open"),
            "Error message should mention failed to open"
        );
    }
}
