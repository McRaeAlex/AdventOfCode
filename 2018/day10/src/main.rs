use std::io::{self, Read};

// 10630 is the time we get the smallest clustering
fn main() {
    // read in locations and velocities
    let lights: Vec<((i64, i64), (i64, i64))> = read_input();
    println!("Done Parsing input");
    // start time
    let (time, width, height) = find_smallest_clustering(&lights);
    println!(
        "Done step 2: time: {} width: {} height: {}",
        time, width, height
    );
    // find the times
    let (min_x, min_y) = find_mins_at_time(&lights, time);
    println!("minX: {}, minY: {}", min_x, min_y);
    // output
    let mut display: Vec<char> = Vec::with_capacity((width * (height + 1)) as usize);
    for i in 0..(width * (height + 1)) {
        display.push(' ');
    }
    for light in &lights {
        let y = (light.0).1 + (light.1).1 * time;
        let x = (light.0).0 + (light.1).0 * time;
        display[(y * width + x) as usize] = '*';
    }
    for (i, charr) in display.iter().enumerate() {
        print!("{}", charr);
        if i % width as usize == 0 {
            println!("");
        }
    }
    println!("\nTime: {}", time);
}

fn read_input() -> Vec<((i64, i64), (i64, i64))> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Error reading in");
    let lines: Vec<&str> = buffer.as_str().lines().collect();
    let mut result: Vec<((i64, i64), (i64, i64))> = vec![];
    for line in lines {
        let posx: i64 = line[10..16].trim().parse::<i64>().unwrap();
        let posy: i64 = line[18..24].trim().parse::<i64>().unwrap();
        let velx: i64 = line[36..38].trim().parse::<i64>().unwrap();
        let vely: i64 = line[40..42].trim().parse::<i64>().unwrap();
        //println!("{} {} {} {}", posx, posy, velx, vely);
        result.push(((posx, posy), (velx, vely)));
    }
    result
}

fn find_smallest_clustering(lights: &Vec<((i64, i64), (i64, i64))>) -> (i64, i64, i64) {
    let mut min_time = 0;
    let mut min_y_diff = 10000;
    let mut x_diff = 0;
    for time in 0..20000 {
        let mut min_y = 0;
        let mut max_y = 0;
        let mut min_x = 0;
        let mut max_x = 0;
        for light in lights {
            let y = (light.0).1 + (light.1).1 * time;
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
            let x = (light.0).0 + (light.1).0 * time;
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
        }
        /*println!(
            "time: {} maxY: {} minY: {} diff: {}",
            time,
            max_y,
            min_y,
            max_y - min_y
        );*/
        if min_y_diff > max_y - min_y {
            min_y_diff = max_y - min_y;
            min_time = time;
            x_diff = max_x - min_x;
        }
    }
    (min_time, x_diff, min_y_diff)
}

fn find_mins_at_time(lights: &Vec<((i64, i64), (i64, i64))>, time: i64) -> (i64, i64) {
    let mut min_y = 0;
    let mut min_x = 0;
    for light in lights {
        let y = (light.0).1 + (light.1).1 * time;
        if y < min_y {
            min_y = y;
        }

        let x = (light.0).0 + (light.1).0 * time;
        if x < min_x {
            min_x = x;
        }
    }
    (min_x, min_y)
}
