fn get_input(path: &str) -> Vec<Vec<String>> {

    let raw = aoc::read_one_per_line::<String>(path).unwrap();

    let lines: Vec<String> = raw.iter()
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();


    let mut ret: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let line: String = line.split("|").enumerate().map(|(i,v)| {
            (i, v.trim().to_string())
        })
        .filter(|(i,_)| *i==1)
            .map(|(_,v)| v)
            .collect();

        let signals: Vec<String> = line.split(" ").map(|s|s.to_string()).collect();
        ret.push(signals);
    }

    ret
}

fn get_count(codes: &Vec<Vec<String>>) -> usize {
    let flat_codes: Vec<&String> = codes.iter().flatten().filter(|s|{
        s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7
    })
    .map(|x|x).collect();
    
    flat_codes.len()
}

fn main() {
    let codes = get_input("inputs/day8.txt");
    println!("Part 1: {}", get_count(&codes));
}
