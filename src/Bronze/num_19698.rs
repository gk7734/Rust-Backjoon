use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = values[0]; // 응모한 소의 수
    let w = values[1]; // 헛간의 너비
    let h = values[2]; // 헛간의 높이
    let l = values[3]; // 소 한 마리당 필요한 공간의 크기

    // 헛간에 들어갈 수 있는 소의 수 계산
    let max_in_width = w / l; // 헛간 너비에 들어갈 수 있는 소의 수
    let max_in_height = h / l; // 헛간 높이에 들어갈 수 있는 소의 수

    let max_cows = max_in_width * max_in_height;

    let result = std::cmp::min(max_cows, n);

    println!("{}", result);
}
