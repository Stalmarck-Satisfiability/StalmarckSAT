"""Tests for PyStalmarck"""

import os
import tempfile

import pytest

from pystalmarck import PyStalmarckSolver, solve_cnf_file


def create_test_cnf(content: str) -> str:
    """Create a temporary CNF file with the given content"""
    fd, path = tempfile.mkstemp(suffix=".cnf")
    try:
        with os.fdopen(fd, "w") as f:
            f.write(content)
        return path
    except Exception:
        os.close(fd)
        raise


class TestPyStalmarckSolver:
    """Test the PyStalmarckSolver class"""

    def test_solver_creation(self):
        """Test that we can create a solver instance"""
        solver = PyStalmarckSolver()
        assert solver is not None
        assert repr(solver) == "PyStalmarckSolver()"

    def test_simple_sat_formula(self):
        """Test solving a simple satisfiable formula"""
        # (x1 OR x2) AND (NOT x1 OR x2) - satisfiable
        cnf_content = """c Simple satisfiable formula
                        p cnf 2 2
                        1 2 0
                        -1 2 0
                        """
        cnf_file = create_test_cnf(cnf_content)
        try:
            solver = PyStalmarckSolver()
            result = solver.solve_file(cnf_file)
            assert result is True
        finally:
            os.unlink(cnf_file)

    def test_simple_unsat_formula(self):
        """Test solving a simple unsatisfiable formula"""
        # x1 AND NOT x1 - unsatisfiable
        cnf_content = """c Simple unsatisfiable formula
                        p cnf 1 2
                        1 0
                        -1 0
                        """
        cnf_file = create_test_cnf(cnf_content)
        try:
            solver = PyStalmarckSolver()
            result = solver.solve_file(cnf_file)
            assert result is False
        finally:
            os.unlink(cnf_file)

    def test_verbosity_setting(self):
        """Test setting verbosity levels"""
        solver = PyStalmarckSolver()
        # Should not raise any exceptions
        solver.set_verbosity(0)
        solver.set_verbosity(1)
        solver.set_verbosity(2)

    def test_timeout_setting(self):
        """Test setting timeout"""
        solver = PyStalmarckSolver()
        # Should not raise any exceptions
        solver.set_timeout(10.0)
        solver.set_timeout(60.0)

    def test_nonexistent_file(self):
        """Test that solving a nonexistent file raises an error"""
        solver = PyStalmarckSolver()
        with pytest.raises(RuntimeError):
            solver.solve_file("nonexistent_file.cnf")


class TestConvenienceFunctions:
    """Test the convenience functions"""

    def test_solve_cnf_file_sat(self):
        """Test the solve_cnf_file convenience function with SAT formula"""
        cnf_content = """c Simple satisfiable formula
                        p cnf 2 2
                        1 2 0
                        -1 2 0
                        """
        cnf_file = create_test_cnf(cnf_content)
        try:
            result = solve_cnf_file(cnf_file)
            assert result is True
        finally:
            os.unlink(cnf_file)

    def test_solve_cnf_file_with_options(self):
        """Test the solve_cnf_file function with verbosity and timeout"""
        cnf_content = """c Simple satisfiable formula
                        p cnf 2 2
                        1 2 0
                        -1 2 0
                        """
        cnf_file = create_test_cnf(cnf_content)
        try:
            result = solve_cnf_file(cnf_file, verbosity=1, timeout=5.0)
            assert result is True
        finally:
            os.unlink(cnf_file)

    def test_solve_cnf_file_nonexistent(self):
        """Test that solve_cnf_file raises error for nonexistent file"""
        with pytest.raises(RuntimeError):
            solve_cnf_file("nonexistent_file.cnf")
