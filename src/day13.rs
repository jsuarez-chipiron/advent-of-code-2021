use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x1_fromstr = coords[0].parse::<u32>()?;
        let y1_fromstr = coords[1].parse::<u32>()?;

        Ok(Point { 
            x: x1_fromstr, 
            y: y1_fromstr, 
        })
    }
}

impl FromStr for Fold {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.starts_with("fold along") {
            let parts: Vec<&str> = s[11..].split('=').collect();
            let num = parts[1].parse::<u32>()?;
            if parts[0] == "x" {
                return Ok(Fold::X(num));
            }
            else {
                return Ok(Fold::Y(num));
            }
        }
        else {
            s.parse::<u32>()?;
            // La llamada a parse siempre va a fallar y se devolvera el parse int
            // No he visto otra forma mejor de crear un ParseIntError
            return Ok(Fold::Y(14));
        }
    }
}

fn parse_coord_input() -> Vec<Point> {
    aoc::read_one_per_line::<Point>("inputs/day13_test.txt").unwrap()
}

fn parse_fold_input() -> Vec<Fold> {
    aoc::read_one_per_line::<Fold>("inputs/day13_test.txt").unwrap()
}

fn get_num_rows(points: &Vec<Point>) -> u32 {
    points.iter().map(|i| i.y ).max().unwrap()
}

fn get_num_cols(points: &Vec<Point>) -> u32 {
    points.iter().map(|i| i.x ).max().unwrap()
}

fn main() {
    let points = parse_coord_input();
    println!("{:?}", points);

    let folds = parse_fold_input();
    println!("{:?}", folds);

    println!("max cols: {}", get_num_cols(&points));
    println!("max rows: {}", get_num_rows(&points));
}
