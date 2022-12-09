use std::collections::HashMap;

fn main() {
    let input = include_str!("inputs/input1.txt");

    let meals: Vec<&str> = input.lines().map(str::trim).collect();

    let elves = parse(&meals);
    
    let top_three = get_top_k(&elves, 3);

    println!("Part 1: {}", top_three[0]);

    println!("Part 2: {}", top_three.iter().sum::<u32>());
}

fn parse(meals: &[&str]) -> HashMap::<u32, Vec<u32>> {
    let mut elves = HashMap::<u32, Vec<u32>>::new();
    let mut elf_index = 0;
    for meal in meals {
        if meal.is_empty() {
            elf_index += 1;
        } else {
            let calories = meal.parse().unwrap();
            let elf_meals = elves.entry(elf_index).or_insert(Vec::new());
            elf_meals.push(calories);
        }
    }
    elves
}

fn get_top_k(map: &HashMap<u32, Vec<u32>>, k: usize) -> Vec<u32> {
    let mut packs = Vec::<u32>::new();
    for (_, meals) in map {
        packs.push(meals.iter().sum());
    }
    packs.sort_by(|a, b| b.cmp(a));
    return packs[..k].to_vec();
} 
