use std::collections::HashSet;

type Coords = (usize, usize);

fn parse_input(path: &str) -> Vec<Vec<u32>>
{
    let mut ret = vec![];
    let raw = aoc::read_one_per_line::<String>(path).unwrap();
    let raw: Vec<&String> = raw.iter().take(raw.len() - 1).collect();

    for line in raw {
        let row: Vec<u32> = line
            .chars()
            .filter_map(|v| match v.to_digit(10) {
                Some(val) => Some(val),
                None => None,
            })
            .collect();
        ret.push(row);
    }

    ret
}

fn new_from_current_path(current_path: Vec<Coords>, new_point: Coords) -> Vec<Coords>
{
    let mut ret = current_path.clone();
    ret.push(new_point);
    ret
}

fn new_visited(visited: HashSet<Coords>, new_point: Coords) -> HashSet<Coords>
{
    let mut ret = visited.clone();
    ret.insert(new_point);
    ret
}

// fn has_visited(position: Coords, current_path: &Vec<Coords>) -> bool
// {
//     for p in current_path {
//         if position == *p {
//             return true
//         }
//     }
//     false
// }

fn travel(
    board: &Vec<Vec<u32>>,
    new_position: Coords,
    current_path: Vec<Coords>,
    founded_paths: &mut Vec<Vec<Coords>>,
    visited: HashSet<Coords>,
    rows: usize,
    cols: usize,
)
{
    let current_path = new_from_current_path(current_path, new_position);
    let visited = new_visited(visited, new_position);
    let (x, y) = new_position;
    // println!("going to {}, {}", x, y);
    if x == cols - 1 && y == rows - 1 {
        founded_paths.push(current_path);
    } else {
        // if x > 0 {
        //     let new_position: Coords = (x-1, y);
        //     if !visited.contains(&new_position) {
        //         travel(board, new_position, current_path.clone(), founded_paths, visited.clone(), rows, cols);
        //     }
        // }
        if x < board[0].len() - 1 {
            let new_position: Coords = (x + 1, y);
            if !visited.contains(&new_position) {
                travel(
                    board,
                    new_position,
                    current_path.clone(),
                    founded_paths,
                    visited.clone(),
                    rows,
                    cols,
                );
            }
        }
        // if y > 0 {
        //     let new_position: Coords = (x, y-1);
        //     if !visited.contains(&new_position) {
        //         travel(board, new_position, current_path.clone(), founded_paths, visited.clone(), rows, cols);
        //     }
        // }
        if y < board.len() - 1 {
            let new_position: Coords = (x, y + 1);
            if !visited.contains(&new_position) {
                travel(
                    board,
                    new_position,
                    current_path.clone(),
                    founded_paths,
                    visited.clone(),
                    rows,
                    cols,
                );
            }
        }
    }
}

#[allow(dead_code)]
fn print_paths(paths: &Vec<Vec<Coords>>)
{
    for i in paths {
        println!("{:?}", i);
    }
}

fn convert_to_risk_paths(paths: &Vec<Vec<Coords>>, board: &Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    let mut ret = vec![];

    for i in paths {
        ret.push(i.iter().map(|(x, y)| board[*x][*y]).collect());
    }

    ret
}

fn get_paths_risk(risk_path: &Vec<Vec<u32>>) -> Vec<u32>
{
    let mut ret = vec![];
    for i in risk_path {
        ret.push(i.iter().sum());
    }

    ret
}

fn main()
{
    let board = parse_input("inputs/day15.txt");

    let rows = board.len();
    let cols = board[0].len();
    let mut founded_paths: Vec<Vec<Coords>> = vec![];
    let current_path: Vec<Coords> = vec![];
    let visited_map: HashSet<Coords> = HashSet::new();
    let startion_position: Coords = (0, 0);

    travel(
        &board,
        startion_position,
        current_path,
        &mut founded_paths,
        visited_map,
        rows,
        cols,
    );

    // print_paths(&founded_paths);

    let risk_paths = convert_to_risk_paths(&founded_paths, &board);

    println!(
        "Part 1: {}",
        get_paths_risk(&risk_paths).iter().min().unwrap() - 1
    );
}
