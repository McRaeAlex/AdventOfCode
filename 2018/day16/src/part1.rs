use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut before: [i64; 4] = [0, 0, 0, 0];
    let mut instruction: (usize, usize, usize, usize) = (0, 0, 0, 0);
    let mut after: [i64; 4] = [0, 0, 0, 0];
    let mut double_newline: bool = false;
    let mut result: usize = 0;

    for (line_num, line) in stdin.lock().lines().enumerate() {
        println!("Line num: {}", line_num);
        let line = line.unwrap();
        println!("Line: {}", line);
        println!("Line Val: {}", line == "");
        match (line_num % 4, line == "") {
            (0, false) => {
                double_newline = false;
                // set the before array to the array
                parse_array(&mut before, line)
            }
            (1, false) => {
                double_newline = false;
                // set this to the instruction
                instruction = parse_instruction(line);
            }
            (2, false) => {
                double_newline = false;
                // set the after array to the value
                parse_array(&mut after, line);
                // simulate all the instructions then compare the outputs to the
                // output array values
                if simulate_instructions(&mut before, &instruction, &mut after) >= 3 {
                    result += 1;
                }
            }
            (_, true) => {
                if double_newline == true {
                    println!("Result: {}", result);
                    return;
                } else {
                    double_newline = true;
                }
            }
            (_, _) => {
                panic!("Math no longer works");
            }
        }
    }
}

fn parse_array(array: &mut [i64; 4], line: String) {
    // parse the string
    // write the values into the array
    let chars: Vec<char> = line.chars().collect();
    array[0] = chars[9].to_digit(10).unwrap() as i64;
    array[1] = chars[12].to_digit(10).unwrap() as i64;
    array[2] = chars[15].to_digit(10).unwrap() as i64;
    array[3] = chars[18].to_digit(10).unwrap() as i64;
}

fn parse_instruction(line: String) -> (usize, usize, usize, usize) {
    // parse the string into a tuple then return the tuple
    let instruct: Vec<usize> = line
        .split_whitespace()
        .map(|val| val.parse::<usize>().unwrap())
        .collect();
    return (instruct[0], instruct[1], instruct[2], instruct[3]);
}

fn simulate_instructions(
    before: &mut [i64; 4],
    instruction: &(usize, usize, usize, usize),
    after: &mut [i64; 4],
) -> usize {
    let mut result = 0;
    for func in [
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ]
        .iter()
    {
        // make a copy of the registers
        let mut copy: [i64; 4] = *before;
        func(&mut copy, instruction);
        // compare copy of register to after
        if copy == *after {
            result += 1;
        }
    }

    return result;
}

fn addr(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1] + registers[instruction.2];
}

fn addi(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1] + instruction.2 as i64;
}

fn mulr(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1] * registers[instruction.2];
}

fn muli(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1] * instruction.2 as i64;
}

fn banr(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1] & registers[instruction.2];
}

fn bani(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1] & instruction.2 as i64;
}

fn borr(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1] | registers[instruction.2];
}

fn bori(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1] | instruction.2 as i64;
}

fn setr(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = registers[instruction.1];
}

fn seti(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = instruction.1 as i64;
}

fn gtir(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = match instruction.1 as i64 > registers[instruction.2] {
        true => 1,
        false => 0,
    };
}

fn gtri(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = match registers[instruction.1] > instruction.2 as i64 {
        true => 1,
        false => 0,
    };
}

fn gtrr(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = match registers[instruction.1] > registers[instruction.2] {
        true => 1,
        false => 0,
    };
}

fn eqir(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = match instruction.1 as i64 == registers[instruction.2] {
        true => 1,
        false => 0,
    };
}

fn eqri(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = match registers[instruction.1] == instruction.2 as i64 {
        true => 1,
        false => 0,
    };
}

fn eqrr(registers: &mut [i64; 4], instruction: &(usize, usize, usize, usize)) {
    registers[instruction.3] = match registers[instruction.1] == registers[instruction.2] {
        true => 1,
        false => 0,
    };
}
