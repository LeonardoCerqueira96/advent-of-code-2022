use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use regex::Regex;

#[derive(Debug)]
struct MoveInstruction {
    amount: usize,
    origin: usize,
    destination: usize,
}

impl MoveInstruction {
    fn new(amount: usize, origin: usize, destination: usize) -> Self {
        MoveInstruction {
            amount,
            origin,
            destination,
        }
    }
}

enum MoverModel {
    CM9000,
    CM9001,
}

struct Cargo {
    stacks: Vec<Vec<char>>,
    model: MoverModel,
}

impl Display for Cargo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_height = self
            .stacks
            .iter()
            .map(|s| s.len())
            .max()
            .expect("No stacks to display");
        while max_height > 0 {
            for (i, stack) in self.stacks.iter().enumerate() {
                let suffix = if i == self.stacks.len() - 1 { "" } else { " " };
                let crate_rep = if let Some(name) = stack.get(max_height - 1) {
                    format!("[{}]", name)
                } else {
                    "   ".to_string()
                };
                write!(f, "{}{}", crate_rep, suffix)?;
            }
            writeln!(f)?;

            max_height -= 1;
        }

        let crate_numbers = (1..=self.stacks.len())
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("   ");
        write!(f, " {}", crate_numbers)
    }
}

impl Cargo {
    fn new(stacks: Vec<Vec<char>>, model: MoverModel) -> Self {
        Cargo { stacks, model }
    }

    fn move_cargo(&mut self, instruction: &MoveInstruction) {
        match self.model {
            MoverModel::CM9000 => {
                for _ in 0..instruction.amount {
                    let moving_crate = self.stacks[instruction.origin - 1]
                        .pop()
                        .expect("Stack is empty");
                    self.stacks[instruction.destination - 1].push(moving_crate);
                }
            }
            MoverModel::CM9001 => {
                let split_index = self.stacks[instruction.origin - 1]
                    .len()
                    .saturating_sub(instruction.amount);
                let moving_crates = self.stacks[instruction.origin - 1].split_off(split_index);
                self.stacks[instruction.destination - 1].extend(moving_crates);
            }
        }
    }

    fn get_top_string(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last().map_or("".to_string(), |c| c.to_string()))
            .collect::<Vec<_>>()
            .join("")
    }
}

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<(Vec<Vec<char>>, Vec<MoveInstruction>)> {
    // Setup regexes
    let stack_re = Regex::new(r"(\s{3}|(?:\[(\w)\]))\s?").expect("Error compiling regex");
    let move_instruction_re =
        Regex::new(r"move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)").expect("Error compiling regex");

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);
    let mut lines_it = input_buf.lines();

    // Parse the stacks first
    let mut stacks = Vec::new();
    loop {
        let line = lines_it.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Expected input not found")
        })??;

        // Allocate stacks
        if stacks.is_empty() {
            let stacks_amount = (line.len() + 1) / 4;
            stacks.resize_with(stacks_amount, Vec::new);
        }

        // Done reading the stacks
        if line.starts_with(" 1 ") {
            // Skip the empty linee
            _ = lines_it.next();
            break;
        }

        // Push crates to their stacks
        for (i, cap) in stack_re.captures_iter(&line).enumerate() {
            if let Some(crate_match) = cap.get(2) {
                let crate_name = crate_match
                    .as_str()
                    .chars()
                    .next()
                    .expect("Match string is empty");
                stacks[i].insert(0, crate_name);
            }
        }
    }

    // Parse move instructions
    let mut instructions = Vec::new();
    for line in lines_it {
        let line = line?;

        // Capture numbers
        let number_cap = move_instruction_re
            .captures(&line)
            .expect("Regex didn't match the input");
        let amount = number_cap
            .get(1)
            .expect("Didn't match amount to move")
            .as_str()
            .parse()
            .expect("Failed to parse number");
        let origin = number_cap
            .get(2)
            .expect("Didn't match origin to move from")
            .as_str()
            .parse()
            .expect("Failed to parse number");
        let destination = number_cap
            .get(3)
            .expect("Didn't match destination to move to")
            .as_str()
            .parse()
            .expect("Failed to parse number");
        instructions.push(MoveInstruction::new(amount, origin, destination));
    }

    Ok((stacks, instructions))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (stacks, instructions) = parse_input("inputs/day05.in")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let mut cargo_p1 = Cargo::new(stacks.clone(), MoverModel::CM9000);
    for inst in &instructions {
        cargo_p1.move_cargo(inst);
    }
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let mut cargo_p2 = Cargo::new(stacks, MoverModel::CM9001);
    for inst in &instructions {
        cargo_p2.move_cargo(inst);
    }
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 final distribution:\n\n{}\nTop string: {}\n",
        part1_time,
        cargo_p1,
        cargo_p1.get_top_string()
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 final distribution:\n\n{}\nTop string: {}\n",
        part2_time,
        cargo_p2,
        cargo_p2.get_top_string()
    );

    Ok(())
}
