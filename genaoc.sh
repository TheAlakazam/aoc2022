#!/usr/bin/env bash

day=${1##+(0)}
if ((day < 1 || day > 25)); then
  exit
fi

AOC_SESSION=53616c7465645f5f178ee18885c499f9b9654ad7cd77bd3f8e71ff2d5ddf7d9a986ea6afb497b0ca937dde12f73258d5b1cc1f09c37177cc4edc055417bdf37b

project=$(printf "day%02d" $1)
if [ -z "$AOC_SESSION" ]; then
  echo "$AOC_SESSION isn't set. Cannot continue."
  exit
fi

cargo new ${project}
cd ${project}
curl -s "https://adventofcode.com/2022/day/${day}/input" --cookie "session=${AOC_SESSION}" -o input.txt

echo -n 'fn main() {
  let data = include_str!("../input.txt").trim();
  println!(
    "Part 1: {}", ""
  );

  println!(
    "Part 2: {}", ""
  );
}' > src/main.rs
