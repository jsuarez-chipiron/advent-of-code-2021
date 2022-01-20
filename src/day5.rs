use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct VentVector
{
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point
{
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Board
{
    points: HashMap<Point, i32>,
}

impl FromStr for VentVector
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let line = s.replace(" -> ", ",");
        let coords: Vec<&str> = line.split(',').collect();

        let x1_fromstr = coords[0].parse::<i32>()?;
        let y1_fromstr = coords[1].parse::<i32>()?;
        let x2_fromstr = coords[2].parse::<i32>()?;
        let y2_fromstr = coords[3].parse::<i32>()?;

        Ok(VentVector {
            x1: x1_fromstr,
            y1: y1_fromstr,
            x2: x2_fromstr,
            y2: y2_fromstr,
        })
    }
}

impl VentVector
{
    fn get_points(&self) -> Vec<Point>
    {
        let mut ret: Vec<Point> = Vec::new();
        // println!("vector: {:?}", self);

        if self.x1 == self.x2 {
            // vertical line
            let (first, last) = sort(self.y1, self.y2);
            for i in first..=last {
                ret.push(Point { x: self.x1, y: i });
            }
        } else {
            if self.y1 == self.y2 {
                // horizontal line
                let (first, last) = sort(self.x1, self.x2);
                // println!("y: {}: {}", first, last);
                for i in first..=last {
                    ret.push(Point { x: i, y: self.y1 });
                }
            } else {
                // diagonal line
                let mut i = 0;
                while i <= (self.x2 - self.x1).abs() {
                    if self.x1 < self.x2 {
                        if self.y1 < self.y2 {
                            // println!("up - up");
                            // println!("xx: {}-{}-{}", i, self.x1+i, self.y1+i);
                            ret.push(Point {
                                x: self.x1 + i,
                                y: self.y1 + i,
                            });
                        } else {
                            // println!("up - down");
                            // println!("xx: {}-{}-{}", i, self.x1+i, self.y1-i);
                            ret.push(Point {
                                x: self.x1 + i,
                                y: self.y1 - i,
                            });
                        }
                    } else {
                        if self.y1 < self.y2 {
                            // println!("down - up");
                            // println!("xx: {}-{}-{}", i, self.x1-i, self.y1+i);
                            ret.push(Point {
                                x: self.x1 - i,
                                y: self.y1 + i,
                            });
                        } else {
                            // println!("down - down");
                            // println!("xx: {}-{}-{}", i, self.x1-i, self.y1-i);
                            ret.push(Point {
                                x: self.x1 - i,
                                y: self.y1 - i,
                            });
                        }
                    }
                    i += 1;
                }
            }
        }

        ret
    }
}

fn sort(x: i32, y: i32) -> (i32, i32)
{
    if x > y {
        return (y, x);
    }
    (x, y)
}

impl Board
{
    fn new() -> Self
    {
        Board {
            points: HashMap::new(),
        }
    }

    fn add_vent(&mut self, vv: &VentVector)
    {
        let vent_points = vv.get_points();

        for vent_point in vent_points {
            self.add_vent_point(vent_point);
        }
    }

    fn add_vent_point(&mut self, vp: Point)
    {
        *self.points.entry(vp).or_insert(0) += 1;
    }
}

fn main()
{
    let vents = aoc::read_one_per_line::<VentVector>("./inputs/day5.txt").unwrap();

    let vents_horizontal_and_vertical: Vec<&VentVector> = vents
        .iter()
        .filter(|v| v.x1 == v.x2 || v.y1 == v.y2)
        .map(|x| x)
        .collect();

    let mut board = Board::new();
    vents_horizontal_and_vertical
        .iter()
        .for_each(|vv| board.add_vent(vv));
    let result = board
        .points
        .iter()
        .filter(|&(_, x)| *x > 1)
        .fold(0, |acc, _| acc + 1);

    println!("part 1 result: {}", result);

    let mut board = Board::new();
    vents.iter().for_each(|vv| board.add_vent(vv));
    let result = board
        .points
        .iter()
        .filter(|&(_, x)| *x > 1)
        .fold(0, |acc, _| acc + 1);

    println!("part 2 result: {}", result);
}
