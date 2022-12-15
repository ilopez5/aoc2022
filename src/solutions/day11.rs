#[derive(Copy, Clone, Debug)]
enum Op {
    Add(usize),
    Mul(usize),
    Div,
    Square,
}

type Item = Vec<Op>;

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
    fn fling(&self, item: &mut Item, part1: bool) -> usize {
        item.push(self.operation);
        if part1 {
            item.push(Op::Div);
        }

        let mut solution = 0;
        for op in item {
            solution = match op {
                Op::Add(addend) => solution + *addend,
                Op::Mul(factor) => solution * *factor,
                Op::Square => solution * solution,
                Op::Div => solution / 3,
            };
            if !part1 {
                solution %= self.divisor;
            }
        }

        if part1 {
            solution %= self.divisor;
        }

        if solution == 0 {
            self.target1
        } else {
            self.target2
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
    for j in 0..rounds {
        if j != 0 && j % 1000 == 0 {
            println!("Checkpoint: {j}/{rounds}");
        }

        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                let m1 = &monkeys[m];

                let mut item: Item = m1.items[i].clone();
                let target = m1.fling(&mut item, part1);

                let m2 = &mut monkeys[target];
                m2.catch(item);
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
                .map(|x| vec![Op::Add(x.trim().parse().unwrap())])
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
