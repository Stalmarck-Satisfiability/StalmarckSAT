# StalmarckSAT

[![CI](https://github.com/Stalmarck-Satisfiability/StalmarckSAT/actions/workflows/build.yml/badge.svg)](https://github.com/Stalmarck-Satisfiability/StalmarckSAT/actions/workflows/build.yml)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

StalmarckSAT is a SAT solver based on the Stålmarck Procedure. It is designed with the goal of furthering the research and development of the Stålmarck Procedure, a boolean satisfiability procedure that has been untouched for the last three decades.

## Requirements

- Rust 1.70 or later

## Setup

Clone and build the project:
```bash
git clone https://github.com/Stalmarck-Satisfiability/StalmarckSAT.git
cd StalmarckSAT
cargo build --release
```

## Usage

StalmarckSAT accepts DIMACS CNF format files and outputs either `SAT` or `UNSAT`:

```bash
./target/debug/stalmarck_sat formula.cnf
```

Options:
```bash
./target/debug/stalmarck_sat [OPTIONS] <FILE_PATH>

Options:
  -v, --verbosity <VERBOSITY>  Verbosity level (0-2) [default: 0]
  -t, --timeout <TIMEOUT>      Timeout in seconds [default: 30.0]
  -h, --help                   Print help
```

Example DIMACS CNF file:
```
c Simple formula: (x1 OR x2) AND (NOT x1 OR x2)
p cnf 2 2
1 2 0
-1 2 0
```

StalmarckSAT can also be used as a library:
```rust
use stalmarck_sat::{StalmarckSolver, Result};

fn main() -> Result<()> {
    let mut solver = StalmarckSolver::new();
    let is_satisfiable = solver.solve_from_file("formula.cnf")?;
    
    println!("{}", if is_satisfiable { "SAT" } else { "UNSAT" });
    Ok(())
}
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on contributing to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
