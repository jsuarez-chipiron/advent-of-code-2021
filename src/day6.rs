#[derive(Debug, Clone, Copy)]
struct Fish {
    counter: i64,
}

impl Fish {
    fn new(counter: i64) -> Self {
        Fish{counter}
    }
}

#[derive(Debug, Clone)]
struct FishGroup {
    fishes: Vec<Fish>,
}

impl FishGroup {
    fn new(fishes: Vec<Fish>) -> Self {
        FishGroup{fishes}
    }

    fn next_gen(&self) -> FishGroup {
        let fishes: Vec<Fish> = self.fishes.iter().map(|x| Fish{counter: x.counter-1} ).collect();
        let mut new_fishes = fishes.clone();

        fishes.iter().filter(|&x| x.counter < 0).for_each(|_| new_fishes.push(Fish{counter: 8}));

        let fishes: Vec<Fish> = new_fishes.iter().map(|&x|{
            if x.counter < 0 {
                return Fish{counter: 6};
            }
            x
        }).collect();

        FishGroup{fishes}
    }

    fn len(&self) -> usize {
        self.fishes.len()
    }
}

fn get_fishes() -> Vec<Fish> {
    let fishes = aoc::read_one_per_line::<String>("./inputs/day6.txt").unwrap();

    let mut new_fishes = Vec::new();
    for n in fishes {
        if n != "" {
            new_fishes.push(n);
        }
    }
    let fishes = new_fishes;

    let fishes = fishes.get(0).unwrap();

    let fishes: Vec<i64> = fishes
        .split(',')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    let fishes = fishes.iter().map(|&x| Fish::new(x)).collect();
     
    fishes

}

fn main() {
    let mut fishes_group = FishGroup::new(get_fishes());

    (0..80).for_each(|_|{
        fishes_group = fishes_group.next_gen();
    });

    println!("part 1, result: {}", fishes_group.len());

    let mut fishes_group = FishGroup::new(get_fishes());

    (0..256).for_each(|_|{
        fishes_group = fishes_group.next_gen();
    });

    println!("part 2, result: {}", fishes_group.len());
}
