// struct Point {
//     i: u32,
//     j: u32,
// }

fn parse_input() -> Vec<Vec<u32>> {
    let raw = aoc::read_one_per_line::<String>("inputs/day11_test.txt").unwrap();

    let mut ret: Vec<Vec<u32>> = Vec::new();

    raw.iter().for_each(|line| {
        if line != "" {
            ret.push(line.chars().filter_map(|i| i.to_digit(10)).collect());
        }
    });

    ret
}

fn get_num_of_fashes(octopuses: &mut Vec<Vec<u32>>) -> u32 {
    increase_octopuses_energy(octopuses);

    let num_of_flashes = get_num_of_fashes_recur(octopuses);

    octopuses.iter_mut().for_each(|o| {
        o.iter_mut().for_each(|i| {
            if *i > 9 {
                *i = 0;
            }
        });
    });

    num_of_flashes
}

fn get_num_of_fashes_recur(octopuses: &mut Vec<Vec<u32>>) -> u32 {
    let num_of_flashes = process_flashes(octopuses);
    if num_of_flashes == 0 {
        return 0;
    }
    else {
        return num_of_flashes + get_num_of_fashes_recur(octopuses);
    }
}

fn process_flashes(octopuses: &mut Vec<Vec<u32>>) -> u32 {
    let num_rows = octopuses.len();
    let num_cols = octopuses[0].len();

    let mut num_of_flashes = 0;

    for i in 0..num_rows {
        for j in 0..num_cols {
            if octopuses[i][j] == 10 {
                num_of_flashes += 1;
                for i2 in (i as i32 - 1)..=(i as i32 + 1) {
                    if i2 >= 0 && i2 < num_rows as i32 {
                        for j2 in (j as i32 - 1)..=(j as i32 + 1) {
                            if j2 >= 0 && j2 < num_cols as i32 {
                                octopuses[i2 as usize][j2 as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    num_of_flashes
}

fn increase_octopuses_energy(octopuses: &mut Vec<Vec<u32>>) {
    octopuses.iter_mut().for_each(|o| {
        o.iter_mut().for_each(|i| {*i=*i+1;});
    })

}

fn main() {
    let mut octopuses = parse_input();
    println!("{:?}", octopuses);

    let mut  num_of_fashes = 0;
    (0..10).for_each(|_| {
        num_of_fashes += get_num_of_fashes(&mut octopuses);
    });
    // println!("{:?}", octopuses);
    println!("num_of_flashes: {}", num_of_fashes);
}
