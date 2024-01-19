/*
Searches a path for duplicate files first in parallel and then in serial
*/
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Kahlia Hogg",
    about = "Compare serial and variable thread exectution of program",
    after_help = "Example: cargo run parallel --path /src/data/"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Parallel {
        #[clap(short, long, default_value = "./src/data/")]
        path: String,
        #[clap(short, long)]
        threads: Option<usize>,
    },
    Serial {
        #[clap(short, long, default_value = "./src/data/")]
        path: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Parallel { path, threads }) => match threads {
            Some(threads) => {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build_global()
                    .unwrap();
                let num_threads = rayon::current_num_threads();
                println!("Running parallel version of the program with {num_threads} threads...");
                parallel::run_parallel(&path);
            }
            None => {
                let num_threads = rayon::current_num_threads();
                println!("Running parallel version of the program with {num_threads} threads...");
                parallel::run_parallel(&path);
            }
        },
        Some(Commands::Serial { path }) => {
            println!("Running serial version of the program...");
            parallel::run_serial(&path);
        }
        None => println!("No command specified"),
    }
}
