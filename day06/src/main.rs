use itertools::Itertools;

fn find_unique(input: &str, size: usize) -> usize {
    size + input.as_bytes()
        .windows(size)
        .position(|window| window.iter().tuple_combinations().all(|(a, b)| a != b))
        .unwrap()
}
fn main() {
    let data = include_str!("../input.txt").trim();
    println!("Part 1: {}", find_unique(data, 4));

    println!("Part 2: {}", find_unique(data, 14));
}

