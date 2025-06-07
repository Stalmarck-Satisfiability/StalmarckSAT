# Contributing to StalmarckSAT

Thank you for your interest in contributing to StalmarckSAT! This document provides guidelines for contributors.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contribution Guidelines](#contribution-guidelines)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Issue Reporting](#issue-reporting)
- [Architecture Overview](#architecture-overview)

## Getting Started

### Prerequisites

- Rust 1.70 or later

### Development Setup

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/StalmarckSAT.git
   cd StalmarckSAT
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests** to ensure everything works:
   ```bash
   cargo test
   ```

## Contribution Guidelines

### Types of Contributions

We welcome several types of contributions:

- **Bug fixes**: Fix issues in the codebase
- **Features**: Add new functionality to the solver
- **Performance improvements**: Optimize existing algorithms
- **Documentation**: Improve code documentation
- **Tests**: Add or improve test coverage

### Before You Start

1. **Check existing issues** to see if your contribution is already being worked on
2. **Open an issue** to discuss major changes before implementing them
3. **Keep changes focused**: Submit one feature/fix per pull request

## Code Style

### Rust Conventions

We follow standard Rust conventions:

- Use `cargo fmt` to format code
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Code Formatting

```bash
# Format code
cargo fmt

```

### Documentation

- Add doc comments (`///`) for all public APIs
- Include examples in doc comments where helpful
- Update README.md if adding user-facing features

Example:
```rust
/// Applies simple rules to propagate variable assignments
/// 
/// This function implements the 0-saturation phase of Stålmarck's method,
/// applying rules 1-7 until no more progress can be made.
/// 
/// # Returns
/// 
/// Returns `true` if any assignments were made, `false` otherwise.
pub fn apply_simple_rules(&mut self) -> bool {
    // Implementation...
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test solver_test

# Run integration tests only
cargo test --test integration_tests
```

### Test Categories

1. **Unit Tests**: Test individual functions and modules
   - Located in `src/*/mod.rs` files or separate `*_test.rs` files
   - Test internal logic and edge cases

2. **Integration Tests**: Test the solver end-to-end
   - Located in `tests/` directory
   - Test with real CNF files

### Writing Tests

When adding new functionality:

1. Write unit tests for new functions
2. Add integration tests for new features
3. Test edge cases and error conditions
4. Use descriptive test names

Example test:
```rust
#[test]
fn test_simple_rule_application() {
    let mut solver = Solver::new();
    // Setup test conditions
    solver.current_triplets.push((
        TripletVar::Const(false), 
        TripletVar::Var(1), 
        TripletVar::Var(2)
    ));
    
    // Apply the rule
    solver.apply_simple_rules();
    
    // Verify expected outcomes
    assert_eq!(solver.assignments.get(&1), Some(&true));
    assert_eq!(solver.assignments.get(&2), Some(&false));
}
```

### Test Files

For integration tests, add CNF files to `tests/cnf_files/` with descriptive names:
- `description_sat.cnf` for satisfiable formulas
- `description_unsat.cnf` for unsatisfiable formulas

## Submitting Changes

### Pull Request Process

1. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** with clear, focused commits:
   ```bash
   git add .
   git commit -m "Add feature: brief description of change"
   ```

3. **Update from upstream** before submitting:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

4. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create a pull request** on GitHub

### Pull Request Guidelines

- **Title**: Use a clear, descriptive title
- **Description**: Explain what changes you made and why, be detailed in the description
- **Link issues**: Reference any related issues
- **Tests**: Ensure all tests pass
- **Documentation**: Update documentation if needed

## Issue Reporting

### Bug Reports

When reporting bugs, include:

1. Clear description of the issue
2. Steps to reproduce the problem
3. Expected behavior vs actual behavior
4. Environment information:
   - Rust version (`rustc --version`)
   - Operating system
   - Input file (if applicable)
5. Error messages or logs

### Feature Requests

When requesting features:

1. Describe the feature and its purpose
2. Explain the use case or problem it solves
3. Provide examples if possible
4. Consider implementation complexity

## Architecture Overview

Understanding the codebase structure will help you contribute effectively:

```
src/
├── lib.rs              # Library entry point
├── main.rs             # CLI application entry point
├── error.rs            # Error handling
├── core/               # Core algorithm implementation
│   ├── mod.rs
│   ├── formula.rs      # Formula representation
│   ├── formula_test.rs # Formula tests
│   └── stalmarck.rs    # Main solver interface
├── parser/             # Input parsing
│   ├── mod.rs
│   ├── dimacs.rs       # DIMACS CNF parser
│   └── dimacs_test.rs  # Parser tests
└── solver/             # Core solving logic
    ├── mod.rs
    ├── solver.rs       # Rule application and search
    └── solver_test.rs  # Solver tests
```

### Key Components

1. **`Solver`**: Core algorithm implementation
2. **`Formula`**: Encodes parsed CNF into triplet form
3. **`Parser`**: Handles DIMACS CNF input format

### Adding New Features

When adding features:

1. **Consider the architecture**: Where does your feature fit?
2. **Maintain separation of concerns**: Keep parsing, solving, and I/O separate
3. **Add appropriate tests**: Unit tests for internal logic, integration tests for user-facing features
4. **Update documentation**: Both code comments and user documentation

## Questions and Support

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Code Review**: All pull requests receive thorough review
