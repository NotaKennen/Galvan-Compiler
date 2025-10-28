use std::fs::read_to_string;

pub fn load_file(filename: &str) -> Option<String> {
    let contents = read_to_string(filename);

    if contents.is_ok() {
        Some(contents.unwrap())
    } else {
        None
    }
    
}