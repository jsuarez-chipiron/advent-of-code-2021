use std::collections::VecDeque;

fn get_fishes() -> Vec<usize> {
    let fishes = aoc::read_one_per_line::<String>("./inputs/day6.txt").unwrap();

    let mut new_fishes = Vec::new();
    for n in fishes {
        if n != "" {
            new_fishes.push(n);
        }
    }
    let fishes = new_fishes;

    let fishes = fishes.get(0).unwrap();

    let fishes: Vec<usize> = fishes
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    fishes

}

fn get_count(puzzle: &Vec<usize>, days: u32) -> u64 {
    let mut counts = VecDeque::from(vec![0; 9]);

    puzzle.iter().for_each(|i| counts[*i] += 1);

    (0..days).for_each(|_| {
        let new_babies = counts.pop_front().unwrap();
        counts[6] += new_babies;
        counts.push_back(new_babies);
    });

    counts.iter().sum()
}

fn main() {
    //TJ's solution
    
    let initial_fishes = get_fishes();

    println!("Part1: {}", get_count(&initial_fishes, 80));
    println!("Part2: {}", get_count(&initial_fishes, 256));
}
