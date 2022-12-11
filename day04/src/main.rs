#[derive(Debug)]
struct LoadRange {
    start: u8,
    end: u8
}

fn convert_to_loadrange(line: &str) -> (LoadRange, LoadRange) {
    let mut splits = line.splitn(4, &[',', '-'])
        .map(|s| s.parse::<u8>().unwrap());
    let e1: LoadRange = LoadRange { start: splits.next().unwrap(), end: splits.next().unwrap() };
    let e2: LoadRange = LoadRange { start: splits.next().unwrap(), end: splits.next().unwrap() };
    (e1, e2)
}

fn can_cover(e1: &LoadRange, e2: &LoadRange) -> bool {
    (e1.start <= e2.start && e1.end >= e2.end) || (e2.start <= e1.start && e2.end >= e1.end)
}

fn is_overlap(e1: &LoadRange, e2: &LoadRange) -> bool {
    (e1.start..=e1.end).contains(&e2.start) || (e2.start..=e2.end).contains(&e1.start) || (e1.start..=e1.end).contains(&e2.end)
}
fn main() {
    let data = include_str!("../input.txt").trim();
    let inputs: Vec<(LoadRange, LoadRange)> = data.lines()
        .map(convert_to_loadrange)
        .collect();
    let part1 = inputs.iter()
        .filter(|&(e1, e2)| can_cover(e1, e2))
        .count();
    println!("Part 1: {}", part1);
    let part2 = inputs.iter()
        .filter(|&(e1, e2)| is_overlap(e1, e2))
        .count();
    println!("Part 2: {}", part2);
}

