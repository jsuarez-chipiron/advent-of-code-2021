use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    val: u32,
    i: usize,
    j: usize,
}

impl Point {
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

fn add_to_basin(point: &Point, basin: &mut Vec<Point>, heightmap: &Vec<Vec<u32>>, visited: &mut HashSet<Point>) {
    if point.val != 9 {
        basin.push(point.clone());
        visited.insert(point.clone());

        if point.i > 0 {
            let p = Point::from_heightmap(heightmap, point.i-1, point.j);
            if !visited.contains(&p) {
                add_to_basin(&p, basin, heightmap, visited);
            }
        }
        if point.i < heightmap.len()-1 {
            let p = Point::from_heightmap(heightmap, point.i+1, point.j);
            if !visited.contains(&p) {
                add_to_basin(&p, basin, heightmap, visited);
            }
        }
        if point.j > 0 {
            let p = Point::from_heightmap(heightmap, point.i, point.j-1);
            if !visited.contains(&p) {
                add_to_basin(&p, basin, heightmap, visited);
            }
        }
        if point.j < heightmap[0].len()-1 {
            let p = Point::from_heightmap(heightmap, point.i, point.j+1);
            if !visited.contains(&p) {
                add_to_basin(&p, basin, heightmap, visited);
            }
        }
    }
}

fn get_three_biggest_basins_coeficient(heightmap: &Vec<Vec<u32>>, low_points: &Vec<Point>) -> usize {

    let mut basins: Vec<Vec<Point>> = Vec::new();

    low_points.iter().for_each(|i|{
        let mut basin: Vec<Point> = Vec::new();
        let mut visited: HashSet<Point> = HashSet::new();
        add_to_basin(i, &mut basin, &heightmap, &mut visited);
        basins.push(basin);
    });

    let mut basins: Vec<usize> = basins.iter().map(|i| i.len()).collect();
    basins.sort();

    basins.iter().rev().take(3).fold(1, |a,&i| a*i )

}

fn main() {
    let raw = aoc::read_one_per_line::<String>("inputs/day9.txt").unwrap();
    let raw: Vec<&String> = raw.iter().take(raw.len()-1).collect();

    let mut heightmap: Vec<Vec<u32>> = Vec::new();

    raw.iter().for_each(|line| { 
        heightmap.push(line.chars().filter_map(|i| i.to_digit(10)).collect());
    });

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

    println!("Part2: {}", get_three_biggest_basins_coeficient(&heightmap, &low_points));


}
