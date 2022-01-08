use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Point
{
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Fold
{
    X(u32),
    Y(u32),
}

impl FromStr for Point
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let coords: Vec<&str> = s.split(',').collect();

        let x1_fromstr = coords[0].parse::<usize>()?;
        let y1_fromstr = coords[1].parse::<usize>()?;

        Ok(Point {
            x: x1_fromstr,
            y: y1_fromstr,
        })
    }
}

impl FromStr for Fold
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s.starts_with("fold along")
        {
            let parts: Vec<&str> = s[11..].split('=').collect();
            let num = parts[1].parse::<u32>()?;
            if parts[0] == "x"
            {
                return Ok(Fold::X(num));
            }
            else
            {
                return Ok(Fold::Y(num));
            }
        }
        else
        {
            s.parse::<u32>()?;
            // La llamada a parse siempre va a fallar y se devolvera el parse int
            // No he visto otra forma mejor de crear un ParseIntError
            return Ok(Fold::Y(14));
        }
    }
}

fn parse_coord_input(path: &str) -> Vec<Point>
{
    aoc::read_one_per_line::<Point>(path).unwrap()
}

fn parse_fold_input(path: &str) -> Vec<Fold>
{
    aoc::read_one_per_line::<Fold>(path).unwrap()
}

fn init_fixed(rows: usize, cols: usize) -> Vec<Vec<u32>>
{
    let mut ret: Vec<Vec<u32>> = Vec::new();

    for _ in 0..rows
    {
        ret.push(vec![0; cols]);
    }

    ret
}

fn convert_to_fixed(points: Vec<Point>) -> Vec<Vec<u32>>
{
    let rows = get_num_rows(&points) as usize;
    let cols = get_num_cols(&points) as usize;

    let mut ret = init_fixed(rows, cols);

    points.iter().for_each(|i| {
        ret[i.y][i.x] = 1;
    });

    ret
}

fn print_fixed<T>(points: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    for i in 0..points.len()
    {
        for j in 0..points[0].len()
        {
            print!("{} ", points[i][j]);
        }
        println!();
    }
}

fn get_num_rows(points: &Vec<Point>) -> usize
{
    points.iter().map(|i| i.y).max().unwrap() + 1
}

fn get_num_cols(points: &Vec<Point>) -> usize
{
    points.iter().map(|i| i.x).max().unwrap() + 1
}

fn unenumerate(v: Vec<(usize, &Vec<u32>)>) -> Vec<Vec<u32>>
{
    v.iter().map(|(_, x)| x.clone().clone()).collect()
}

fn reverse_horizontally(v: Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    v.iter().rev().map(|i| i.clone()).collect()
}

fn process_fold(p: &u32, points: Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    let (up, down): (Vec<(usize, &Vec<u32>)>, Vec<(usize, &Vec<u32>)>) = points
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != *p as usize)
        .partition(|(i, _)| *i < *p as usize);

    let up = unenumerate(up);
    let down = unenumerate(down);
    let down = reverse_horizontally(down);

    return sum_chunks(up, down);
}

fn sum_chunks(part1: Vec<Vec<u32>>, part2: Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    let mut ret: Vec<Vec<u32>> = init_fixed(part2.len(), part2[0].len());
    for i in 0..part2.len()
    {
        for j in 0..part2[0].len()
        {
            ret[i][j] = part1[i][j] + part2[i][j];
        }
    }

    ret
}

fn transpose(v: Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    let mut ret: Vec<Vec<u32>> = init_fixed(v[0].len(), v.len());

    for i in 0..ret.len()
    {
        for j in 0..ret[0].len()
        {
            ret[i][j] = v[j][i];
        }
    }

    ret
}

fn fold_fixed(points: Vec<Vec<u32>>, fold: &Fold) -> Vec<Vec<u32>>
{
    match fold
    {
        Fold::Y(p) => process_fold(p, points),
        Fold::X(p) =>
        {
            let temp_points = process_fold(p, transpose(points));
            transpose(temp_points)
        }
    }
}

fn count_points(points: Vec<Vec<u32>>) -> usize
{
    let mut ret = 0;
    points.iter().for_each(|i| {
        ret += i.iter().filter(|&x| *x > 0).count();
    });

    ret
}

fn format(points: &Vec<Vec<u32>>) -> Vec<Vec<char>>
{
    let mut ret = Vec::new();
    points.iter().for_each(|i| {
        ret.push(
            i.iter()
                .map(|&x| match x
                {
                    0 => '.',
                    _ => '#',
                })
                .collect(),
        );
    });

    ret
}

fn main()
{
    let path = "inputs/day13.txt";

    let points = parse_coord_input(path);
    let folds = parse_fold_input(path);

    let points = convert_to_fixed(points);
    let first_fold = fold_fixed(points, &folds[0]);

    let count = count_points(first_fold);
    println!("Part 1: {}", count);

    let points = parse_coord_input(path);
    let folds = parse_fold_input(path);

    let mut points = convert_to_fixed(points);

    for i in folds
    {
        points = fold_fixed(points, &i);
    }

    println!("Part 2:");
    print_fixed::<char>(&format(&points));
    // the result is EAHKRECP
}
