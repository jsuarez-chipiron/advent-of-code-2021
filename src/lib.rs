use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>, std::io::Error>
where 
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
       .split("\n")
       .filter_map(|line| line.parse::<T>().ok())
       .collect())
}

pub fn get_integers(path: &str) -> Vec<usize> {
    let integers = read_one_per_line::<String>(path).unwrap();

    let mut new_integers = Vec::new();
    for n in integers {
        if n != "" {
            new_integers.push(n);
        }
    }
    let integers = new_integers;

    let integers = integers.get(0).unwrap();

    let integers: Vec<usize> = integers
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    integers

}

