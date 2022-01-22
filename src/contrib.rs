use aoc;

fn main()
{
    let input = aoc::read_one_per_line::<String>(
        "/home/javier/Projects/homeserve/SpainSFDCClaims/main/force-app/main/default/proc2",
    )
    .unwrap();

    // let numbers: Vec<u32> = input
    //     .iter()
    //     .filter(|i| *i != "")
    //     .map(|x| String::from(x.trim()))
    //     .filter_map(|i| i.parse::<u32>().ok())
    //     .collect();

    // let total: u32 = numbers
    //     .iter()
    //     .sum();

    // let max: u32 = numbers
    //     .into_iter()
    //     .max()
    //     .unwrap();

    // let total = total as f32;
    // let max = max as f32;

    let mut total = 0;
    let mut max = 0;

    for i in input
        .into_iter()
        .filter(|i| i != "")
        .map(|x| String::from(x.trim()))
        .filter_map(|i| i.parse::<u32>().ok())
    {
        total += i;

        if i > max {
            max = i;
        }
    }

    let total = total as f32;
    let max = max as f32;

    println!("{:?}", max / total);
}
