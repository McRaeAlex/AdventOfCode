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
    let mut before: [i64; 4] = [0, 0, 0, 0];
    let mut instruction: (usize, usize, usize, usize) = (0, 0, 0, 0);
    let mut after: [i64; 4] = [0, 0, 0, 0];
    let mut double_newline: bool = false;
    let mut result: usize = 0;
    /**
     * this is a matrix of values if the value is none then we know that its not
     * that opcode. if it is some then we are not sure.
     */
    let mut opcode_matrix: Vec<Vec<bool>> = vec![vec![true; 16]; 16];

    for (line_num, line) in stdin.lock().lines().enumerate() {
        //println!("Line num: {}", line_num);
        let line = line.unwrap();
        //println!("Line: {}", line);
        //println!("Line Val: {}", line == "");
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
                if simulate_instructions(&mut before, &instruction, &mut after, &mut opcode_matrix)
                    >= 3
                {
                    result += 1;
                }
                //println!("{:?}", opcode_matrix);
            }
            (_, true) => {
                if double_newline == true {
                    println!("Result: {}", result);
                    break;
                } else {
                    double_newline = true;
                }
            }
            (_, _) => {
                panic!("Math no longer works");
            }
        }
    }

    println!("Starting matrix breakdown");
    // now we go through the matrix and find all the ones with only one in them get rid of that function
    // in the others
    /**
     * while not all found {
     *  iterate through matrix and find one with only one.
     *  save the index of the function it maps too.
     *  remove that function from all rows.
     * }
     */
    let mut found: usize = 0;
    while found < 16 {
        let mut index_to_remove: Option<usize> = None;
        for i in 0..opcode_matrix.len() {
            let row = &mut opcode_matrix[i];
            //println!("{}: {:?}", i, row);
            let mut count = 0;
            let mut index = 0;
            for j in 0..row.len() {
                let val = row[j];
                if val == true {
                    count += 1;
                    index = j;
                }
            }
            if count == 1 {
                println!("Found opcode {}: {}", i, index);
                found += 1;
                index_to_remove = Some(index);
                break;
            }
        }
        match index_to_remove {
            Some(index) => {
                //println!("Removing {}", index);
                for i in 0..opcode_matrix.len() {
                    opcode_matrix[i][index] = false;
                }
            }
            None => {
                // in the event we cannot find a w row with only one value we must find a column with only one row. we can then say that row definitely has that mapping and we can get rid of the other values from the row allowing the loop to start again.
                //let mut some_name: Option<usize> = None;
                for j in 0..opcode_matrix[0].len() {
                    let mut count = 0;
                    let mut index = 0;
                    for i in 0..opcode_matrix.len() {
                        let val = opcode_matrix[i][j];
                        if val == true {
                            count += 1;
                            index = i;
                        }
                    }
                    if count == 1 {
                        for i in 0..opcode_matrix[index].len() {
                            opcode_matrix[index][i] = false;
                        }
                        opcode_matrix[index][j] = true;
                        break;
                    }
                }
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
