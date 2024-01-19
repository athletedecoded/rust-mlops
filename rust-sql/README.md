# Rust x SQL

**Setup**

```
cd rust-sql
sqlite3 --version
```

**Run CLI**
```
# Execute (for CREATE, ALTER)
cargo run execute --db <db_name> --q <query>
cargo run execute --db sharks --q "CREATE TABLE cool_sharks(id integer NOT NULL, name text NOT NULL, species text NOT NULL);"
cargo run execute --db sharks --q "ALTER TABLE cool_sharks ADD COLUMN age integer;"
cargo run execute --db sharks --q "UPDATE cool_sharks SET age = 272 WHERE id=1;"

# Insert
cargo run insert --db <db_name> --table <table_name> --data <data>
cargo run insert --db sharks --table cool_sharks --data "(1,'Bruce','Great White')"

# Read
cargo run read --db <db_name> --table <table_name> 
cargo run read --db sharks --table cool_sharks

# Drop table
cargo run drop --db <db_name> --table <table_name> 
cargo run drop --db sharks --table cool_sharks
```

**Unit Tests**

```
make tests
```

## Future ToDos:
- [ ] Add struct parsing

## Resources

- [Rust Cookbook: SQLite](https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html)
- [rusqlite docs](https://docs.rs/rusqlite/latest/rusqlite/)