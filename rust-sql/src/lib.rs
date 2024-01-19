use rusqlite::{Connection, Result};
// connect to db
pub fn connect_db(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;
    Ok(conn)
}

// Unit tests
#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    #[test]
    fn test_create1() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("execute")
            .arg("--db")
            .arg("sharks")
            .arg("--q")
            .arg("CREATE TABLE cool_sharks (id INTEGER PRIMARY KEY, name TEXT NOT NULL, species TEXT NOT NULL);")
            .assert()
            .success();
    }
    
    #[test]
    fn test_create2() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("execute")
            .arg("--db")
            .arg("sharks")
            .arg("--q")
            .arg("CREATE TABLE shark_specs (id INTEGER PRIMARY KEY, age INTEGER NOT NULL, length INTEGER NOT NULL);")
            .assert()
            .success();
    }

    #[test]
    fn test_insert1() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("insert")
            .arg("--db")
            .arg("sharks")
            .arg("--table")
            .arg("cool_sharks")
            .arg("--data")
            .arg("(1, 'Bruce', 'Great White Shark')")
            .assert()
            .success();
    }
    #[test]
    fn test_insert2() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("insert")
            .arg("--db")
            .arg("sharks")
            .arg("--table")
            .arg("shark_specs")
            .arg("--data")
            .arg("(1, 72, 207)")
            .assert()
            .success();
    }

    #[test]
    fn test_update() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("execute")
            .arg("--db")
            .arg("sharks")
            .arg("--q")
            .arg("UPDATE cool_sharks SET name = 'Jaws' WHERE id = 1;")
            .assert()
            .success();
    }

    #[test]
    fn test_read() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("read")
            .arg("--db")
            .arg("sharks")
            .arg("--table")
            .arg("cool_sharks")
            .assert()
            .success();
    }

    #[test]
    fn test_join() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("execute")
            .arg("--db")
            .arg("sharks")
            .arg("--q")
            .arg("SELECT * FROM cool_sharks INNER JOIN shark_specs ON cool_sharks.id = shark_specs.id;")
            .assert()
            .success();
    }

    #[test]
    fn test_drop1() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("drop")
            .arg("--db")
            .arg("sharks")
            .arg("--table")
            .arg("cool_sharks")
            .assert()
            .success();
    }
    #[test]
    fn test_drop2() {
        let mut cmd = Command::cargo_bin("rust-sql").unwrap();
        cmd.arg("drop")
            .arg("--db")
            .arg("sharks")
            .arg("--table")
            .arg("shark_specs")
            .assert()
            .success();
    }

}