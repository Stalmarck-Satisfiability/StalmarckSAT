#!/bin/bash

# Python development script for StalmarckSAT

set -e

PYTHON_DIR="python"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${GREEN}=== $1 ===${NC}"
}

print_error() {
    echo -e "${RED}ERROR: $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

install_dev_deps() {
    print_header "Installing development dependencies"
    pip install -r python/requirements-dev.txt
}

format_python() {
    print_header "Formatting Python code"
    black python/
    isort python/
    echo -e "${GREEN}Python code formatted successfully!${NC}"
}

check_format() {
    print_header "Checking Python formatting"
    black --check --diff python/
    isort --check-only --diff python/
    echo -e "${GREEN}Python formatting check passed!${NC}"
}

lint_python() {
    print_header "Linting Python code"
    flake8 python/ --max-line-length=88 --extend-ignore=E203,W503
    mypy python/pystalmarck/ --ignore-missing-imports
    echo -e "${GREEN}Python linting passed!${NC}"
}

test_python() {
    print_header "Running Python tests"
    cd python && python -m pytest tests/ -v
    echo -e "${GREEN}Python tests passed!${NC}"
}

build_python() {
    print_header "Building Python package"
    maturin develop
    echo -e "${GREEN}Python package built successfully!${NC}"
}

rust_format() {
    print_header "Formatting Rust code"
    cargo fmt
    echo -e "${GREEN}Rust code formatted successfully!${NC}"
}

rust_test() {
    print_header "Running Rust tests"
    cargo test
    echo -e "${GREEN}Rust tests passed!${NC}"
}

all_checks() {
    print_header "Running all checks"
    rust_format
    rust_test
    check_format
    lint_python
    build_python
    test_python
    echo -e "${GREEN}All checks passed!${NC}"
}

help() {
    echo "Python development script for StalmarckSAT"
    echo
    echo "Usage: $0 [command]"
    echo
    echo "Commands:"
    echo "  install-dev     Install development dependencies"
    echo "  format          Format Python code (black + isort)"
    echo "  check-format    Check Python formatting"
    echo "  lint            Lint Python code (flake8 + mypy)"
    echo "  test-python     Run Python tests"
    echo "  build-python    Build Python package with maturin"
    echo "  rust-format     Format Rust code"
    echo "  rust-test       Run Rust tests"
    echo "  all             Run all checks (format, lint, build, test)"
    echo "  help            Show this help message"
}

# Main script logic
case "${1:-help}" in
    install-dev)
        install_dev_deps
        ;;
    format)
        format_python
        ;;
    check-format)
        check_format
        ;;
    lint)
        lint_python
        ;;
    test-python)
        test_python
        ;;
    build-python)
        build_python
        ;;
    rust-format)
        rust_format
        ;;
    rust-test)
        rust_test
        ;;
    all)
        all_checks
        ;;
    help|--help|-h)
        help
        ;;
    *)
        print_error "Unknown command: $1"
        echo
        help
        exit 1
        ;;
esac
