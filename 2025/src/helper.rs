use std::fs;

pub fn read_input(file_path: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents.lines().map(|s| s.to_string()).collect()
}