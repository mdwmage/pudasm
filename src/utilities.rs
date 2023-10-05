use std::fs;

pub fn print_help() {
    println!("PUD-ASM");
    println!("inputs:");
    println!("/path/to/file.psm");
    println!("-----------------------");
    println!(
        "Reads in psm files (Any file with written with correct PUD-ASM syntax). Prints output"
    )
}

pub fn read(path: &str) {
    let file = fs::read_to_string(path).expect("File not readable");
    println!("File: {}", path);
    println!("With text:\n{}", file)
}

pub fn docs() {
    let docs = "PUD-ASM docs
Writing PUD-ASM files:

Keywords:
    P(u32) - Push (Pushes number to current location in memory)
    U(u32) -  Pop (Resets a number to 0 at current location)
    D(u32) -  Display (Print value at specified location)
    A(u32, u32) -  Add (Adds two values in memory and pushes result to current location)
    S(u32, u32) -  Subtract (Subtracts two values in memory and pushes result to current location)
    M(usize) -  Move (Moves pointer to new location in memory)

To write a PUD-ASM file, simply create a file with the ending \".psm\" (It does not have to end in psm, this is done simply to differentiate the file)

Running PUD-ASM files:

To run a PUD-ASM file, invoke the pudc compiler and give the path to the .psm file as an argument";
    println!("{docs}");
}
