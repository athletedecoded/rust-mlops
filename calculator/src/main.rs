//A command-line calculator
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Kahlia Hogg", about = "A simple calculator")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Kahlia Hogg")]
    Calc {
        #[clap(short, long)]
        a: f64,
        #[clap(short, long)]
        b: f64,
        #[clap(short, long)]
        op: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Calc { a, b, op }) => match op.as_str() {
            "+" => println!("Result: {:.2}", calculator::add(a, b)),
            "-" => println!("Result: {:.2}", calculator::subtract(a, b)),
            "x" => println!("Result: {:.2}", calculator::multiply(a, b)),
            "/" => println!("Result: {:.2}", calculator::divide(a, b)),
            _ => println!("Invalid operation"),
        },
        None => println!("No command given"),
    }
}
