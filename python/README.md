# PyStalmarck

Python bindings for StalmarckSAT - A SAT solver based on the Stålmarck procedure.

## Installation

### From Source

```bash
# Install maturin (the build tool for Python extensions written in Rust)
pip install maturin

# Build and install the package
maturin develop
```

### From PyPI

```bash
pip install pystalmarck
```

## Usage

### Basic Usage

```python
from pystalmarck import PyStalmarckSolver

# Create a solver instance
solver = PyStalmarckSolver()

# Solve a CNF file
is_satisfiable = solver.solve_file("formula.cnf")

if is_satisfiable:
    print("SAT")
else:
    print("UNSAT")
```

### Convenience Function

```python
from pystalmarck import solve_cnf_file

# Quick one-liner to solve a file
result = solve_cnf_file("formula.cnf", verbosity=1, timeout=60.0)
print("SAT" if result else "UNSAT")
```

### Configuration Options

```python
solver = PyStalmarckSolver()

# Set verbosity level (0=quiet, 1=normal, 2=verbose)
solver.set_verbosity(1)

# Set timeout in seconds
solver.set_timeout(30.0)

# Solve the formula
result = solver.solve_file("formula.cnf")

# Check if the formula was a tautology
if solver.is_tautology():
    print("The formula is a tautology")
```

## Input Format

PyStalmarck accepts DIMACS CNF format files:

```
c This is a comment
p cnf 3 2
1 -3 0
2 3 -1 0
```

Where:
- `c` lines are comments
- `p cnf <variables> <clauses>` declares the problem
- Each clause ends with `0`
- Positive numbers represent variables, negative numbers represent negated variables

## API Reference

### PyStalmarckSolver

- `__init__()`: Create a new solver instance
- `solve_file(file_path: str) -> bool`: Solve a CNF file, returns True if SAT, False if UNSAT
- `set_verbosity(level: int)`: Set verbosity level (0-2)
- `set_timeout(timeout: float)`: Set timeout in seconds
- `is_tautology() -> bool`: Check if the last solved formula was a tautology

### Functions

- `solve_cnf_file(file_path: str, verbosity: int = 0, timeout: float = 30.0) -> bool`: Convenience function to solve a CNF file

## License

MIT License - see the main project LICENSE file for details.
