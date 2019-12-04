use std::fs::File;
use std::io::prelude::*;
use std::usize;

#[derive(Debug, Clone, Copy)]
struct Tile {
    red: bool,
    blue: bool,
}

#[derive(Debug, Clone, Copy)]
struct Tile2 {
    red: Option<usize>,
    blue: Option<usize>,
}

fn main() {
    part2();
}

fn part2() {
    const VAL: usize = 40000;
    // We will set middle to 1000, 1000

    // Create the board
    let mut board: Vec<Vec<Tile2>> = vec![
        vec![
            Tile2 {
                red: None,
                blue: None,
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
    let mut red_moves = 0;
    let mut blue_pos = (VAL / 2, VAL / 2);
    let mut blue_mov = 0;

    for command in red.split(",") {
        //println!("pos: {:?}, command: {:?}", red_pos, command);

        match command.chars().nth(0).expect("Failed to get first char") {
            'R' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_moves += 1;
                    red_pos.0 += 1;
                    let mut tile = board
                        .get_mut(red_pos.0)
                        .expect("Invalid index")
                        .get_mut(red_pos.1)
                        .expect("Invalid index y");
                    if tile.red == None {
                        tile.red = Some(red_moves);
                    }
                }
            }
            'L' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_moves += 1;
                    red_pos.0 -= 1;
                    let mut tile = board
                        .get_mut(red_pos.0)
                        .expect("Invalid index")
                        .get_mut(red_pos.1)
                        .expect("Invalid index y");
                    if tile.red == None {
                        tile.red = Some(red_moves);
                    }
                }
            }
            'U' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_pos.1 += 1;
                    let mut tile = board
                        .get_mut(red_pos.0)
                        .expect("Invalid index")
                        .get_mut(red_pos.1)
                        .expect("Invalid index y");
                    if tile.red == None {
                        tile.red = Some(red_moves);
                    }
                }
            }
            'D' => {
                let val = command[1..].parse::<usize>().expect("Failed to parse int");
                for _ in 0..val {
                    red_pos.1 -= 1;
                    let mut tile = board
                        .get_mut(red_pos.0)
                        .expect("Invalid index")
                        .get_mut(red_pos.1)
                        .expect("Invalid index y");
                    if tile.red == None {
                        tile.red = Some(red_moves);
                    }
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
                    blue_mov += 1;
                    blue_pos.0 += 1;
                    let mut tile = board[blue_pos.0][blue_pos.1];
                    if tile.blue == None {
                        tile.blue = Some(blue_mov);
                    }
                }
            }
            'L' => {
                let val = command[1..].parse::<usize>().expect("failed to parse int");
                for _ in 0..val {
                    blue_mov += 1;
                    blue_pos.0 -= 1;
                    let mut tile = board[blue_pos.0][blue_pos.1];
                    if tile.blue == None {
                        tile.blue = Some(blue_mov);
                    }
                }
            }
            'U' => {
                let val = command[1..].parse::<usize>().expect("failed to parse int");
                for _ in 0..val {
                    blue_mov += 1;
                    blue_pos.1 += 1;
                    let mut tile = board[blue_pos.0][blue_pos.1];
                    if tile.blue == None {
                        tile.blue = Some(blue_mov);
                    }
                }
            }
            'D' => {
                let val = command[1..].parse::<usize>().expect("failed to parse int");
                for _ in 0..val {
                    blue_mov += 1;
                    blue_pos.1 -= 1;
                    let mut tile = board[blue_pos.0][blue_pos.1];
                    if tile.blue == None {
                        tile.blue = Some(blue_mov);
                    }
                }
            }
            _ => panic!("Not parsing correctly"),
        }
    }

    let mut steps = 10000000;
    let mut point = ((VAL / 2) as isize, (VAL / 2) as isize);

    for (x, line) in board.iter().enumerate() {
        for (y, tile) in line.iter().enumerate() {
            match (tile.red, tile.blue) {
                (Some(i), Some(j)) => {
                    let temp = i + j ;
                    if temp < steps {
                        steps = temp;
                        point = (x as isize - (VAL / 2) as isize, y as isize - (VAL / 2) as isize);
                        println!("{:?}, {}", point, steps);
                    }
                },
                _ => {},
            }
        }
    }

    println!(
        "Lest steps point: {}, {}\nSteps: {}",
        point.0, point.1, steps
    );
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
