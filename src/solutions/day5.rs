#[derive(Clone, Debug)]
struct Crate<'a> {
    mark: &'a str,
}

fn main() {
    let input: &str = include_str!("inputs/input5.txt");
    let lines: Vec<&str> = input.lines().collect();

    let part1: String = solution(&lines, true);
    println!("Part 1: {}", part1);

    let part2: String = solution(&lines, false);
    println!("Part 2: {}", part2);
}

fn solution(lines: &Vec<&str>, part: bool) -> String {
    let mut layout = vec![vec![]; 10];

    for line in lines {
        if line.is_empty() {
            continue;
        } else if &line[..4] == "move" {
            //this line contains instructions
            let tokens: Vec<_> = line.split_whitespace().collect();

            let how_many: usize = tokens[1].parse().unwrap();
            let from: usize = tokens[3].parse().unwrap();
            let to: usize = tokens[5].parse().unwrap();
           
            if part {
                for _ in 0..how_many {
                    let transfer: Crate = layout[from].pop().unwrap();
                    layout[to].push(transfer);
                }
            } else {
                let idx = layout[from].len() - how_many;
                let slice = layout[from].split_off(idx);
                layout[to].extend(slice);
            }

            // println!("{layout:?}");

        } else if line.contains('[') {
            //this line contains crate(s)
            for (i, ch) in line.char_indices() {
                if ch == '[' {
                    let j = i / 4 + 1;
                    //want to store the crate into our map
                    let cr = Crate {
                        mark: &line[i+1..i+2],
                    };
                    layout[j].insert(0, cr);
                }
            }
        } 
    }

    let mut output: String = String::new();
    for mut stack in layout {
        if let Some(cr) = stack.pop() {
            output.push_str(cr.mark);
        }
    }
    output
}
