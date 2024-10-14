use::std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("{}", input.trim().to_lowercase());
    }
}