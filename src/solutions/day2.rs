#[derive(Copy, Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn main() {
    let input = include_str!("inputs/input2.txt");
    let lines: Vec<&str> = input.lines().map(str::trim).collect();

    let part1 = part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = part2(&lines);
    println!("Part 2: {}", part2);
}

fn part1(lines: &[&str]) -> u32 {
    let mut score: u32 = 0;
    for line in lines {
        let round: Vec<&str> = line.split_whitespace().collect();
        let their_hand = get_elf_hand(round[0]);
        let my_hand = get_elf_hand(round[1]);
        score += calculate_score(&their_hand, &my_hand);
    }
    score
}

fn part2(lines: &[&str]) -> u32 {
    let mut score = 0;
    for line in lines {
        let round: Vec<&str> = line.split_whitespace().collect();
        let their_hand = get_elf_hand(round[0]);
        let match_outcome = get_match_outcome(round[1]);

        score += if match_outcome == Outcome::Draw {
            calculate_score(&their_hand, &their_hand)
        } else {
            let my_hand = get_human_hand(&their_hand, &match_outcome);
            calculate_score(&their_hand, &my_hand)
        };
    }
    score
}

fn calculate_score(them: &Hand, me: &Hand) -> u32 {
    get_hand_rank(me) + if *them == *me {
        3
    } else {
        match (*them, *me) {
            (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Scissors, Hand::Rock) => 6,
            _ => 0,
        }
    }
}

fn get_elf_hand(choice: &str) -> Hand {
    match choice {
        "A" => Hand::Rock,
        "X" => Hand::Rock,
        "B" => Hand::Paper,
        "Y" => Hand::Paper,
        "C" => Hand::Scissors,
        "Z" => Hand::Scissors,
        _ => panic!(),
    }
}

fn get_human_hand(them: &Hand, result: &Outcome) -> Hand {
    match them {
        Hand::Rock => match result {
            Outcome::Win => Hand::Paper,
            Outcome::Loss => Hand::Scissors,
            Outcome::Draw => Hand::Rock,
        },
        Hand::Paper => match result {
            Outcome::Win => Hand::Scissors,
            Outcome::Loss => Hand::Rock,
            Outcome::Draw => Hand::Paper,
        },
        Hand::Scissors => match result {
            Outcome::Win => Hand::Rock,
            Outcome::Loss => Hand::Paper,
            Outcome::Draw => Hand::Scissors,
        },
    }
}

fn get_match_outcome(choice: &str) -> Outcome {
    match choice {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!(),
    }
}

fn get_hand_rank(hand: &Hand) -> u32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}
