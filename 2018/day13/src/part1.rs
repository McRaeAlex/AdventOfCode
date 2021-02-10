use std::io::{self, prelude::*};

fn main() {
    let (map, mut carts) = parse_input();
    display_map(&map);
    display_map_and_carts(&map, &carts);
    let mut tick = 0;
    loop {
        for cart in &mut carts {
            move_cart(cart);
            match map[cart.pos.1][cart.pos.0] {
                '\\' => match cart.direction {
                    0 => cart.direction = 3,
                    1 => cart.direction = 2,
                    2 => cart.direction = 1,
                    3 => cart.direction = 0,
                    _ => {}
                },
                '/' => match cart.direction {
                    0 => cart.direction = 1,
                    1 => cart.direction = 0,
                    2 => cart.direction = 3,
                    3 => cart.direction = 2,
                    _ => {}
                },
                '+' => match cart.step {
                    0 => {
                        match cart.direction {
                            0 => cart.direction = 3,
                            1 => cart.direction = 0,
                            2 => cart.direction = 1,
                            3 => cart.direction = 2,
                            _ => {}
                        }
                        cart.step += 1;
                    }
                    1 => {
                        cart.step += 1;
                    }
                    2 => {
                        match cart.direction {
                            0 => cart.direction = 1,
                            1 => cart.direction = 2,
                            2 => cart.direction = 3,
                            3 => cart.direction = 0,
                            _ => {}
                        }
                        cart.step = 0;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        tick += 1;
        match display_map_and_carts(&map, &carts) {
            Some(x) => {
                println!("{:?}", x);
                return;
            }
            None => {}
        }
    }
}

#[derive(Debug)]
struct Cart {
    direction: u8, // 0:up, 1:right, 2:down, 3:left
    pos: (usize, usize),
    step: usize,
}

fn move_cart(cart: &mut Cart) {
    match cart.direction {
        0 => cart.pos = (cart.pos.0, cart.pos.1 - 1),
        1 => cart.pos = (cart.pos.0 + 1, cart.pos.1),
        2 => cart.pos = (cart.pos.0, cart.pos.1 + 1),
        3 => cart.pos = (cart.pos.0 - 1, cart.pos.1),
        _ => {}
    }
}

fn display_map(map: &Vec<Vec<char>>) {
    for y in map {
        for x in y {
            print!("{}", x);
        }
        println!("");
    }
}

fn display_map_and_carts(map: &Vec<Vec<char>>, carts: &Vec<Cart>) -> Option<(usize, usize)> {
    let mut ret: Option<(usize, usize)> = None;
    let mut display = map.clone();
    for cart in carts {
        println!("{:?}", cart);
        match display[cart.pos.1][cart.pos.0] {
            '^' | '>' | '<' | 'v' => {
                ret = Some(cart.pos);
                display[cart.pos.1][cart.pos.0] = 'X';
                continue;
            }
            _ => {}
        };
        display[cart.pos.1][cart.pos.0] = match cart.direction {
            0 => '^',
            1 => '>',
            2 => 'v',
            3 => '<',
            _ => 'O',
        };
    }
    for y in display {
        for x in y {
            print!("{}", x);
        }
        println!("");
    }
    ret
}

fn parse_input() -> (Vec<Vec<char>>, Vec<Cart>) {
    let stdin = io::stdin();
    let mut tracks: Vec<Vec<char>> = vec![];
    let mut carts: Vec<Cart> = vec![];
    for (y, line) in stdin.lock().lines().enumerate() {
        let mut new_track: Vec<char> = vec![];
        for (x, track) in line.unwrap().chars().enumerate() {
            match track {
                '>' => {
                    carts.push(Cart {
                        direction: 1,
                        pos: (x, y),
                        step: 0,
                    });
                    new_track.push('-');
                }
                '<' => {
                    carts.push(Cart {
                        direction: 3,
                        pos: (x, y),
                        step: 0,
                    });
                    new_track.push('-');
                }
                '^' => {
                    carts.push(Cart {
                        direction: 0,
                        pos: (x, y),
                        step: 0,
                    });
                    new_track.push('|');
                }
                'v' => {
                    carts.push(Cart {
                        direction: 2,
                        pos: (x, y),
                        step: 0,
                    });
                    new_track.push('|');
                }
                _ => new_track.push(track),
            }
        }
        tracks.push(new_track);
    }
    (tracks, carts)
}
