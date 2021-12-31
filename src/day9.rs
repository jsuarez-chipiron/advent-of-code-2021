#[derive(Debug, Clone, Copy)]
struct Point {
    val: u32,
    i: usize,
    j: usize,
}

impl Point {
    fn new(val: u32, i: usize, j: usize) -> Self {
        Point{val, i, j}
    }

    fn from_heightmap(heightmap: &Vec<Vec<u32>>, i: usize, j: usize) -> Self {
        Point{val: heightmap[i][j], i, j}
    }
}

fn is_lower(heightmap: &Vec<Vec<u32>>, coords: (usize, usize)) -> bool {
    let (i, j) = coords;
    let val = heightmap[i][j];

    if i > 0 && val >= heightmap[i-1][j] {
        return false;
    }
    if i < heightmap.len()-1 && val >= heightmap[i+1][j] {
        return false;
    }
    if j > 0 && val >= heightmap[i][j-1] {
        return false;
    }
    if j < heightmap[0].len()-1 && val >= heightmap[i][j+1] {
        return false;
    }

    true

}

fn add_to_basin(point: &Point, basin: &mut Vec<Point>, heightmap: &Vec<Vec<u32>>, comefrom: u8) {
    println!("ALEXA({}): {:?}", comefrom, point);
    if point.val != 9 {
        basin.push(point.clone());

        if point.i > 0 && comefrom != 2 {
            println!("rama 1({}) {:?}", comefrom, point);
            let p = Point::from_heightmap(heightmap, point.i-1, point.j);
            add_to_basin(&p, basin, heightmap, 1);
        }
        if point.i < heightmap.len()-1 && comefrom != 1 {
            println!("rama 2({}) {:?}", comefrom, point);
            let p = Point::from_heightmap(heightmap, point.i+1, point.j);
            add_to_basin(&p, basin, heightmap, 2);
        }
        if point.j > 0 && comefrom != 4 {
            println!("rama 3({}) {:?}", comefrom, point);
            let p = Point::from_heightmap(heightmap, point.i, point.j-1);
            add_to_basin(&p, basin, heightmap, 3);
        }
        if point.j < heightmap[0].len()-1 && comefrom != 3 {
            println!("rama 4({}) {:?}", comefrom, point);
            let p = Point::from_heightmap(heightmap, point.i, point.j+1);
            add_to_basin(&p, basin, heightmap, 4);
        }
    }
    else {
        println!("END {:?}", point);
    }

}

fn main() {
    // let raw = aoc::read_one_per_line::<String>("inputs/day9_test.txt").unwrap();
    let raw = aoc::read_one_per_line::<String>("inputs/day9_test.txt").unwrap();
    let raw: Vec<&String> = raw.iter().take(raw.len()-1).collect();

    let mut heightmap: Vec<Vec<u32>> = Vec::new();

    raw.iter().for_each(|line| { 
        heightmap.push(line.chars().filter_map(|i| i.to_digit(10)).collect());
    });

    // println!("{:?}", heightmap);
    let mut low_points: Vec<Point> = Vec::new();

    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            if is_lower(&heightmap, (i, j)) {
                low_points.push(Point { val: heightmap[i][j], i, j });
            }
        }
    }

    let risk_level = low_points.iter().fold(0, |a, i| a + 1 + i.val );

    println!("Part 1: {}", risk_level);

    println!("{:?}", low_points[0]);

    let mut basin: Vec<Point> = Vec::new();
    add_to_basin(&low_points[0], &mut basin, &heightmap, 0);
    println!("{:?}", basin);

}
