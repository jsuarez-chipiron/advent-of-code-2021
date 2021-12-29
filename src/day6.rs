use std::collections::VecDeque;

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
    
    let initial_fishes = aoc::get_integers("inputs/day6.txt");

    println!("Part1: {}", get_count(&initial_fishes, 80));
    println!("Part2: {}", get_count(&initial_fishes, 256));
}
