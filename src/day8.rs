use std::collections::HashMap;

fn get_codes(path: &str, io: usize) -> Vec<Vec<String>>
{
    let raw = aoc::read_one_per_line::<String>(path).unwrap();

    let lines: Vec<String> = raw
        .iter()
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();

    let mut ret: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let line: String = line
            .split("|")
            .enumerate()
            .map(|(i, v)| (i, v.trim().to_string()))
            .filter(|(i, _)| *i == io)
            .map(|(_, v)| v)
            .collect();

        let signals: Vec<String> = line
            .split(" ")
            .map(|s| string_sort(&s.to_string()))
            .collect();
        ret.push(signals);
    }

    ret
}

fn get_count(codes: &Vec<Vec<String>>) -> usize
{
    let flat_codes: Vec<&String> = codes
        .iter()
        .flatten()
        .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
        .map(|x| x)
        .collect();

    flat_codes.len()
}

fn string_sort(s: &String) -> String
{
    let mut l: Vec<char> = s.chars().collect();
    l.sort_unstable();
    l.into_iter().collect()
}

fn code_in_code(c1: &String, c2: &String) -> bool
{
    // unoptimal code. easy to make it more eficient by sorting the signals of a digit
    let mut ret = true;
    for c in c1.chars() {
        let mut sw = false;
        for b in c2.chars() {
            if b == c {
                sw = true;
                break;
            }
        }
        if !sw {
            ret = false;
            break;
        }
    }
    ret
}

fn decode_input(input: &Vec<String>) -> HashMap<u32, String>
{
    let mut ret = HashMap::new();

    for code in input {
        match code.len() {
            2 => {
                ret.insert(1, code.to_string());
            }
            4 => {
                ret.insert(4, code.to_string());
            }
            3 => {
                ret.insert(7, code.to_string());
            }
            7 => {
                ret.insert(8, code.to_string());
            }
            _ => {}
        };
    }

    for code in input {
        match code.len() {
            6 => {
                // 0,6,9
                if !code_in_code(&ret[&1], code) {
                    ret.insert(6, code.to_string());
                } else {
                    if code_in_code(&ret[&4], code) {
                        ret.insert(9, code.to_string());
                    } else {
                        ret.insert(0, code.to_string());
                    }
                }
            }
            _ => {}
        };
    }

    for code in input {
        match code.len() {
            5 => {
                // 2,3,5
                if code_in_code(&ret[&1], code) {
                    ret.insert(3, code.to_string());
                } else {
                    if code_in_code(code, &ret[&6]) {
                        ret.insert(5, code.to_string());
                    } else {
                        ret.insert(2, code.to_string());
                    }
                }
            }
            _ => {}
        };
    }

    ret
}

fn main()
{
    let path = "inputs/day8.txt";
    let outputs = get_codes(path, 1);
    println!("Part 1: {}", get_count(&outputs));

    let inputs = get_codes(path, 0);
    let mut counter = 0;
    let mut decoded_outputs: Vec<u32> = Vec::new();

    inputs.iter().for_each(|i| {
        let hm = decode_input(i);

        let mut mh: HashMap<String, u32> = HashMap::new();
        hm.iter().for_each(|(old_key, old_val)| {
            mh.insert(old_val.to_string(), *old_key);
        });

        let decoded_output = outputs[counter]
            .iter()
            .map(|i| mh[i])
            .rev()
            .enumerate()
            .map(|(i, v)| 10u32.pow(i as u32) * v)
            .sum();

        decoded_outputs.push(decoded_output);

        counter += 1;
    });

    println!("Part 2: {}", decoded_outputs.iter().sum::<u32>());
}
