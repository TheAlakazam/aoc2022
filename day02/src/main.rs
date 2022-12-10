type InputType = (u8, u8);

fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| (line.as_bytes()[0] - b'A', line.as_bytes()[2] - b'X'))
        .collect()
}

fn score(a: i32, b: i32) -> i32 {
    let s: i32 = a - b;
    match s {
        -1 | 2 => 1 + b + 6,
        0 => 1 + b + 3,
        _ => 1 + b
    }
}

fn score_2(a: i32, b: i32) -> i32 {
    match b {
        0 => (3 + (a - 1)) % 3 + 1,
        1 => (4 + a),
        2 => (7 + (a + 1) % 3),
        _ => 0
    }
}

fn main() {
    let data = include_str!("../input.txt").trim();
    let moves: Vec<InputType> = input_generator(data);
    let total: i32 = moves.iter()
        .map(|&(a, b)| score(a as i32, b as i32))
        .sum(); println!("Part 1: {}", total);
    let total_2: i32 = moves.iter()
        .map(|&(a, b)| score_2(a as i32, b as i32))
        .sum();
    println!("Part 2: {}", total_2);
}
