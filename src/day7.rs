fn main() {
    let crabs = aoc::get_integers::<i32>("./inputs/day7.txt");
    println!("{:?}", crabs);

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut fuel = i32::MAX;
    
    for i in *min as usize..*max as usize {
        println!("atempt: {}", i);
        let atempt = cost(&crabs, i);
        if atempt < fuel {
            fuel = atempt;
        }
    };
        
    println!("fuel: {}", fuel);
}

fn cost(crabs: &Vec<i32>, pos: usize) -> i32 {
    let fuel: Vec<i32> = crabs.iter().map(|i| (pos as i32 - *i).abs()).collect::<Vec<i32>>();
    fuel.iter().sum() 
}
