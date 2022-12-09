use std::collections::HashSet;

fn main() {
    let input = include_str!("inputs/input3.txt");

    let lines: Vec<&str> = input.lines().map(str::trim).collect();

    let part1 = part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = part2(&lines);
    println!("Part 2: {}", part2);
}

fn part1(lines: &[&str]) -> u32 {
    let mut count: u32 = 0;
    for line in lines {
        let size = line.len();
        let compartment_size = size / 2;

        let mut left = HashSet::<char>::new();
        let mut right = HashSet::<char>::new();

        for (i, ch) in line.chars().enumerate() {
            if i < compartment_size {
                left.insert(ch);
            } else {
                right.insert(ch);
            }
        }

        for j in left.intersection(&right) {
            count += priority(*j);
        }
    }
    count
}

fn part2(lines: &[&str]) -> u32 {
    let mut count: u32 = 0;

    let mut elf1 = HashSet::<char>::new();
    let mut elf2 = HashSet::<char>::new();
    let mut elf3 = HashSet::<char>::new();

    let mut i: u8 = 0;
    for line in lines {
        if i == 0 {
            for ch in line.chars() {
                elf1.insert(ch);
            }
            i += 1;
        } else if i == 1 {
            for ch in line.chars() {
                elf2.insert(ch);
            }
            i += 1;
        } else {
            for ch in line.chars() {
                elf3.insert(ch);
            }

            let common: HashSet<char> = elf1
                .intersection(&elf2)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&elf3)
                .copied()
                .collect::<HashSet<char>>();

            for ch in common {
                count += priority(ch);
            }

            elf1.clear();
            elf2.clear();
            elf3.clear();
            i = 0;
        }
    }
    count
}

fn priority(c: char) -> u32 {
    let ascii = c as u32;
    if c.is_uppercase() {
        ascii - 38
    } else {
        ascii - 96
    }
}
