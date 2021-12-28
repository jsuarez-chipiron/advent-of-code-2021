use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct VentLine {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl FromStr for VentLine {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let coords: Vec<&str> = s.trim_matches(|p| p == ' ' )
        //                          .replace("->", ",")
        //                          .split(',')
        //                          .collect();
        let line = s.replace(" -> ", ",");
        let coords: Vec<&str> = line.split(',').collect();

        let x1_fromstr = coords[0].parse::<u32>()?;
        let y1_fromstr = coords[1].parse::<u32>()?;
        let x2_fromstr = coords[2].parse::<u32>()?;
        let y2_fromstr = coords[3].parse::<u32>()?;

        Ok(VentLine { 
            x1: x1_fromstr, 
            y1: y1_fromstr, 
            x2: x2_fromstr, 
            y2: y2_fromstr, 
        })
    }
}

fn main() {
    let x = aoc::read_one_per_line::<VentLine>("./inputs/day5.txt").unwrap();
    println!("{:?}", x);
}
