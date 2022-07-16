use clap::Parser;
use maze;
use rand::SeedableRng;

/// Generate a maze with the provided dimensions
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The number of columns in the grid
    #[clap(short, long, value_parser)]
    columns: usize,
    /// The number of rows in the grid
    #[clap(short, long, value_parser)]
    rows: usize,
    /// A number to use as the seed
    #[clap(short, long, value_parser)]
    seed: Option<u64>,
}

fn main() {
    let args = Cli::parse();

    let mut rng = match args.seed {
        Some(seed) => rand::rngs::StdRng::seed_from_u64(seed),
        None => rand::rngs::StdRng::from_entropy(),
    };

    use std::time::Instant;
    let now = Instant::now();

    let grid = maze::gen(args.columns, args.rows, &mut rng);

    let elapsed = now.elapsed();

    let size = args.columns * args.rows;
    println!("({size}) {grid:?}");
    println!("Elapsed: {elapsed:.2?}");
}
