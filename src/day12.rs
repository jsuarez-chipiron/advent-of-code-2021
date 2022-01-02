use std::collections::HashSet;

fn parse_input() -> Vec<HashSet<String>> {
    let raw = aoc::read_one_per_line::<String>("inputs/day12_test.txt").unwrap();

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

fn is_big_cave(cave: &String) -> bool {
    cave.to_uppercase() == *cave
}

// fn get_next_steps<'a> (cave: &String, caves: &'a Vec<HashSet<String>>) -> Vec<&'a HashSet<String>> {
//     let mut ret: Vec<&HashSet<String>> = Vec::new();

//     caves.iter().for_each(|i| {
//         if i.contains(cave) {
//             ret.push(i);
//         }
//     });

//     ret
// }

fn get_next_caves (cave: &String, caves: &Vec<HashSet<String>>) -> Vec<String> {
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

fn run_paths(current_cave: &String, caves: &Vec<HashSet<String>>, visited_caves: &mut HashSet<String>, founded_paths: &mut Vec<String>) {
    founded_paths.push(current_cave.clone());
    let next_caves = get_next_caves(current_cave, caves);
    for next_cave in next_caves {
        if next_cave == "end" {
            println!("TERMINE");
        }
        else {
            if !is_big_cave(&next_cave) && !visited_caves.contains(&next_cave) {
                visited_caves.insert(next_cave.clone());
                run_paths(&next_cave, caves, visited_caves, founded_paths);
            }
        }
    }

     println!("visited_caves: {:?}", visited_caves);
     println!("founded_paths: {:?}", founded_paths);
}

fn main() {
    let caves = parse_input();
    println!("{:?}", caves);

    let mut visited_caves: HashSet<String> = HashSet::new();
    let mut founded_paths: Vec<String> = Vec::new();

    let starting_position = String::from("start");

    let from_start = get_next_caves(&starting_position, &caves);
    println!("{:?}", from_start);

    run_paths(&starting_position, &caves, &mut visited_caves, &mut founded_paths);


}
