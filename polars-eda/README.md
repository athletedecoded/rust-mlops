# Polars EDA

Rust CLI tool to support data exploratory for .csv and .json datafiles using [Polars](https://github.com/pola-rs/polars)

## Useage

**For CSV files**
```
# If csv file includes headers
cargo run csv --path /path/to/data.csv --headers

# If csv doesn't have headers
cargo run csv --path /path/to/data.csv
```

**For JSON files**
```
cargo run json --path /path/to/data.csv
```

