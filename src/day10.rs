use std::collections::{VecDeque, HashMap};

fn validate_line(line: &String) -> Result<(), char> {
    let mut stack: VecDeque<char> = VecDeque::new();

    for i in line.chars() {
        match i {
            ']' => {
                let c = stack.pop_front().unwrap();
                if c != '[' {
                    return Err(i);
                }
            },
            ')' => {
                let c = stack.pop_front().unwrap();
                if c != '(' {
                    return Err(i);
                }
            },
            '}' => {
                let c = stack.pop_front().unwrap();
                if c != '{' {
                    return Err(i);
                }
            },
            '>' => {
                let c = stack.pop_front().unwrap();
                if c != '<' {
                    return Err(i);
                }
            },
            _ => {
                stack.push_front(i);
            },
        };
    }

    Ok(())
}

fn main() {
    let mut dict: HashMap<char, u32> = HashMap::new();

    dict.insert(')', 3);
    dict.insert(']', 57);
    dict.insert('}', 1197);
    dict.insert('>', 25137);

    let raw = aoc::read_one_per_line::<String>("inputs/day10.txt").unwrap();
    let raw: Vec<&String> = raw.iter().take(raw.len()-1).collect();

    let mut errors: Vec<char> = Vec::new();
    raw.iter().for_each(|i| {
        match validate_line(*i) {
            Ok(()) => {},
            Err(c) => {
                errors.push(c);
            },
        }
    });

    println!("Part1: {}", errors.iter().map(|i| dict[i]).sum::<u32>());
}
