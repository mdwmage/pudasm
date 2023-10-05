// Weird little thing.
// Mimics an instruction set
// Reads file from cla and computes

/* Instructions
**  P(i32), // Push
**  U(i32), // Pop
**  D(i32) // Display
**  A(i32, i32), // Add
**  S(i32, i32), // Subtract
**  M(i32) // Move
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
        match instructions[0] {
            "P" => mem[ptr] = read_u32(instructions[1]),
            "U" => mem[ptr] = 0,
            "D" => println!("{}", mem[ptr]),
            "A" => mem[ptr] = mem[read_usize(instructions[1])] + mem[read_usize(instructions[2])],
            "S" => mem[ptr] = if read_usize(instructions[1]) > read_usize(instructions[2]) { mem[read_usize(instructions[1])] - mem[read_usize(instructions[2])] } else { mem[read_usize(instructions[2])] - mem[read_usize(instructions[1])] },
            "M" => ptr = read_usize(instructions[1]),
            _ => panic!("Invalid keyword"),
        }
    }
}

fn read_usize(input: &str) -> usize { input.trim().parse().expect("Failed to parse and integer") }
fn read_u32(input: &str) -> u32 { input.trim().parse().expect("Failed to parse and integer") }