use std::collections::{HashMap, VecDeque};

fn validate_line(line: &String) -> Result<(), char>
{
    let mut stack: VecDeque<char> = VecDeque::new();

    for i in line.chars()
    {
        match i
        {
            ']' =>
            {
                let c = stack.pop_front().unwrap();
                if c != '['
                {
                    return Err(i);
                }
            }
            ')' =>
            {
                let c = stack.pop_front().unwrap();
                if c != '('
                {
                    return Err(i);
                }
            }
            '}' =>
            {
                let c = stack.pop_front().unwrap();
                if c != '{'
                {
                    return Err(i);
                }
            }
            '>' =>
            {
                let c = stack.pop_front().unwrap();
                if c != '<'
                {
                    return Err(i);
                }
            }
            _ =>
            {
                stack.push_front(i);
            }
        };
    }

    Ok(())
}

fn complete(line: &String) -> Vec<char>
{
    let mut stack: VecDeque<char> = VecDeque::new();
    for i in line.chars()
    {
        match i
        {
            ']' | ')' | '>' | '}' =>
            {
                let _ = stack.pop_front().unwrap();
            }
            '[' | '(' | '<' | '{' =>
            {
                stack.push_front(i);
            }
            _ =>
            {}
        };
    }

    let ret: Vec<char> = stack
        .iter()
        .map(|&i| match i
        {
            '[' => ']',
            '(' => ')',
            '{' => '}',
            '<' => '>',
            _ => i,
        })
        .collect();

    ret
}

fn main()
{
    let mut dict: HashMap<char, u64> = HashMap::new();

    dict.insert(')', 3);
    dict.insert(']', 57);
    dict.insert('}', 1197);
    dict.insert('>', 25137);

    let mut dict2: HashMap<char, u64> = HashMap::new();

    dict2.insert(')', 1);
    dict2.insert(']', 2);
    dict2.insert('}', 3);
    dict2.insert('>', 4);

    let raw = aoc::read_one_per_line::<String>("inputs/day10.txt").unwrap();
    let lines: Vec<&String> = raw.iter().take(raw.len() - 1).collect();

    let mut errors: Vec<char> = Vec::new();
    lines.iter().for_each(|i| match validate_line(*i)
    {
        Ok(()) =>
        {}
        Err(c) =>
        {
            errors.push(c);
        }
    });

    println!("Part1: {}", errors.iter().map(|i| dict[i]).sum::<u64>());

    let corrects: Vec<&String> = raw
        .iter()
        .filter(|&i| {
            if i.len() == 0
            {
                return false;
            }
            if let Ok(()) = validate_line(i)
            {
                return true;
            }
            false
        })
        .collect();

    let mut completes: Vec<Vec<char>> = Vec::new();

    corrects.iter().for_each(|i| {
        completes.push(complete(*i));
    });

    let mut results: Vec<u64> = Vec::new();
    for line in completes
    {
        let mut score = 0;
        for c in line
        {
            score *= 5;
            score += dict2[&c];
        }
        results.push(score);
    }

    results.sort();

    println!("Part 2: {}", results[results.len() / 2]);
}
