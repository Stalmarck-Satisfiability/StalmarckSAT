use std::path::Path;
use std::process::Command;

/// Test case structure for CNF files
struct TestCase {
    filename: &'static str,
    expected_result: &'static str,
    description: &'static str,
}

/// Collection of test cases with expected results
const TEST_CASES: &[TestCase] = &[
    TestCase {
        filename: "trivial_sat.cnf",
        expected_result: "SAT",
        description: "Single positive literal",
    },
    TestCase {
        filename: "trivial_unsat.cnf",
        expected_result: "UNSAT",
        description: "Single negative literal",
    },
    TestCase {
        filename: "simple_sat.cnf",
        expected_result: "SAT",
        description: "Simple satisfiable formula",
    },
    TestCase {
        filename: "simple_unsat.cnf",
        expected_result: "UNSAT",
        description: "Simple unsatisfiable formula",
    },
    TestCase {
        filename: "3sat_sat.cnf",
        expected_result: "SAT",
        description: "3-SAT satisfiable formula",
    },
    TestCase {
        filename: "3sat_unsat.cnf",
        expected_result: "UNSAT",
        description: "3-SAT unsatisfiable formula",
    },
];

/// Helper function to build the solver binary
fn ensure_binary_built() {
    let output = Command::new("cargo")
        .args(&["build", "--bin", "stalmarck_sat"])
        .output()
        .expect("Failed to build binary");

    if !output.status.success() {
        panic!(
            "Failed to build binary:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Helper function to run the solver on a CNF file
fn run_solver_on_file(cnf_file: &str) -> (String, i32) {
    let binary_path = "./target/debug/stalmarck_sat";
    let cnf_path = format!("tests/cnf_files/{}", cnf_file);

    let output = Command::new(binary_path)
        .arg(&cnf_path)
        .output()
        .expect("Failed to execute solver");

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, exit_code)
}

/// Test individual CNF files
#[test]
fn test_individual_cnf_files() {
    ensure_binary_built();

    for test_case in TEST_CASES {
        println!(
            "Testing: {} - {}",
            test_case.filename, test_case.description
        );

        let cnf_path = format!("tests/cnf_files/{}", test_case.filename);
        assert!(
            Path::new(&cnf_path).exists(),
            "CNF file {} does not exist",
            cnf_path
        );

        let (output, exit_code) = run_solver_on_file(test_case.filename);

        // Check the output matches expected result
        assert_eq!(
            output, test_case.expected_result,
            "Test case '{}' failed: expected '{}', got '{}'",
            test_case.description, test_case.expected_result, output
        );

        // Check exit codes (10 for SAT, 20 for UNSAT)
        let expected_exit_code = match test_case.expected_result {
            "SAT" => 10,
            "UNSAT" => 20,
            _ => panic!("Invalid expected result: {}", test_case.expected_result),
        };

        assert_eq!(
            exit_code, expected_exit_code,
            "Test case '{}' failed: expected exit code {}, got {}",
            test_case.description, expected_exit_code, exit_code
        );

        println!("✓ {} passed", test_case.filename);
    }
}

/// Test solver with invalid file
#[test]
fn test_invalid_file() {
    ensure_binary_built();

    let binary_path = "./target/debug/stalmarck_sat";
    let invalid_path = "tests/cnf_files/nonexistent.cnf";

    let output = Command::new(binary_path)
        .arg(invalid_path)
        .output()
        .expect("Failed to execute solver");

    // Should exit with error code 1
    assert_eq!(output.status.code().unwrap_or(-1), 1);

    // Should contain error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error"));
}

/// Benchmark test to ensure solver completes in reasonable time
#[test]
fn test_performance() {
    ensure_binary_built();

    let start = std::time::Instant::now();

    for test_case in TEST_CASES {
        let _ = run_solver_on_file(test_case.filename);
    }

    let duration = start.elapsed();

    // All test cases should complete within 10 seconds
    assert!(
        duration.as_secs() < 10,
        "Solver took too long: {:?}",
        duration
    );

    println!("All tests completed in {:?}", duration);
}

/// Test that solver handles timeout correctly
#[test]
fn test_timeout_handling() {
    ensure_binary_built();

    let binary_path = "./target/debug/stalmarck_sat";
    let cnf_path = "tests/cnf_files/simple_sat.cnf";

    // Test with very short timeout
    let output = Command::new(binary_path)
        .args(&["-t", "0.001", cnf_path])
        .output()
        .expect("Failed to execute solver with timeout");

    // Should either complete normally or handle timeout gracefully
    let exit_code = output.status.code().unwrap_or(-1);
    assert!(
        exit_code == 10 || exit_code == 20 || exit_code == 1,
        "Unexpected exit code with timeout: {}",
        exit_code
    );
}
