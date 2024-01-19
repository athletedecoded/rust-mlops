use assert_cmd::Command;

#[test]
// test the runtime of the parallel version with default threads
fn test_parallel() {
    let mut cmd = Command::cargo_bin("parallel").unwrap();
    cmd.arg("parallel").arg("--path").arg("./src/data/");
    cmd.assert().success();
}

#[test]
// test the runtime of the parallel version with 8 threads
fn test_parallel8() {
    let mut cmd = Command::cargo_bin("parallel").unwrap();
    cmd.arg("parallel")
        .arg("--path")
        .arg("./src/data/")
        .arg("--threads")
        .arg("8");
    cmd.assert().success();
}

#[test]
// test the runtime of the serial version of the program
fn test_serial() {
    let mut cmd = Command::cargo_bin("parallel").unwrap();
    cmd.arg("serial").arg("--path").arg("./src/data/");
    cmd.assert().success();
}
