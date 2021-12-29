use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct VentVector {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x:u32,
    y:u32,
    num_intersecs: u32,
}

#[derive(Debug)]
struct Board {
    points: Vec<Point>,
}

impl FromStr for VentVector {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.replace(" -> ", ",");
        let coords: Vec<&str> = line.split(',').collect();

        let x1_fromstr = coords[0].parse::<u32>()?;
        let y1_fromstr = coords[1].parse::<u32>()?;
        let x2_fromstr = coords[2].parse::<u32>()?;
        let y2_fromstr = coords[3].parse::<u32>()?;

        Ok(VentVector { 
            x1: x1_fromstr, 
            y1: y1_fromstr, 
            x2: x2_fromstr, 
            y2: y2_fromstr, 
        })
    }
}

impl VentVector {
    fn get_points(&self) -> Vec<Point> {
        
        let mut ret: Vec<Point> = Vec::new();

        if self.x1 == self.x2 {
            // vertical line
            let (first, last) = sort(self.y1, self.y2);
            for i in first..=last {
                ret.push(Point { x: self.x1, y: i, num_intersecs: 1 });
            }
        } else {
            // horizontal line
            let (first, last) = sort(self.x1, self.x2);
            // println!("y: {}: {}", first, last);
            for i in first..=last {
                ret.push(Point { x: i, y: self.y1, num_intersecs: 1 });
            }
        }

        ret
    }
}

fn sort(x: u32, y:u32) -> (u32, u32) {
    if x > y {
        return (y, x);
    }
    (x, y)
}

impl Board {
    fn new() -> Self {
        Board{points: Vec::new()}
    }

    fn add_vent(&mut self, vv: &VentVector) {
        let vent_points = vv.get_points();

        for vent_point in vent_points {
            self.add_vent_point(vent_point);
        }
    }

    fn add_vent_point(&mut self, vp: Point) {
        for bp in self.points.iter_mut() {
            if bp.x == vp.x && bp.y == vp.y {
                bp.num_intersecs += 1;
                return;
            }
        }
        self.points.push(vp);
    }
}

fn main() {
    let vents = aoc::read_one_per_line::<VentVector>("./inputs/day5.txt").unwrap();

    let vents_horizontal_and_vertical: Vec<&VentVector> = vents.iter().filter(|v| v.x1 == v.x2 || v.y1 == v.y2).map(|x|x).collect();

    let mut board = Board::new();

    // let v = VentVector{x1: 0, y1: 0, x2: 0, y2: 2 };
    // // let r = v.get_points();
    // board.add_vent(&v);

    // // println!("{:?}", r);

    // let v = VentVector{x1: 0, y1: 1, x2: 3, y2: 1 };
    // // let r = v.get_points();
    // board.add_vent(&v);

    // // println!("{:?}", r);
    // println!("{:?}", board);

    vents_horizontal_and_vertical.iter().for_each(|vv| board.add_vent(vv));

    let result = board.points.iter().filter(|x| x.num_intersecs > 1).fold(0, |acc, _| acc + 1);
    println!("part 1 result: {}", result);
}
