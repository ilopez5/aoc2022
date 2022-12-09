use std::collections::HashSet;

fn main() {
    let test = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let input = include_str!("inputs/input6.txt");

    let part1 = solution(input, 4);
    println!("Part 1: {}", part1);

    let part2 = solution(input, 14);
    println!("Part 2: {}", part2);
}

fn solution(line: &str, size: usize) -> usize {
    for (i, window) in line.as_bytes().windows(size).enumerate() {
        let num_unique = window.iter().copied().collect::<HashSet<_>>().len();
        if num_unique == size {
            return i + size;
        }
    }
    0
}
