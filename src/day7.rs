use std::collections::HashMap;

fn main()
{
    let crabs = aoc::get_integers::<i32>("./inputs/day7.txt");

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut fuel = i32::MAX;

    for i in *min as usize..*max as usize {
        let atempt = cost(&crabs, i);
        if atempt < fuel {
            fuel = atempt;
        }
    }

    println!("part1 fuel: {}", fuel);

    let mut fuel = i32::MAX;
    for i in *min as usize..*max as usize {
        let atempt = cost2(&crabs, i);
        if atempt < fuel {
            fuel = atempt;
        }
    }

    println!("part2 fuel: {}", fuel);
}

fn fill_fuel_map(max: i32) -> HashMap<i32, i32>
{
    let mut ret: HashMap<i32, i32> = HashMap::new();
    ret.insert(0, 0);
    for i in 1..=max {
        ret.insert(i, i + ret[&(i - 1)]);
    }

    ret
}

fn cost(crabs: &Vec<i32>, pos: usize) -> i32
{
    let fuel: Vec<i32> = crabs
        .iter()
        .map(|i| (pos as i32 - *i).abs())
        .collect::<Vec<i32>>();
    fuel.iter().sum()
}

fn cost2(crabs: &Vec<i32>, pos: usize) -> i32
{
    let max = crabs.iter().max().unwrap();
    let fuel_map = fill_fuel_map(*max);

    let fuel: Vec<i32> = crabs
        .iter()
        .map(|i| fuel_map[&((pos as i32 - *i).abs())])
        .collect::<Vec<i32>>();
    fuel.iter().sum()
}
