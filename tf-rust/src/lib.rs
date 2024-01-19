use std::fs::File;

// Function to read a JSON file and return the value of a key
pub fn get_label(idx: &str) -> String {
    // Open the file in read-only mode with buffer.
    let file = File::open("./src/labels.json").expect("check labels.json file");
    // Read JSON
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    // Get the value of the key as string
    let label = json[idx].as_str().unwrap();
    // Return the value
    label.to_string()
}
