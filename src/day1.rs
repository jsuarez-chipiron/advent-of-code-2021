use aoc;

fn main()
{
    let input = aoc::read_one_per_line::<u32>("./inputs/day1.txt").unwrap();

    let mut counter = 0;

    // for i in 1..input.len() {
    //     // println!("{}", &input[i]);
    //     if &input[i] > &input[i - 1] {
    //         counter += 1;
    //     }
    // }
    // println!("Part 1: the result is {}", counter);

    // counter = 0;
    // input.windows(2).for_each(|i| { if i[1] > i[0] { counter += 1; } });
    // println!("Part 1: the result is {}", counter);

    let result = input.windows(2).filter(|i| { i[1] > i[0] }).count();
    println!("Part 1: the result is {}", result);

    let mut new_input = Vec::new();

    for i in 1..input.len() - 1 {
        let new_item = &input[i - 1] + &input[i] + &input[i + 1];
        new_input.push(new_item);
    }

    let mut counter = 0;

    for i in 1..new_input.len() {
        // println!("{}", &input[i]);
        if &new_input[i] > &new_input[i - 1] {
            counter += 1;
        }
    }
    println!("Part 2: the result is {}", counter);
}
