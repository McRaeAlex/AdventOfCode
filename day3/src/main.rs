use std::fs::File;
use std::io::prelude::*;
use std::usize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
struct Tile {
    red: bool,
    blue: bool,
}

#[derive(Debug, Clone, Copy)]
struct Tile2 {
    red: usize,
    blue: usize,
}

fn main() {
    part2();
}

fn part2() {
    const VAL: usize = 40000;
    // We will set middle to 1000, 1000

    // Create the board
    //let mut intersections: BTreeSet<(isize, isize, usize)> = BTreeSet::new();
    let mut red_path: Vec<(usize, usize, usize)> = Vec::new();
    let mut blue_path: Vec<(usize, usize, usize)> = Vec::new();

    // Read in file
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file to string");

    let mut lines = content.lines();
    let red = lines.next().expect("failed to get red line");
    let blue = lines.next().expect("failed to get blue line");

    println!("Going through red");

    let mut red_moves = 0;
    let mut red_pos = (VAL / 2, VAL / 2);

    for command in red.split(",") {
        match command.chars().nth(0).expect("Failure") {
            'R' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_moves += 1;
                    red_pos.0 += 1;
                    red_path.push((red_pos.0, red_pos.1, red_moves));
                }
            },
            'L' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse L ");
                for _ in 0..val {
                    red_moves += 1;
                    red_pos.0 -= 1;
                    red_path.push((red_pos.0, red_pos.1, red_moves));
                }
            },
            'D' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse D");
                for _ in 0..val {
                    red_moves += 1;
                    red_pos.1 -= 1;
                    red_path.push((red_pos.0, red_pos.1, red_moves));
                }
            },
            'U' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse U");
                for _ in 0..val {
                    red_moves += 1;
                    red_pos.1 += 1;
                    red_path.push((red_pos.0, red_pos.1, red_moves));
                }
            }
            _ => {},
        }
    }

    println!("Going through blue now!");

    let mut blue_moves = 0;
    let mut blue_pos = (VAL / 2, VAL / 2);

    for command in blue.split(",") {
        match command.chars().nth(0).expect("Failure") {
            'R' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse");
                for _ in 0..val {
                    blue_moves += 1;
                    blue_pos.0 += 1;
                    blue_path.push((blue_pos.0, blue_pos.1, blue_moves));
                }
            },
            'L' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse");
                for _ in 0..val {
                    blue_moves += 1;
                    blue_pos.0 -= 1;
                    blue_path.push((blue_pos.0, blue_pos.1, blue_moves));
                }
            },
            'D' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse");
                for _ in 0..val {
                    blue_moves += 1;
                    blue_pos.1 -= 1;
                    blue_path.push((blue_pos.0, blue_pos.1, blue_moves));
                }
            },
            'U' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse");
                for _ in 0..val {
                    blue_moves += 1;
                    blue_pos.1 += 1;
                    blue_path.push((blue_pos.0, blue_pos.1, blue_moves));
                }
            }
            _ => {},
        }
    }

    let mut steps = usize::max_value();

    //println!("{:?}", red_path);

    for (x1, y1, rm) in &red_path {
        //println!("{}, {}, {}", x1, y1, rm);
        for (x2, y2, bm) in &blue_path {
            //println!("Hello");
            //println!("    {}, {}, {}", x2, y2, bm);
            if *x1 == *x2 && *y1 == *y2 {
                
                let temp = rm + bm;
                if temp < steps {
                    steps = temp;
                    println!("x: {} y: {} steps: {}", x1, y1, steps);
                }
            }
        }
    }

    println!("Answer: {}", steps);
}

fn part1() {
    const VAL: usize = 40000;
    // We will set middle to 1000, 1000

    // Create the board
    let mut board: Vec<Vec<Tile>> = vec![
        vec![
            Tile {
                red: false,
                blue: false,
            };
            VAL
        ];
        VAL
    ];

    // Read in file
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file to string");

    let mut lines = content.lines();
    let red = lines.next().expect("failed to get red line");
    let blue = lines.next().expect("failed to get blue line");

    let mut red_pos = (VAL / 2, VAL / 2);
    let mut blue_pos = (VAL / 2, VAL / 2);

    for command in red.split(",") {
        //println!("pos: {:?}, command: {:?}", red_pos, command);

        match command.chars().nth(0).expect("Failed to get first char") {
            'R' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_pos.0 += 1;
                    board
                        .get_mut(red_pos.0)
                        .expect("Invalid index")
                        .get_mut(red_pos.1)
                        .expect("Invalid index y")
                        .red = true;
                }
            }
            'L' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_pos.0 -= 1;
                    board
                        .get_mut(red_pos.0)
                        .expect("Invalid index")
                        .get_mut(red_pos.1)
                        .expect("Invalid index y")
                        .red = true;
                }
            }
            'U' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_pos.1 += 1;
                    board
                        .get_mut(red_pos.0)
                        .expect("Invalid index")
                        .get_mut(red_pos.1)
                        .expect("Invalid index y")
                        .red = true;
                }
            }
            'D' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_pos.1 -= 1;
                    board
                        .get_mut(red_pos.0)
                        .expect("Invalid index")
                        .get_mut(red_pos.1)
                        .expect("Invalid index y")
                        .red = true;
                }
            }
            _ => panic!("Not parsing correctly"),
        }
    }

    for command in blue.split(",") {
        //println!("pos: {:?}, command: {:?}", blue_pos, command);

        match command.chars().nth(0).expect("Failed to get first char") {
            'R' => {
                let val = command[1..].parse::<usize>().expect("failed to parse int");
                for _ in 0..val {
                    blue_pos.0 += 1;
                    board[blue_pos.0][blue_pos.1].blue = true;
                }
            }
            'L' => {
                let val = command[1..].parse::<usize>().expect("failed to parse int");
                for _ in 0..val {
                    blue_pos.0 -= 1;
                    board[blue_pos.0][blue_pos.1].blue = true;
                }
            }
            'U' => {
                let val = command[1..].parse::<usize>().expect("failed to parse int");
                for _ in 0..val {
                    blue_pos.1 += 1;
                    board[blue_pos.0][blue_pos.1].blue = true;
                }
            }
            'D' => {
                let val = command[1..].parse::<usize>().expect("failed to parse int");
                for _ in 0..val {
                    blue_pos.1 -= 1;
                    board[blue_pos.0][blue_pos.1].blue = true;
                }
            }
            _ => panic!("Not parsing correctly"),
        }
    }

    let mut closest = 10000000;
    let mut point = ((VAL / 2) as isize, (VAL / 2) as isize);

    for (x, line) in board.iter().enumerate() {
        for (y, tile) in line.iter().enumerate() {
            if tile.red && tile.blue {
                let i = x as isize - (VAL / 2) as isize;
                let j = y as isize - (VAL / 2) as isize;
                let temp = i.abs() + j.abs();
                if temp < closest {
                    closest = temp;
                    point = (i, j);
                    println!("{:?}", point);
                }
            }
        }
    }

    println!(
        "Closest point: {}, {}\nDistance: {}",
        point.0, point.1, closest
    );
}
