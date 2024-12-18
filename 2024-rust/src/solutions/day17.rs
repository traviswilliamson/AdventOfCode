use std::{error::Error, u32};

use itertools::Itertools;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut halves = _input.trim().split("\n\n");
    let mut registers = halves.next().unwrap().split("\n").map(|l| {
        l.split(": ").skip(1).next().unwrap().parse::<u32>().unwrap()
    }).collect::<Vec<u32>>();
    let commands = halves.next().unwrap().split(": ").skip(1).next().unwrap()
        .split(",").map(|i| i.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    // println!("{:?}", registers);
    // println!("{:?}", commands);
    let mut instruction_pointer = 0;
    let mut output: Vec<u32> = Vec::new();

    // Okay, go time
    while instruction_pointer < commands.len() {
        let command = commands[instruction_pointer];
        let literal_operand = commands[instruction_pointer + 1];
        let combo_operand = match literal_operand {
            0 | 1 | 2 | 3 => literal_operand,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            7 => u32::MAX,
            _ => panic!("Bad input of {:?}", literal_operand)
        };
        // println!("{:?}", command);
        // println!("{:?}", literal_operand);
        // println!("{:?}", combo_operand);
        // println!("{:?}\n", registers);
        match command {
            0 => registers[0] = registers[0].checked_shr(combo_operand).unwrap_or(0),
            1 => registers[1] = registers[1] ^ literal_operand,
            2 => registers[1] = combo_operand & 0b111,
            3 => {
                if registers[0] > 0 {
                    instruction_pointer = literal_operand as usize;
                    continue;
                }
            }
            4 => registers[1] = registers[1] ^ registers[2],
            5 => output.push(combo_operand & 0b111),
            6 => registers[1] = registers[0].checked_shr(combo_operand).unwrap_or(0),
            7 => registers[2] = registers[0].checked_shr(combo_operand).unwrap_or(0),
            _ => panic!("Bad command of {:?}", command)
        }
        instruction_pointer += 2;
    }

    Ok(output.iter().map(|u| u.to_string()).join(","))
}

static mut PROGRAM: Vec<u64> = vec![];

fn find_a_for_output(index_of_output_to_sieve: i32, a_to_check: u64) -> Option<u64> {
    if index_of_output_to_sieve < 0 {
        // We done!
        Some(a_to_check)
    }
    else {
        for low_digits in 0..8 {
            // Trust me, it's fine
            unsafe {
                let mut a = (a_to_check << 3) | low_digits;
                // Turns out, these are reset every loop
                let mut b = 0_u64;
                let mut c = 0_u64;
                let mut instruction_pointer = 0;
                let mut output = u64::MAX;

                while instruction_pointer < PROGRAM.len() {
                    let command = PROGRAM[instruction_pointer];
                    let literal_operand = PROGRAM[instruction_pointer + 1];
                    let combo_operand = match literal_operand {
                        0 | 1 | 2 | 3 => literal_operand,
                        4 => a,
                        5 => b,
                        6 => c,
                        7 => u64::MAX,
                        _ => panic!("Bad input of {:?}", literal_operand)
                    };
                    // println!("{:?}", command);
                    // println!("{:?}", literal_operand);
                    // println!("{:?}", combo_operand);
                    match command {
                        // Why doesn't it have a 64 bit check shifted function?
                        0 => a = a.checked_shr(combo_operand.try_into().unwrap()).unwrap_or(0),
                        1 => b = b ^ literal_operand,
                        2 => b = combo_operand & 0b111,
                        3 => {
                            if a > 0 {
                                instruction_pointer = literal_operand as usize;
                                continue;
                            }
                        }
                        4 => b = b ^ c,
                        5 => {
                            // New stuff here
                            // We're only looking for one output
                            output = combo_operand & 0b111;
                            break;
                        },
                        6 => b = a.checked_shr(combo_operand.try_into().unwrap()).unwrap_or(0),
                        7 => c = a.checked_shr(combo_operand.try_into().unwrap()).unwrap_or(0),
                        _ => panic!("Bad command of {:?}", command)
                    }
                    instruction_pointer += 2;
                }

                if output == PROGRAM[index_of_output_to_sieve as usize] {
                    let bigger_a = find_a_for_output(index_of_output_to_sieve - 1, (a_to_check << 3) | low_digits);
                    if bigger_a.is_some() {
                        // println!("Found output {:?} at index {:?}", output, index_of_output_to_sieve);
                        // println!("A was {:?}, trying out {:?}", a_to_check, (a_to_check << 3) | low_digits);
                        return bigger_a;
                    }
                }
            }
        }
        None
    }
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    // I know, trust me
    unsafe {
        PROGRAM = _input.trim().split("\n\n").skip(1).next().unwrap().split(": ").skip(1).next().unwrap()
            .split(",").map(|i| i.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        // println!("{:?}", PROGRAM);
        Ok(find_a_for_output((PROGRAM.len() - 1) as i32, 0).unwrap().to_string())
    }
}
