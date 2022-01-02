use std::collections::HashSet;

fn parse_input() -> Vec<HashSet<String>> {
    let raw = aoc::read_one_per_line::<String>("inputs/day12_test.txt").unwrap();

    let mut ret: Vec<HashSet<String>> = Vec::new();

    raw.iter().for_each(|line| {
        if line != "" {
            let mut hm: HashSet<String> = HashSet::new();
            line.split(",").for_each(|i| {
               hm.insert(String::from(i));
            });
            ret.push(hm);
        }
    });

    ret
}

fn main() {
    let caves = parse_input();
    println!("{:?}", caves);
}
