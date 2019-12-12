use std::fs;

pub fn read_input(filename: &str) -> String {
    let file_content = fs::read_to_string(format!("../input/{}", filename));
    return file_content.unwrap().trim().to_string()
}
