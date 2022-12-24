use itertools::Itertools;

fn parse_input̐(input: &str) -> (Vec<(usize, usize, usize)>, Vec<Vec<char>>) {
    let (boxes, rest) = input.split_once("\n\n").unwrap();
    let mut stacks = vec![vec![]; 9];
    for l in boxes.lines().rev().skip(1).map(str::as_bytes) {
        for i in 0..stacks.len() {
            let c = l[i * 4 + 1];
            if c.is_ascii_alphabetic() {
                stacks[i].push(c as char);
            }
        }
    }
    let instructions = rest
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();

    (instructions, stacks)
}

fn part1(instructions: &[(usize, usize, usize)], mut stacks: Vec<Vec<char>>) -> String {
    for &(times, from, to) in instructions {
        for _ in 0..times {
            let item = stacks[from-1].pop().unwrap();
            stacks[to-1].push(item);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).join("")
}

fn part2(instructions: &[(usize, usize, usize)], mut stacks: Vec<Vec<char>>) -> String {
    for &(times, from, to) in instructions {
        let len = stacks[to-1].len() + times;
        stacks[to-1].resize(len, 'x');
        for i in 0..times {
            let item = stacks[from-1].pop().unwrap();
            stacks[to-1][len-1-i] = item;
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).join("")
}


fn main() {
    let data = include_str!("../input.txt").trim();
    let (instructions, stacks) = parse_input̐(data);

    println!("Part 1: {}", part1(&instructions, stacks.clone()));


    println!("Part 2: {}", part2(&instructions, stacks));
}

