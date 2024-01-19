/*
Rust CLI for SQLite CRUD operations
*/

use clap::Parser;
use rusqlite::Result;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Kahlia Hogg",
    about = "A CLI tool SQLite",
    after_help = "Example: cargo run execute --db <db_name> --query <query>"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Execute {
        #[clap(long)]
        db: String,
        #[clap(short, long)]
        q: String,
    },
    Insert {
        #[clap(long)]
        db: String,
        #[clap(long)]
        table: String,
        #[clap(long)]
        data: String,
    },
    Read {
        #[clap(long)]
        db: String,
        #[clap(long)]
        table: String,
    },
    Drop {
        #[clap(long)]
        db: String,
        #[clap(long)]
        table: String,
    }
}

// Main fxn returns result or error
fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Execute { db, q }) => {
            let conn = rust_sql::connect_db(&(db + ".db"))?;
            conn.execute(&q, ())?;
            println!("SUCCESS: Executed query {:?}", q);
            Ok(())
        }
        Some(Commands::Insert { db, table, data }) => {
            let conn = rust_sql::connect_db(&(db + ".db"))?;
            let q = format!("INSERT INTO {} VALUES {};", table, data);
            conn.execute(&q, ())?;
            println!("SUCCESS: Executed query {:?}", q);
            Ok(())
        }
        Some(Commands::Read { db, table }) => {
            let conn = rust_sql::connect_db(&(db + ".db"))?;
            let q = format!("SELECT * FROM {}", table);
            let mut stmt = conn.prepare(&q)?;
            // let cols = stmt.column_names();
            let mut rows = stmt.query([])?;
            println!("Table: {:?}:", table);
            while let Some(row) = rows.next()? {
                println!("{:?}", row)
            }
            Ok(())
        }
        Some(Commands::Drop { db, table }) => {
            let conn = rust_sql::connect_db(&(db + ".db"))?;
            let q = format!("DROP TABLE {}", table);
            conn.execute(&q, ())?;
            println!("SUCCESS: Dropped table {:?}", table);
            Ok(())
        }
        None => {
            println!("No command specified");
            Ok(())
        }
    }
}
