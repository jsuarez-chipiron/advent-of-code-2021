struct LowPoint {
    val: u32,
    _coords: (usize, usize),
}

fn is_lower(heatmap: &Vec<Vec<u32>>, coords: (usize, usize)) -> bool {
    let (i, j) = coords;
    let val = heatmap[i][j];

    if i > 0 && val >= heatmap[i-1][j] {
        return false;
    }
    if i < heatmap.len()-1 && val >= heatmap[i+1][j] {
        return false;
    }
    if j > 0 && val >= heatmap[i][j-1] {
        return false;
    }
    if j < heatmap[0].len()-1 && val >= heatmap[i][j+1] {
        return false;
    }

    true

}

fn main() {
    // let raw = aoc::read_one_per_line::<String>("inputs/day9_test.txt").unwrap();
    let raw = aoc::read_one_per_line::<String>("inputs/day9.txt").unwrap();
    let raw: Vec<&String> = raw.iter().take(raw.len()-1).collect();

    let mut heatmap: Vec<Vec<u32>> = Vec::new();

    raw.iter().for_each(|line| { 
        heatmap.push(line.chars().filter_map(|i| i.to_digit(10)).collect());
    });

    // println!("{:?}", heatmap);
    let mut low_points: Vec<LowPoint> = Vec::new();

    for i in 0..heatmap.len() {
        for j in 0..heatmap[0].len() {
            if is_lower(&heatmap, (i, j)) {
                low_points.push(LowPoint { val: heatmap[i][j], _coords: (i, j) });
            }
        }
    }

    let risk_level = low_points.iter().fold(0, |a, i| a + 1 + i.val );

    println!("Part 1: {}", risk_level);

}
