use std::io::{self, prelude::*};

/**
 * SCRATCH NOTES:
 * we need to create a list of all the functions each opcode could be.
 * then we need to iterate over that list removing all of the ones that only
 * have one opcode.
 * like if we see that 9 is addr we can remove addr from the other lists. if we
 * do this each time we should get a list that is complete or close to complete.
 * We can then use this table to solve the next part. I think it would be easier
 * to just look at the results and map manually for the second part of the 2nd
 * part. There are not that many functions and once we know we know.
 * Results:
Starting matrix breakdown
Found opcode 14: 13
Found opcode 6: 15
Found opcode 7: 11
Found opcode 0: 12
Found opcode 3: 14
Found opcode 2: 10
Found opcode 8: 4
Found opcode 10: 8
Found opcode 15: 5
Found opcode 5: 9
Found opcode 4: 0
Found opcode 9: 1
Found opcode 13: 3
Found opcode 12: 7
Found opcode 1: 6
Found opcode 11: 2
 */

fn main() {
    let stdin = io::stdin();
    let mut registers = [0, 0, 0, 0];

    for (line_num, line) in stdin.lock().lines().enumerate() {
        //println!("Line num: {}", line_num);
        let line = line.unwrap();
        if line == "\n" {
            continue;
        }
        let instruction = parse_instruction(line);
        match instruction.0 {
            0 => gtrr(&mut registers, &instruction),
            1 => borr(&mut registers, &instruction),
            2 => gtir(&mut registers, &instruction),
            3 => eqri(&mut registers, &instruction),
            4 => addr(&mut registers, &instruction),
            5 => seti(&mut registers, &instruction),
            6 => eqrr(&mut registers, &instruction),
            7 => gtri(&mut registers, &instruction),
            8 => banr(&mut registers, &instruction),
            9 => addi(&mut registers, &instruction),
            10 => setr(&mut registers, &instruction),
            11 => mulr(&mut registers, &instruction),
            12 => bori(&mut registers, &instruction),
            13 => muli(&mut registers, &instruction),
            14 => eqir(&mut registers, &instruction),
            15 => bani(&mut registers, &instruction),
            _ => panic!("ERROR"),
        }
    }
    println!("{:?}", registers);
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
    opcode_matrix: &mut Vec<Vec<bool>>,
) -> usize {
    let mut result = 0;
    let function_list = [
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];
    for (i, func) in function_list.iter().enumerate() {
        // make a copy of the registers
        let mut copy: [i64; 4] = *before;
        func(&mut copy, instruction);
        // compare copy of register to after
        if copy == *after {
            result += 1;
        } else {
            opcode_matrix[instruction.0][i] = false;
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
