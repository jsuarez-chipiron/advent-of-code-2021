use std::collections::{HashMap, HashSet};

fn parse_input() -> Vec<HashSet<String>>
{
    let raw = aoc::read_one_per_line::<String>("inputs/day12.txt").unwrap();

    let mut ret: Vec<HashSet<String>> = Vec::new();

    raw.iter().for_each(|line| {
        if line != "" {
            let mut hm: HashSet<String> = HashSet::new();
            line.split("-").for_each(|i| {
                hm.insert(String::from(i));
            });
            ret.push(hm);
        }
    });

    ret
}

fn is_small_cave(cave: &String) -> bool
{
    cave.to_lowercase() == *cave
}

fn num_of_twos(mapa: &HashMap<&str, u8>) -> u8
{
    let mut count = 0u8;
    mapa.iter().for_each(|(_, &v)| {
        if v == 2 {
            count += 1;
        }
    });
    count
}

fn can_continue(cave: &String, current_path: &Vec<String>) -> bool
{
    if cave == "start" {
        return false;
    }

    if !is_small_cave(cave) {
        return true;
    }

    let mut mapa: HashMap<&str, u8> = HashMap::new();

    current_path.iter().for_each(|i| {
        if is_small_cave(i) && i != "start" {
            *mapa.entry(i).or_insert(0) += 1;
        }
    });

    match mapa.get(&cave[..]) {
        Some(&v) => {
            if v == 0 {
                return true;
            } else {
                if num_of_twos(&mapa) == 0 {
                    return true;
                } else {
                    return false;
                }
            }
        }
        None => {
            return true;
        }
    }
}

fn get_next_caves(cave: &String, caves: &Vec<HashSet<String>>) -> Vec<String>
{
    let mut ret: Vec<String> = Vec::new();

    caves.iter().for_each(|i| {
        if i.contains(cave) {
            i.iter().for_each(|x| {
                if x != cave {
                    ret.push(x.to_string());
                }
            });
        }
    });

    ret
}

fn new_from_current_path(cave: &String, old_path: Vec<String>) -> Vec<String>
{
    let mut ret: Vec<String> = Vec::new();
    old_path.iter().for_each(|i| {
        ret.push(i.to_string());
    });
    ret.push(cave.to_string());
    ret
}

fn run_paths(
    current_cave: &String,
    caves: &Vec<HashSet<String>>,
    current_path: Vec<String>,
    founded_paths: &mut Vec<Vec<String>>,
    level: u8,
)
{
    let current_path = new_from_current_path(current_cave, current_path);
    if current_cave == "end" {
        founded_paths.push(current_path);
    } else {
        let next_caves = get_next_caves(current_cave, caves);
        for next_cave in next_caves {
            if can_continue(&next_cave, &current_path) && next_cave != "start" {
                run_paths(
                    &next_cave,
                    caves,
                    current_path.clone(),
                    founded_paths,
                    level + 1,
                );
            }
        }
    }
}

fn main()
{
    let caves = parse_input();

    let mut founded_paths: Vec<Vec<String>> = Vec::new();
    let starting_position = String::from("start");
    let current_path: Vec<String> = Vec::new();

    run_paths(
        &starting_position,
        &caves,
        current_path,
        &mut founded_paths,
        0,
    );

    println!("Part 2: {}", founded_paths.len());
}
