fn main() {
    let data = include_str!("./input.txt").trim();
    // Part 1
    let mut elves: Vec<usize> = data.split("\n\n")
        .map(|elf: &str| elf.split("\n")
            .map(|e: &str| e.parse::<usize>().unwrap())
            .sum()
        )
        .collect();
    let elf_max = elves.iter().max();
    println!(
        "Part 1: {:?}", elf_max
    );
    elves.sort();
    let max_3_elves: usize = elves.iter().rev().take(3).sum();
    println!(
        "Part 2: {:?}", max_3_elves
    );
}
