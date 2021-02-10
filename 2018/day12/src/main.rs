fn main() {
    let mut plants: Vec<char> = String::from("..#......##...#.#.###.#.##..##.#.....##....#.#.##.##.#..#.##........####.###.###.##..#....#...###.##.......................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................").chars().collect();
    for gen in 0..500 {
        println!("{:?}", plants);
        let mut new_generation: Vec<char> = vec!['.', '.'];
        for i in 2..plants.len() - 2 {
            let mut pat = vec![];
            for j in i - 2..i + 3 {
                pat.push(plants.get(j).unwrap());
            }
            new_generation.push(match_pattern(pat.into_iter().collect()));
        }
        new_generation.push('.');
        new_generation.push('.');
        let mut total = 0;
        for i in 2..plants.len() - 2 {
            if plants[i] == '#' {
                total += i - 2;
            }
        }
        eprintln!("{} {}", gen, total);
        plants = new_generation;
    }
    // done generations now add up the index's of all the pots
    let mut total = 0;
    for i in 2..plants.len() - 2 {
        if plants[i] == '#' {
            total += i - 2;
        }
    }
    println!("{}", total);
}

fn part1() {
    let mut plants: Vec<char> = String::from("..#......##...#.#.###.#.##..##.#.....##....#.#.##.##.#..#.##........####.###.###.##..#....#...###.##...............................................................................................................................").chars().collect();
    for _ in 0..20 {
        println!("{:?}", plants);
        let mut new_generation: Vec<char> = vec!['.', '.'];
        for i in 2..plants.len() - 2 {
            let mut pat = vec![];
            for j in i - 2..i + 3 {
                pat.push(plants.get(j).unwrap());
            }
            new_generation.push(match_pattern(pat.into_iter().collect()));
        }
        new_generation.push('.');
        new_generation.push('.');
        plants = new_generation;
    }
    // done generations now add up the index's of all the pots
    let mut total = 0;
    for i in 2..plants.len() - 2 {
        if plants[i] == '#' {
            total += i - 2;
        }
    }
    println!("{}", total);
}

fn match_pattern(string: String) -> char {
    match string.as_str() {
        ".#.##" => '.',
        ".####" => '.',
        "#..#." => '.',
        "##.##" => '#',
        "..##." => '#',
        "##..." => '#',
        "..#.." => '#',
        "#.##." => '.',
        "##.#." => '.',
        ".###." => '#',
        ".#.#." => '#',
        "#..##" => '#',
        ".##.#" => '#',
        "#.###" => '#',
        ".##.." => '#',
        "###.#" => '.',
        "#.#.#" => '#',
        "#...." => '.',
        "#...#" => '.',
        ".#..." => '#',
        "##..#" => '.',
        "....#" => '.',
        "....." => '.',
        ".#..#" => '#',
        "#####" => '.',
        "#.#.." => '.',
        "..#.#" => '#',
        "...##" => '.',
        "...#." => '#',
        "..###" => '.',
        "####." => '#',
        "###.." => '#',
        _ => '.',
    }
}
