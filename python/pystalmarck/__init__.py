"""
PyStalmarck - Python bindings for StalmarckSAT

A Python interface to the StalmarckSAT solver, which implements
Stålmarck's method for Boolean satisfiability solving.
"""

from .pystalmarck import PyStalmarckSolver, solve_cnf_file

__version__ = "0.1.0"
__author__ = "Liam Davis, Sergei Leonov"

__all__ = ["PyStalmarckSolver", "solve_cnf_file"]
