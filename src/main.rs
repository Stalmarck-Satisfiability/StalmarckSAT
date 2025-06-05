use clap::Parser;
// Import from our library
use stalmarck_sat::Result;
use stalmarck_sat::StalmarckSolver;

/// Stålmarck's SAT solver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the CNF file to solve
    #[arg(required = true)]
    file_path: String,

    /// Verbosity level (0-2)
    #[arg(short, long, default_value_t = 0)]
    verbosity: i32,

    /// Timeout in seconds
    #[arg(short, long, default_value_t = 30.0)]
    timeout: f64,
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Parse command-line arguments
    let args = Args::parse();

    // Create a new solver instance
    let mut solver = StalmarckSolver::new();
    solver.set_verbosity(args.verbosity);
    solver.set_timeout(args.timeout);

    // Solve the formula
    match solver.solve_from_file(&args.file_path) {
        Ok(is_satisfiable) => {
            // Print the result
            println!("{}", if is_satisfiable { "SAT" } else { "UNSAT" });

            // Return standard SAT solver exit codes
            if is_satisfiable {
                std::process::exit(10);
            } else {
                std::process::exit(20);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
