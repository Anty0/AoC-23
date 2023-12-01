use std::fs;

const INPUT_DIR: &str = "inputs";

pub fn input_files(prefix: &str) -> Vec<String> {
    // Find all files in the input directory that start with the prefix
    let mut files = Vec::new();

    let full_prefix = format!("{}/{}", INPUT_DIR, prefix);
    let full_prefix = full_prefix.as_str();

    let paths = fs::read_dir(INPUT_DIR).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.starts_with(full_prefix) {
            files.push(path_str.to_owned());
        }
    }
    files.sort();
    files
}
