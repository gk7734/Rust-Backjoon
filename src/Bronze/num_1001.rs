use::std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let value: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a Number"))
        .collect();

    println!("{}", value[0] - value[1] );
}