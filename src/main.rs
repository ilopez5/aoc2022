fn main() {
    let input = include_str!("inputs/input8.txt");

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars()
             .map(|c| c.to_digit(10).unwrap())
             .collect())
        .collect();

    let part1 = part1(&grid);
    println!("Part 1: {}", part1);
    
    let part2 = part2(&grid);
    println!("Part 2: {}", part2);
}

fn part1(grid: &Vec<Vec<u32>>) -> u32 {
    let dim = grid.len() - 1;
    let mut visible: u32 = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, tree) in row.iter().enumerate() {
            visible += if r == 0 || c == 0 || r == dim || c == dim {
                1
            } else if tree == &0 {
                0
            } else if tree > row[..c].iter().max().unwrap() {
                1
            } else if tree > row[c+1..].iter().max().unwrap() {
                1
            } else if tree > grid[..r].iter().map(|rw| &rw[c]).max().unwrap() {
                1
            } else if tree > grid[r+1..].iter().map(|rw| &rw[c]).max().unwrap() {
                1
            } else {
                0
            };
        }
    }
    visible
}


fn part2(grid: &Vec<Vec<u32>>) -> u32 {
    let dim = grid.len() - 1;
    let mut max_scenic_score: u32 = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, tree) in row.iter().enumerate() {
            if r == 0 || c == 0 || r == dim || c == dim {
                continue;
            }

            let left = {
                let range = row[..c].iter().rev();
                get_scenic_score(range, *tree)
            };

            let right = get_scenic_score(&row[c+1..], *tree);

            let above = {
                let range = grid[..r].iter().map(|_row| &_row[c]).rev();
                get_scenic_score(range, *tree)
            };

            let below = {
                let range = grid[r+1..].iter().map(|_row| &_row[c]);
                get_scenic_score(range, *tree)
            };
            
            let score = left * right * above * below;
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }
    max_scenic_score
}

fn get_scenic_score<'a>(los: impl IntoIterator<Item = &'a u32>, tree: u32) -> u32 {
    let mut score: u32 = 0;
    for neighbor in los {
        score += 1;
        if neighbor >= &tree {
            break;
        }
    }
    score
}
