// Weird little thing.
// Mimics an instruction set
// Reads file from cla and computes

/* Instructions
**  P(u32) // Push (Pushes number to current location in memory)
**  U(u32) // Pop (Resets a number to 0 at current location)
**  D(u32) // Display (Print value at specified location)
**  A(u32, u32) // Add (Adds two values in memory and pushes result to current location)
**  S(u32, u32) // Subtract (Subtracts two values in memory and pushes result to current location)
**  M(usize) // Move (Moves pointer to new location in memory)
*/

mod utilities;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    check_args(args);
}

fn check_args(args: Vec<String>) {
    match args[1].as_str() {
        "help" => utilities::print_help(),
        "read" => utilities::read(&args[2]),
        _ => compute(&args[1]),
    }
}

fn compute(path: &str) {
    let mut ptr: usize = 0;
    let mut mem: [u32; 128] = [0; 128];
    let file = fs::read_to_string(path).expect("File not readable");
    for line in file.lines() {
        let instructions: Vec<&str> = line.split(' ').collect();
        match instructions[0].trim() {
            "P" => mem[ptr] = read_u32(instructions[1]),
            "U" => mem[ptr] = 0,
            "D" => println!("{}", mem[ptr]),
            "A" => mem[ptr] = mem[read_usize(instructions[1])] + mem[read_usize(instructions[2])],
            "S" => mem[ptr] = mem[read_usize(instructions[1])] - mem[read_usize(instructions[2])],
            "M" => ptr = read_usize(instructions[1]),
            _ => panic!("Invalid keyword"),
        }
    }
}

fn read_usize(input: &str) -> usize { input.trim().parse().expect("Failed to parse and integer") }
fn read_u32(input: &str) -> u32 { input.trim().parse().expect("Failed to parse and integer") }