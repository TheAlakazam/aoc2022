type InputType = String;
fn get_input(input: &str) -> Vec<InputType> {
    input.lines().map(|line| line.to_owned()).collect()
}
fn priority(c: char) -> u8 {
    if ('a'..='z').contains(&c) {
        c as u8 - b'a' + 1
    } else {
        27 + c as u8 - b'A'
    }
}
fn main() {
    let data = include_str!("../input.txt").trim();
    let items = get_input(data);
    let part1: u16 = items.iter()
            .map(|line| {
            let l = line.len();
            (&line.as_str()[..l / 2], &line.as_str()[l/2..])
        })
            .map(|(first, second)| {
            let mut result = 0;
            for c in first.chars() {
                if second.contains(c) {
                    result = priority(c);
                    break;
                }
            }
            result as u16
        })
            .sum();
    println!("Part 1: {}", part1);
    let mut part2: u16 = 0;
    for i in (0..items.len()).step_by(3) {
        for c in items[i].chars() {
            if items[i+1].contains(c) && items[i+2].contains(c) {
                part2 += priority(c) as u16;
                break;
            }
        }
    }
    println!("Part 2: {}", part2);
}

