use aoc;

fn convert_to_structure(input: &Vec<String>) -> Vec<Vec<char>>
{
    let item0 = *input.iter().peekable().peek().unwrap();

    let mut r = Vec::new();

    for _ in 0..item0.len()
    {
        r.push(Vec::new());
    }

    for item in input
    {
        for i in 0..item0.len()
        {
            let c = item.chars().collect::<Vec<char>>();
            if let Some(v) = c.get(i)
            {
                r[i].push(*v);
            }
        }
    }

    r
}

fn get_num_of(pos: usize, v: &Vec<Vec<char>>, c: char) -> usize
{
    let num_of_ones = v[pos].iter().fold(0, |acc, &item| {
        if item == c
        {
            return acc + 1;
        }
        acc
    });

    num_of_ones
}

fn get_num_of_ones(pos: usize, v: &Vec<Vec<char>>) -> usize
{
    get_num_of(pos, &v, '1')
}

fn get_num_of_zeroes(pos: usize, v: &Vec<Vec<char>>) -> usize
{
    get_num_of(pos, &v, '0')
}

fn calculate_variants(v: &Vec<Vec<char>>) -> (String, String)
{
    let len = v.len();
    let len2 = v[0].len();

    let mut r1 = String::from("");
    let mut r2 = String::from("");

    for i in 0..len
    {
        let num_of_ones = get_num_of_ones(i, &v);
        if num_of_ones > len2 / 2
        {
            r1.push('1');
            r2.push('0');
        }
        else
        {
            r1.push('0');
            r2.push('1');
        }
    }

    (r1, r2)
}

fn convert_str_to_decimal(s: &str) -> u32
{
    let mut i = 0;
    let res = s.chars().rev().fold(0, |acc, item| {
        let shift = 1u32 << i;
        i += 1;

        if item == '1'
        {
            return acc + shift;
        }
        acc
    });

    res
}

fn get_bit_criteria(pos: usize, v: &Vec<Vec<char>>, flag: bool) -> Option<char>
{
    if pos >= v.len()
    {
        return None;
    }

    let num_of_ones = get_num_of_ones(pos, &v);
    let num_of_zeroes = get_num_of_zeroes(pos, &v);

    // println!("1's: {} - 0's: {}", num_of_ones, num_of_zeroes);

    if flag
    {
        if num_of_ones >= num_of_zeroes
        {
            return Some('1');
        }
        else
        {
            return Some('0');
        }
    }

    if num_of_ones < num_of_zeroes
    {
        return Some('1');
    }
    else
    {
        return Some('0');
    }
}

fn get_filter_indexes(bc: char, vpos: &Vec<char>) -> Vec<usize>
{
    vpos.iter()
        .enumerate()
        .filter(|(_, val)| **val == bc)
        .map(|(idx, _)| idx)
        .collect()
}

fn calculate_oxigen_or_co2(i: usize, v: &Vec<Vec<char>>, flag: bool) -> Vec<Vec<char>>
{
    let bit_criteria = get_bit_criteria(i, &v, flag);
    if let Some(bc) = bit_criteria
    {
        // println!("gg: {} - {}", bc, flag);
        // println!("char: {}", bc);
        // println!("{:?}", get_filter_indexes(bc, &v[i]));
        let new_v = filter_with_indexes(&v, &get_filter_indexes(bc, &v[i]));
        // println!("SUBVECTOR ({} - {}): {:?}", i, new_v[0].len(), new_v);

        if new_v[0].len() == 1
        {
            return new_v;
        }
        else
        {
            return calculate_oxigen_or_co2(i + 1, &new_v, flag);
        }
    }

    Vec::new()
}

fn filter_with_indexes(v: &Vec<Vec<char>>, idx: &Vec<usize>) -> Vec<Vec<char>>
{
    let mut res: Vec<Vec<char>> = Vec::new();

    for ix in v
    {
        let filtered = ix
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                for ii in idx
                {
                    if *ii == *i
                    {
                        return true;
                    }
                }
                false
            })
            .map(|(_, &val)| val)
            .collect::<Vec<char>>();

        res.push(filtered);
    }

    res
}

fn main()
{
    let input = aoc::read_one_per_line::<String>("./inputs/day3.txt").unwrap();
    // let input = vec![
    //     String::from("11010"),
    //     String::from("00110"),
    //     String::from("00010"),
    //     String::from("10101"),
    // ];

    let v = convert_to_structure(&input);
    // println!("{:?}", v);

    let (gamma, epsilon) = calculate_variants(&v);

    let gammna_res = convert_str_to_decimal(&gamma[..]);
    let epsilon_res = convert_str_to_decimal(&epsilon[..]);
    println!("The result of the part1 is {}\n", gammna_res * epsilon_res);

    let o2 = calculate_oxigen_or_co2(0, &v, true);

    let o2 = o2.iter().flatten().collect::<Vec<&char>>();
    let so2 = String::new();
    let o2 = o2.iter().fold(so2, |mut acc, item| {
        acc.push(**item);
        acc
    });

    let o2 = convert_str_to_decimal(&o2[..]);

    // println!("{:?}", v);
    let co2 = calculate_oxigen_or_co2(0, &v, false);

    let co2 = co2.iter().flatten().collect::<Vec<&char>>();
    let sco2 = String::new();
    let co2 = co2.iter().fold(sco2, |mut acc, item| {
        acc.push(**item);
        acc
    });

    let co2 = convert_str_to_decimal(&co2[..]);

    println!("The result of the part2 is {}\n", o2 * co2);
}
