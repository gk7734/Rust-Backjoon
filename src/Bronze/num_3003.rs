use std::io;

fn main() {
    const PIECES: [i32; 6] = [1, 1, 2, 2, 2, 8];
    let mut vec: Vec<i32> = Vec::new();

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse input into a vector of i32 values
    let value: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Loop through the PIECES array and calculate the difference
    for i in 0..PIECES.len() {
        vec.push(PIECES[i] - value[i]); // Subtract value[i] from PIECES[i]
    }

    for num in vec {
        print!("{} ", num); // Print numbers separated by spaces
    }
}
