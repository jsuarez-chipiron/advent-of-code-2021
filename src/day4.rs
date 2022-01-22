use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Number
{
    val: i32,
    checked: bool,
}

#[derive(Debug, Clone)]
struct Carton
{
    numbers: Vec<Number>,
    finished: bool,
    last_number: i32,
}

impl FromStr for Carton
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let numbers = s
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<i32>>();

        if numbers.len() > 1 {
            let vnum = numbers
                .iter()
                .fold(Vec::new(), |mut acc, item| {
                    acc.push(*item);
                    acc
                })
                .iter()
                .fold(Vec::new(), |mut acc, item| {
                    acc.push(Number {
                        val: *item,
                        checked: false,
                    });
                    acc
                });

            return Ok(Carton {
                numbers: vnum,
                finished: false,
                last_number: -1,
            });
        }

        Ok(Carton {
            numbers: Vec::new(),
            finished: false,
            last_number: -1,
        })
    }
}

impl Carton
{
    fn validate_lines(&self) -> bool
    {
        for i in (0..25).step_by(5) {
            let mut is_valid = true;
            for j in 0..5 {
                let k = j + i;
                is_valid = is_valid && self.numbers[k].checked;
            }
            if is_valid {
                return true;
            }
        }

        false
    }

    fn validate_cols(&self) -> bool
    {
        for i in 0..5 {
            let mut is_valid = true;
            for j in (0..25).step_by(5) {
                let k = j + i;
                is_valid = is_valid && self.numbers[k].checked;
            }
            if is_valid {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> i32
    {
        self.numbers
            .iter()
            .filter(|x| !x.checked)
            .fold(0, |acc, item| acc + item.val)
    }

    fn print_carton(&self)
    {
        println!(
            "Last Number: {} - finished {}",
            self.last_number, self.finished
        );
        for i in (0..25).step_by(5) {
            for j in 0..5 {
                let k = j + i;
                let mut c = "";
                if self.numbers[k].checked {
                    c = "*";
                }
                print!("{}{} ", c, self.numbers[k].val);
                // is_valid = is_valid && .checked;
            }
            println!("");
        }
    }
}

fn play(numbers: &Vec<i32>, cartones: &mut Vec<Carton>) -> Option<(i32, Carton)>
{
    for n in numbers {
        // println!("jugada: {}", n);
        let c = mark_number_and_validate(*n, cartones);
        if let Some(v) = c {
            return Some((*n, v));
        }
    }
    return None;
}

fn mark_number_and_validate(number: i32, cartones: &mut Vec<Carton>) -> Option<Carton>
{
    // let mut j = 0;
    for c in cartones.iter_mut() {
        // j += 1;

        for i in 0..25 {
            if c.numbers[i].val == number {
                // println!("changing to marked the number {} in carton {}", number, j);
                c.numbers[i].checked = true;
            }
        }

        let valid_line = c.validate_lines();
        let valid_cols = c.validate_cols();

        if valid_cols || valid_line {
            return Some(c.clone());
        }
    }

    None
}

fn play_to_lose(numbers: &Vec<i32>, cartones: &mut Vec<Carton>) -> Option<(i32, Carton)>
{
    for n in numbers {
        println!("jugada: {}", n);
        mark_number_and_validate_to_lose(*n, cartones);
        let unfinished = get_unfinished(&cartones);
        // println!("unfinished num: {}", unfinished.len());
        if unfinished.len() == 1 {
            return Some((*n, unfinished[0].clone()));
        }
    }
    return None;
}

fn get_unfinished(cartones: &Vec<Carton>) -> Vec<Carton>
{
    let ret = cartones
        .iter()
        .filter(|x| !x.finished)
        .fold(Vec::new(), |mut acc, item| {
            acc.push(item.clone());
            acc
        });
    ret
}

fn mark_number_and_validate_to_lose(number: i32, cartones: &mut Vec<Carton>)
{
    let mut j = 0;
    for c in cartones.iter_mut() {
        j += 1;

        if !c.finished {
            for i in 0..25 {
                if c.numbers[i].val == number {
                    println!("changing to marked the number {} in carton {}", number, j);
                    c.numbers[i].checked = true;
                }
            }

            let valid_line = c.validate_lines();
            let valid_cols = c.validate_cols();

            if valid_cols || valid_line {
                c.finished = true;
                c.last_number = number;
            }
        }
    }
}

fn get_cartones() -> Vec<Carton>
{
    let cartones = aoc::read_one_per_line::<Carton>("./inputs/day4_cartons.txt").unwrap();

    // println!("len: {}", cartones.len());

    let mut new_cartones = Vec::new();
    for c in cartones {
        if c.numbers.len() > 0 {
            new_cartones.push(c);
        }
    }

    new_cartones
}

fn main()
{
    let mut cartones = get_cartones();

    let numbers = aoc::read_one_per_line::<String>("./inputs/day4_numbers.txt").unwrap();
    // println!("{:?}", numbers);

    let mut new_numbers = Vec::new();
    for n in numbers {
        if n != "" {
            new_numbers.push(n);
        }
    }
    let numbers = new_numbers;
    // println!("{:?}", numbers);

    let numbers = numbers.get(0).unwrap();

    let numbers: Vec<i32> = numbers
        .split(',')
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    let res = play(&numbers, &mut cartones);

    match res {
        Some((num_play, winner)) => {
            let sum_unmarked = winner.sum_unmarked();
            println!(
                "Part 1: winner {}, sum unmarked: {}, final result: {}",
                num_play,
                sum_unmarked,
                sum_unmarked * num_play
            );
        }
        None => {
            println!("No winner");
        }
    }

    // part 2
    let mut boards = get_cartones();

    let res = play_to_lose(&numbers, &mut boards);

    match res {
        Some((num_play, loser)) => {
            loser.print_carton();
            let sum_unmarked = loser.sum_unmarked();
            println!(
                "Part 2: loser {}, sum unmarked: {}, final result: {}",
                num_play,
                sum_unmarked,
                sum_unmarked * num_play
            );
        }
        None => {
            println!("No winner");
        }
    }

    // for b in &boards {
    //     b.print_carton();
    //     println!("");
    // }
}
