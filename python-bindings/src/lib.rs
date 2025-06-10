use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use stalmarck_sat::StalmarckSolver;

/// A Python wrapper for the StalmarckSolver
#[pyclass]
struct PyStalmarckSolver {
    solver: StalmarckSolver,
}

#[pymethods]
impl PyStalmarckSolver {
    /// Create a new StalmarckSolver instance
    #[new]
    fn new() -> Self {
        Self {
            solver: StalmarckSolver::new(),
        }
    }

    /// Solve a CNF file and return whether it's satisfiable
    ///
    /// Args:
    ///     file_path: Path to the DIMACS CNF file
    ///
    /// Returns:
    ///     bool: True if satisfiable, False if unsatisfiable
    ///
    /// Raises:
    ///     RuntimeError: If there's an error parsing or solving the file
    fn solve_file(&mut self, file_path: &str) -> PyResult<bool> {
        match self.solver.solve_from_file(file_path) {
            Ok(result) => Ok(result),
            Err(e) => Err(PyRuntimeError::new_err(format!("Solver error: {}", e))),
        }
    }

    /// Set the verbosity level (0-2)
    ///
    /// Args:
    ///     level: Verbosity level (0=quiet, 1=normal, 2=verbose)
    fn set_verbosity(&mut self, level: i32) {
        self.solver.set_verbosity(level);
    }

    /// Set the timeout in seconds
    ///
    /// Args:
    ///     timeout: Timeout in seconds
    fn set_timeout(&mut self, timeout: f64) {
        self.solver.set_timeout(timeout);
    }

    /// Check if the last solved formula was a tautology
    ///
    /// Returns:
    ///     bool: True if the formula was a tautology
    fn is_tautology(&self) -> bool {
        self.solver.is_tautology()
    }

    /// String representation of the solver
    fn __repr__(&self) -> String {
        "PyStalmarckSolver()".to_string()
    }
}

/// Solve a CNF file directly (convenience function)
///
/// Args:
///     file_path: Path to the DIMACS CNF file
///     verbosity: Verbosity level (0-2), default 0
///     timeout: Timeout in seconds, default 30.0
///
/// Returns:
///     bool: True if satisfiable, False if unsatisfiable
///
/// Raises:
///     RuntimeError: If there's an error parsing or solving the file
#[pyfunction]
#[pyo3(signature = (file_path, verbosity=None, timeout=None))]
fn solve_cnf_file(file_path: &str, verbosity: Option<i32>, timeout: Option<f64>) -> PyResult<bool> {
    let mut solver = StalmarckSolver::new();

    if let Some(v) = verbosity {
        solver.set_verbosity(v);
    }

    if let Some(t) = timeout {
        solver.set_timeout(t);
    }

    match solver.solve_from_file(file_path) {
        Ok(result) => Ok(result),
        Err(e) => Err(PyRuntimeError::new_err(format!("Solver error: {}", e))),
    }
}

/// Python module for StalmarckSAT
#[pymodule]
fn pystalmarck(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyStalmarckSolver>()?;
    m.add_function(wrap_pyfunction!(solve_cnf_file, m)?)?;

    // Add module metadata
    m.add("__version__", "0.1.0")?;
    m.add("__author__", "Liam Davis, Sergei Leonov")?;

    Ok(())
}
