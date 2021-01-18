use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found.")
}

pub fn num_of_lines(s: &String) -> usize {
    s.lines().collect::<Vec<&str>>().len()
}

pub fn get_nth_line(s: &String, n: usize) -> String {
    let lines = s.lines().collect::<Vec<&str>>();

    return String::from(lines[n - 1]);
}

pub fn get_nth_word(s: &String, n: usize) -> String {
    let words = s.split(" ").collect::<Vec<&str>>();

    return String::from(words[n - 1]);
}
