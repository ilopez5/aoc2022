#[derive(Clone)]
struct Crate {
    mark: &'static str,
}

impl Crate {
    pub fn new(mark: &'static str) -> Crate {
        Crate {
            mark: mark,
        }
    }
}

fn main() {
    let input = include_str!("inputs/input5.txt");
    let lines: Vec<&str> = input.lines().collect();

    let part1 = solution(&lines, true);
    println!("Part 1: {}", part1);

    let part2 = solution(&lines, false);
    println!("Part 2: {}", part2);
}

fn solution(lines: &[&'static str], part1: bool) -> String {
    let mut layout = vec![vec![]; lines[0].len()];

    for line in lines {
        if line.is_empty() {
            continue;
        } else if &line[..4] == "move" {
            let tokens: Vec<_> = line.split_whitespace().collect();
            let how_many: usize = tokens[1].parse().unwrap();
            let from: usize = tokens[3].parse().unwrap();
            let to: usize = tokens[5].parse().unwrap();
           
            if part1 {
                for _ in 0..how_many {
                    let transfer = layout[from].pop().unwrap();
                    layout[to].push(transfer);
                }
            } else {
                let idx = layout[from].len() - how_many;
                let slice = layout[from].split_off(idx);
                layout[to].extend(slice);
            }
        } else if line.contains('[') {
            for (i, ch) in line.char_indices() {
                if ch == '[' {
                    let mark_idx = i / 4 + 1;
                    layout[mark_idx].insert(0, Crate::new(&line[i+1..i+2]));
                }
            }
        } 
    }

    let mut output = String::new();
    for mut stack in layout {
        if let Some(cr) = stack.pop() {
            output.push_str(cr.mark);
        }
    }
    output
}
