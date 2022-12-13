#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }

    pub fn move_right(&mut self, steps: i32) {
        self.y += steps;
    }
    pub fn move_left(&mut self, steps: i32) {
        self.y -= steps;
    }
    pub fn move_up(&mut self, steps: i32) {
        self.x -= steps;
    }
    pub fn move_down(&mut self, steps: i32) {
        self.x += steps;
    }

    pub fn lead(&mut self, direction: char, distance: i32) -> Vec<Self> {
        let mut moves = Vec::<Self>::new();
        match direction {
            'U' => {
                for _ in 0..distance {
                    self.move_up(1);
                    moves.push(self.clone());
                }
            }
            'D' => {
                for _ in 0..distance {
                    self.move_down(1);
                    moves.push(self.clone());
                }
            }
            'L' => {
                for _ in 0..distance {
                    self.move_left(1);
                    moves.push(self.clone());
                }
            }
            'R' => {
                for _ in 0..distance {
                    self.move_right(1);
                    moves.push(self.clone());
                }
            }
            _ => (),
        };
        moves
    }

    pub fn follow(&mut self, head: &Self) -> Vec<Self> {
        let mut moves = Vec::<Self>::new();
        while !self.hugs(&head) {
            //move vertically
            if self.x > head.x {
                self.move_up(1);
            } else if self.x < head.x {
                self.move_down(1);
            }
            //move horizontally
            if self.y > head.y {
                self.move_left(1);
            } else if self.y < head.y {
                self.move_right(1);
            }
            moves.push(self.clone());
        }
        moves
    }

    fn hugs(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

fn main() {
    let input = include_str!("inputs/input9.txt");
    let lines: Vec<&str> = input.lines().collect();

    let n = 600;
    let mut grid: Vec<Vec<u32>> = vec![vec![0; n]; n];
    grid[n / 2 - 1][n / 2 - 1] = 1;

    let part1 = solution(&mut grid.clone(), &lines, 2);
    println!("Part 1: {}", part1);

    let part2 = solution(&mut grid.clone(), &lines, 10);
    println!("Part 2: {}", part2);
}

fn _print_grid(grid: &[Vec<u32>]) {
    for row in grid {
        let r: String = row.iter().map(|c| if c > &0 { '#' } else { '.' }).collect();
        println!("{r}");
    }
}

fn _print_rope(grid: &[Vec<u32>], rope: &[Pos]) {
    for (r, row) in grid.iter().enumerate() {
        let mut show = Vec::<char>::new();
        for c in 0..row.len() {
            let mut ch: char = '.';
            for (i, knot) in rope.iter().enumerate() {
                if knot.x == r.try_into().unwrap() && knot.y == c.try_into().unwrap() {
                    if i == 0 {
                        ch = 'H';
                    } else {
                        ch = char::from_digit(i as u32, 10).unwrap();
                    }
                    break;
                }
            }
            show.push(ch);
        }
        println!("{}", show.iter().collect::<String>())
    }
}

fn solution(grid: &mut Vec<Vec<u32>>, lines: &[&str], rope_len: usize) -> u32 {
    let dim = (grid.len() as i32) / 2 - 1;
    let mut rope: Vec<Pos> = vec![Pos::new(dim, dim); rope_len];

    for line in lines {
        let instr: Vec<&str> = line.split_whitespace().collect();
        let direction: char = instr[0].chars().next().unwrap();
        let distance: i32 = instr[1].parse().unwrap();

        let mut moves: Vec<Pos> = vec![];
        let mut new_moves: Vec<Pos> = vec![];

        for i in 0..rope.len() {
            let knot = &mut rope[i];
            if i == 0 {
                new_moves.extend(knot.lead(direction, distance));
            } else {
                for m in moves {
                    new_moves.append(&mut knot.follow(&m));
                }
            }
            moves = new_moves.clone();
            new_moves.clear();
        }

        // update the grid with last knot's moves (tail)
        for m in &moves {
            let row = m.x as usize;
            let col = m.y as usize;
            grid[row][col] = 1;
        }
    }

    grid.iter().map(|l| l.iter().sum::<u32>()).sum()
}
