use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>, std::io::Error>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .split('\n')
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn get_integers<T>(path: &str) -> Vec<T>
where
    T: FromStr,
{
    let integers = read_one_per_line::<String>(path).unwrap();

    let mut new_integers = Vec::new();
    for n in integers
    {
        if !n.is_empty()
        {
            new_integers.push(n);
        }
    }
    let integers = new_integers;

    let integers = integers.get(0).unwrap();

    let integers: Vec<T> = integers
        .split(',')
        .filter_map(|n| n.parse::<T>().ok())
        .collect();

    integers
}
