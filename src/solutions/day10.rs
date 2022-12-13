#[derive(Debug)]
struct Instr {
    cycles: u32,
    amount: i32,
}

impl Instr {
    fn new(cycles: u32, amt: i32) -> Self {
        Self {
            cycles: cycles,
            amount: amt,
        }
    }

    fn tick(&mut self) {
        self.cycles -= 1;
    }

    fn is_done(&self) -> bool {
        self.cycles < 1
    }

    fn get_amount(instr: &Self) -> i32 {
        if instr.is_done() {
            instr.amount
        } else {
            0
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Op {
    NOOP,
    ADDX,
}

fn main() {
    let input = include_str!("inputs/input10.txt");
    let lines: Vec<&str> = input.lines().collect();

    let part1 = part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = part2(&lines);
    println!("Part 2:");
    for line in part2 {
        println!("{}", line.into_iter().collect::<String>());
    }
}

fn part1(lines: &[&str]) -> i32 {
    let mut strength: i32 = 0;
    let mut x: i32 = 1;
    let mut operations: Vec<Instr> = vec![];
    let mut pc: u32 = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let instr = get_instruction(parts[0]);
        let cycles = get_cycles(instr);

        match instr {
            Op::NOOP => (),
            Op::ADDX => {
                let amt: i32 = parts[1].parse().unwrap();
                operations.push(Instr::new(cycles + 1, amt));
            }
        };

        for _ in 0..cycles {
            pc += 1;

            for curr in operations.iter_mut() {
                curr.tick();
            }

            x += operations.iter().map(Instr::get_amount).sum::<i32>();
            operations.retain(|op| !op.is_done()); //todo: static method here?

            if pc % 40 == 20 {
                strength += x * (pc as i32);
            }
        }
    }
    strength
}

fn part2<'a>(lines: &'a [&'static str]) -> Vec<Vec<char>> {
    let mut x: i32 = 1;
    let mut operations: Vec<Instr> = vec![];
    let mut display: Vec<Vec<char>> = vec![];
    let mut pixel_row: Vec<char> = vec![];
    let mut pixel_idx = 0;
    let screen_width: usize = 40;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let instr = get_instruction(parts[0]);
        let cycles = get_cycles(instr);

        match instr {
            Op::NOOP => (),
            Op::ADDX => {
                let amt: i32 = parts[1].parse().unwrap();
                operations.push(Instr::new(cycles + 1, amt));
            }
        };

        for _ in 0..cycles {
            for curr in operations.iter_mut() {
                curr.tick();
            }

            x += operations.iter().map(Instr::get_amount).sum::<i32>();
            operations.retain(|op| !op.is_done()); //todo: static method here?

            let sprite = x - 1..x + 2;
            let pixel = if sprite.contains(&pixel_idx) {
                '#'
            } else {
                '.'
            };

            if pixel_row.len() == screen_width {
                display.push(pixel_row.clone());
                pixel_row.clear();
            }
            pixel_row.push(pixel);
            pixel_idx = (1 + pixel_idx) % (screen_width as i32);
        }
    }
    display.push(pixel_row.clone());
    display
}

fn get_instruction(token: &str) -> Op {
    match token {
        "noop" => Op::NOOP,
        "addx" => Op::ADDX,
        _ => panic!(),
    }
}

fn get_cycles(instr: Op) -> u32 {
    match instr {
        Op::NOOP => 1,
        Op::ADDX => 2,
    }
}
