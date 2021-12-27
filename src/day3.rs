use aoc;
fn convert_to_structure(input: &Vec<String>) -> Vec<Vec<char>> { 
    let item0 = *input.iter().peekable().peek().unwrap();
    // println!("{}", item0.len());

    let mut r = Vec::new();

    for _ in 0..item0.len() {
        r.push(Vec::new());
    }

    for item in input {
        for i in 0..item0.len() {
            let c = item.chars().collect::<Vec<char>>();
            if let Some(v) = c.get(i) {
                r[i].push(*v);
                // println!("inserting into arran {} the value {}", i, v);
            }
        }
    }

    r
}

fn calculate_variants(v: Vec<Vec<char>>) -> (String, String) {
    let len = v.len();
    let len2 = v[0].len();

    let mut r1 = String::from("");
    let mut r2 = String::from("");

    for i in 0..len {
        let num_of_ones = v[i].iter().fold(0, |acc, &item|{
            if item == '1' {
                return acc + 1;
            }
            acc
        });
        if num_of_ones > len2/2 {
            r1.push('1');
            r2.push('0');
        }
        else {
            r1.push('0');
            r2.push('1');
        }
    }

    (r1, r2)
}

fn convert_str_to_decimal(s: &str) -> u32 {
    let mut i = 0;
    let res = s.chars().rev().fold(0, |acc, item|{
        let shift = 1u32 << i;
        i += 1;
        
        if item == '1' {
            return acc + shift
        }
        acc 
    });

    res
}

fn main()
{
    let input = aoc::read_one_per_line::<String>("./inputs/day3.txt").unwrap();
    // let input = vec![
    //     String::from("11010"),
    //     String::from("00110"),
    //     String::from("10101"),
    // ];

    let v = convert_to_structure(&input);
    // println!("{:?}", v);

    let (gamma, epsilon) = calculate_variants(v);

    let gammna_res = convert_str_to_decimal(&gamma[..]);
    let epsilon_res = convert_str_to_decimal(&epsilon[..]);

    println!("The result is {}", gammna_res*&epsilon_res);
    // assert_eq!(&gamma[..], gamma.as_str());




    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // let binnumber: u32 = 0b0011;
    // println!("{:04b}", binnumber * 6);

    // println!("{}", binnumber.to_string());
    // let s = "00010";

    // println!("{}", s.parse::<u32>().unwrap());

    // println!("{:?}", s.chars().collect::<Vec<char>>());

    // let x = s.chars().collect::<Vec<char>>();

    // println!("{}", x[3]);
}
