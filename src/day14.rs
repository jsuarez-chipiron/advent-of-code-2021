use std::collections::HashMap;

fn parse_traducctions(path: &str) -> HashMap<String, char>
{
    let mut dict = HashMap::new();

    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .filter(|&line| line.contains(" -> "))
        .for_each(|s| {
            let mut line_iter = s.split(" -> ");
            let key = line_iter.next().unwrap().to_string();
            let val = line_iter.next().unwrap().chars().next().unwrap();
            dict.insert(key, val);
        });

    dict
}

fn parse_template(path: &str) -> String
{
    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .next()
        .unwrap()
        .to_string()
}

fn get_particles(pol: &str) -> Vec<&str>
{
    let l = pol.len();
    let mut ret = vec![];

    for i in 0..l - 1 {
        ret.push(&pol[i..i + 2]);
    }

    ret
}

fn generate_polymer(pol: &str, dict: &HashMap<String, char>) -> String
{
    let mut ret = String::new();

    let particles = get_particles(pol);
    for p in particles {
        let new_particle = format!("{}{}", &p[..1], dict[p]);
        ret += &new_particle;
    }

    ret += &pol[pol.len() - 1..];

    ret
}

fn count_atoms(pol: &str) -> HashMap<char, u32>
{
    let mut ret = HashMap::new();

    pol.chars().for_each(|c| {
        *ret.entry(c).or_insert(0) += 1;
    });

    ret
}

fn get_result(map: &HashMap<char, u32>) -> u32
{
    let max = map.iter().map(|(_, &v)| v).max().unwrap();
    let min = map.iter().map(|(_, &v)| v).min().unwrap();

    max - min
}

fn main()
{
    let path = "inputs/day14.txt";

    let traducctions = parse_traducctions(path);

    let mut polymer = parse_template(path);

    for _ in 0..10 {
        polymer = generate_polymer(&polymer, &traducctions);
    }

    let counts = count_atoms(&polymer);

    println!("Part 1: {}", get_result(&counts));
}
