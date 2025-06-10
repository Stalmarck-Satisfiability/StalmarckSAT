# StalmarckSAT

[![CI](https://github.com/Stalmarck-Satisfiability/StalmarckSAT/actions/workflows/build.yml/badge.svg)](https://github.com/Stalmarck-Satisfiability/StalmarckSAT/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/stalmarck-sat.svg)](https://crates.io/crates/stalmarck-sat)
[![PyPI](https://img.shields.io/pypi/v/pystalmarck.svg)](https://pypi.org/project/pystalmarck/)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

StalmarckSAT is a SAT solver based on the Stålmarck Procedure. It is designed with the goal of furthering the research and development of the Stålmarck Procedure, a boolean satisfiability procedure that has been untouched for the last three decades.

## Installation

### As a Rust Crate

Add StalmarckSAT to your `Cargo.toml`:

```toml
[dependencies]
stalmarck-sat = "0.1.0"
```

Or install the command-line tool:

```bash
cargo install stalmarck-sat
```

### As a Python Package

Install from PyPI:

```bash
pip install pystalmarck
```

## Requirements

- Rust 1.70 or later
- Python 3.8+

## Development Setup

Clone and build the project:
```bash
git clone https://github.com/Stalmarck-Satisfiability/StalmarckSAT.git
cd StalmarckSAT
cargo build --release
```

## Usage

### Command Line

StalmarckSAT accepts DIMACS CNF format files and outputs either `SAT` or `UNSAT`:

```bash
# If installed via cargo install
stalmarck_sat formula.cnf

# Or from source
./target/debug/stalmarck_sat formula.cnf
```

### Rust Library

StalmarckSAT can be used as a library in Rust projects:

```rust
use stalmarck_sat::{StalmarckSolver, Result};

fn main() -> Result<()> {
    let mut solver = StalmarckSolver::new();
    let is_satisfiable = solver.solve_from_file("formula.cnf")?;
    
    println!("{}", if is_satisfiable { "SAT" } else { "UNSAT" });
    Ok(())
}
```

### Python Library

Use StalmarckSAT from Python:

```python
import pystalmarck

# Create a solver instance
solver = pystalmarck.PyStalmarckSolver()

# Solve from file
result = solver.solve_from_file("formula.cnf")
print("SAT" if result else "UNSAT")

# Or solve from CNF string
cnf_content = """
c Simple formula: (x1 OR x2) AND (NOT x1 OR x2)
p cnf 2 2
1 2 0
-1 2 0
"""
result = solver.solve_from_string(cnf_content)
print("SAT" if result else "UNSAT")
```

### Command Line Options

```bash
stalmarck_sat [OPTIONS] <FILE_PATH>

Options:
  -v, --verbosity <VERBOSITY>  Verbosity level (0-2) [default: 1]
  -t, --timeout <TIMEOUT>      Timeout in seconds [default: 30.0]
  -h, --help                   Print help
```

### Example DIMACS CNF Format
```
c Simple formula: (x1 OR x2) AND (NOT x1 OR x2)
p cnf 2 2
1 2 0
-1 2 0
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on contributing to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
