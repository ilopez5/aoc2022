use std::collections::HashSet;

fn main() {
    let input = include_str!("inputs/input4.txt");
    let lines: Vec<&str> = input.lines().map(str::trim).collect();

    let part1 = part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = part2(&lines);
    println!("Part 2: {}", part2);
}

fn part1(lines: &[&str]) -> u32 {
    let mut count = 0;
    for line in lines {
        let r: Vec<&str> = line.split(',').collect();
        let r1: HashSet<u8> = parse_line(r[0]);
        let r2: HashSet<u8> = parse_line(r[1]);
        if r1.is_superset(&r2) || r2.is_superset(&r1) {
            count += 1;
        }
    }
    count
}

fn part2(lines: &[&str]) -> u32 {
    let mut count: u32 = 0;
    for line in lines {
        let r: Vec<&str> = line.split(',').collect();
        let r1: HashSet<u8> = parse_line(r[0]);
        let r2: HashSet<u8> = parse_line(r[1]);
        let r3: HashSet<_> = r1.intersection(&r2).collect();
        if r3.len() > 0 {
            count += 1;
        }
    }
    count
}

fn parse_line(part: &str) -> HashSet<u8> {
    let range: Vec<&str> = part.split('-').collect();
    let r1: u8 = range[0].parse().unwrap();
    let r2: u8 = range[1].parse().unwrap();
    (r1..r2+1).collect()
}
