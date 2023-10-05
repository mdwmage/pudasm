use std::fs;

pub fn print_help() {
    println!("PUD-ASM");
    println!("inputs:");
    println!("/path/to/file.psm");
    println!("-----------------------");
    println!("Reads in psm files (Any file with written with correct PUD-ASM syntax). Prints output")
}

pub fn read(path: &str) {
    let file = fs::read_to_string(path).expect("File not readable");
    println!("File: {}", path);
    println!("With text:\n{}", file)
}