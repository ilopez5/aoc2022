pub struct Dir {
    pub name: &'static str,
    pub files: Vec<File>,
    pub dirs: Vec<Dir>,
}

impl Dir {
    pub fn new(name: &'static str) -> Self {
        Self {
            name: name,
            fils: Vec::new(),
            dirs: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        let size: usize = self.files.iter().map(|f| f.size).sum();
        size + self.dirs.iter().map(|d| d.size()).sum::<usize>()
    }
}

pub struct File {
    pub name: &'static str,
    pub size: usize,
}

impl File {
    pub fn new(name: &'static str, size: usize) -> Self {
        Self {
            name: name,
            size: size,
        }
    }
}

fn main() {
    let input = include_str!("inputs/input7.txt");

    let mut root: Dir = Dir::new("/");
    let lines: Vec<&str> = input.lines().collect();
    
    build_tree(&mut root, &lines[1..]);

    const TOTAL: usize = 70000000;
    const NEEDED: usize = 30000000;
    let remaining = NEEDED - (TOTAL - root.size());

    let part1 = part1(&root, 100000);
    println!("Part 1: {}", part1);
    
    let part2 = part2(&root, remaining);
    println!("Part 2: {}", part2);
}

fn part1(dir: &Dir, k: usize) -> usize {
    let size: usize = dir.size();
    dir.dirs.iter().map(|d| part1(&d, k)).sum::<usize>() + if size <= k {
        size
    } else {
        0
    }
}


fn part2(dir: &Dir, needed: usize) -> usize {
    let size = dir.size();
    if size >= needed {
        let min_subdir_size = dir.dirs
            .iter()
            .map(|d| part2(&d, needed))
            .reduce(|acc, s| std::cmp::min(acc, s)) 
            .unwrap_or(std::usize::MAX);
        return std::cmp::min(size, min_subdir_size);
    }
    std::usize::MAX
}

fn build_tree(root: &mut Dir, lines: &[&'static str]) {
    let mut history: Vec<&str> = vec![];
    for (i, line) in lines.iter().enumerate() {
        let cwd = traverse_mut(&history, root);

        if *line == "$ ls" {
            parse_dir(cwd, &lines[i+1..]);
        } else if *line == "$ cd .." {
            history.pop();
        } else if &line[0..4] == "$ cd" {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            history.push(tokens[2]);
        }
    }
}

fn traverse_mut<'t>(history: &[&str], root: &'t mut Dir) -> &'t mut Dir {
    let mut cwd = root;
    for dir_name in history {
        cwd = find(cwd, dir_name);
    }
    cwd
}

fn find<'f>(cwd: &'f mut Dir, name: &str) -> &'f mut Dir {
    for dir in cwd.dirs.iter_mut() {
        if name == dir.name {
            return dir;
        }
    }
    panic!();
}

fn parse_dir(parent: &mut Dir, lines: &[&'static str]) {
    for line in lines {
        let tokens: Vec::<&str> = line.split_whitespace().collect();
        let first = tokens[0];

        if first == "dir" {
            parent.dirs.push(Dir::new(tokens[1]));
        } else if first != "$" {
            let size = first.parse().unwrap();
            parent.files.push(File::new(tokens[1], size));
        } else {
            return;
        }
    }
}
