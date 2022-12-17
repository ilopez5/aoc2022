#[derive(Copy, Clone, Debug)]
enum Op {
    Add(usize),
    Mul(usize),
    Div(usize),
    Mod(usize),
    Square,
}

type Item = usize;
type Target = usize;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Op,
    divisor: usize,
    target1: usize,
    target2: usize,
    inspections: usize,
}

impl Monkey {
    fn inspect(&self, idx: usize) -> Item {
        let item = self.items[idx];
        match self.operation {
            Op::Add(addend) => item + addend,
            Op::Mul(factor) => item * factor,
            Op::Square => item * item,
            _ => panic!(),
        }
    }

    fn decide(&self, item: Item) -> Target {
        if item % self.divisor == 0 {
            self.target1
        } else {
            self.target2
        }
    }

    fn reduce(&self, item: Item, reducer: Op) -> Item {
        match reducer {
            Op::Mod(modulus) => item % modulus,
            Op::Div(dividend) => item / dividend,
            _ => panic!(),
        }
    }

    fn catch(&mut self, item: Item) {
        self.items.push(item);
    }
}

fn main() {
    let input = include_str!("inputs/input11.txt");
    let lines: Vec<&str> = input.lines().map(str::trim).collect();

    let mut monkeys: Vec<Monkey> = parse_input(&lines);

    let part1 = solution(&mut monkeys.clone(), true);
    println!("Part 1: {}", part1);

    let part2 = solution(&mut monkeys, false);
    println!("Part 2: {}", part2);
}

fn solution(monkeys: &mut [Monkey], part1: bool) -> usize {
    let rounds = if part1 { 20 } else { 10000 };

    let reducer = if part1 {
        Op::Div(3)
    } else {
        let lcm = monkeys.iter().map(|x| x.divisor).product();
        Op::Mod(lcm)
    };

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                let m1 = &monkeys[m];
                let result = m1.inspect(i);
                let worry = m1.reduce(result, reducer);
                let target = m1.decide(worry);

                let m2 = &mut monkeys[target];
                m2.catch(worry);
            }
            let m1 = &mut monkeys[m];
            m1.inspections += m1.items.len();
            m1.items.clear();
        }
    }
    let mut active: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    active.sort_by_key(|l| std::cmp::Reverse(*l));
    active.iter().take(2).product()
}

fn parse_input(lines: &[&str]) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    let mut items: Vec<Item> = vec![];
    let mut operation: Op = Op::Square;
    let mut divisor = 0;
    let mut target1 = 0;
    let mut target2 = 0;
    for line in lines {
        if line.is_empty() {
            let monke = Monkey {
                items,
                operation,
                divisor,
                target1,
                target2,
                inspections: 0,
            };
            monkeys.push(monke);
            items = vec![];
        } else if line.starts_with("Starting items:") {
            items = line
                .split_once(':')
                .unwrap()
                .1
                .split(',')
                .map(|x| x.trim().parse::<Item>().unwrap())
                .collect();
        } else if line.starts_with("Operation: new = ") {
            let mut tokens = line.split_once('=').unwrap().1.trim().split_whitespace();
            tokens.next();
            let op = tokens.next().unwrap();
            let operand = tokens.next().unwrap();
            operation = match (op, operand) {
                ("*", "old") => Op::Square,
                ("*", _) => Op::Mul(operand.parse().unwrap()),
                ("+", _) => Op::Add(operand.parse().unwrap()),
                _ => panic!(),
            }
        } else if line.starts_with("Test: divisible by") {
            divisor = line[19..].parse().unwrap();
        } else if line.starts_with("If true: throw to monkey") {
            target1 = line[25..].parse().unwrap();
        } else if line.starts_with("If false: throw to monkey") {
            target2 = line[26..].parse().unwrap();
        }
    }
    let monke = Monkey {
        items,
        operation,
        divisor,
        target1,
        target2,
        inspections: 0,
    };
    monkeys.push(monke);
    monkeys
}
