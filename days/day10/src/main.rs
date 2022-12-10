use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

enum Instruction {
    AddX(isize),
    Noop,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut str_it = s.split_ascii_whitespace().take(2);
        let instruction_str = str_it
            .next()
            .ok_or("Can't parse empty string to instruction")?;
        let num_str_opt = str_it.next();

        match instruction_str {
            "addx" => {
                let num_str = num_str_opt.ok_or("Expected number string not found")?;
                let num = num_str.parse::<isize>().map_err(|e| e.to_string())?;
                Ok(Self::AddX(num))
            }
            "noop" => Ok(Self::Noop),
            other => Err(format!("Invalid instruction string '{}'", other)),
        }
    }
}

struct Cpu {
    register_x: isize,

    instructions: Vec<Instruction>,
    program_counter: usize,

    cycles_left: usize,
    current_cycle: usize,

    crt: [[char; 40]; 6],
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        let crt = [['.'; 40]; 6];

        let mut cpu = Cpu {
            register_x: 1,
            instructions,
            program_counter: 0,
            cycles_left: 0,
            current_cycle: 0,
            crt,
        };
        cpu.load_next_instruction();

        cpu
    }

    fn load_next_instruction(&mut self) {
        match &self.instructions[self.program_counter] {
            Instruction::AddX(_) => self.cycles_left += 2,
            Instruction::Noop => self.cycles_left += 1,
        }
    }

    fn finish_instruction(&mut self) -> bool {
        match &self.instructions[self.program_counter] {
            Instruction::AddX(num) => self.register_x += num,
            Instruction::Noop => (),
        }

        self.program_counter += 1;
        self.program_counter < self.instructions.len()
    }

    fn run_cycle(&mut self) -> bool {
        self.current_cycle += 1;

        if self.cycles_left == 0 {
            if !self.finish_instruction() {
                return false;
            }
            self.load_next_instruction();
        }

        let crt_pixel_x = ((self.current_cycle - 1) % 40) as isize;
        if crt_pixel_x >= self.register_x - 1 && crt_pixel_x <= self.register_x + 1 {
            let crt_pixel_y = (self.current_cycle - 1) / 40;
            self.crt[crt_pixel_y][crt_pixel_x as usize] = '#';
        }

        self.cycles_left -= 1;

        true
    }

    fn get_crt(&self) -> String {
        self.crt
            .iter()
            .map(String::from_iter)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<Vec<Instruction>> {
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let mut instructions = Vec::new();
    for line in input_buf.lines() {
        let line = line?;

        let instruction = Instruction::from_str(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        instructions.push(instruction);
    }

    Ok(instructions)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let instructions = parse_input("inputs/day10.in")?;
    let mut cpu = Cpu::new(instructions);
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let mut p1_signal_strength_sum = 0;
    while cpu.run_cycle() {
        if cpu.current_cycle >= 20 && (cpu.current_cycle - 20) % 40 == 0 {
            p1_signal_strength_sum += cpu.register_x * cpu.current_cycle as isize;
        }
    }
    let part1_time = t1.elapsed();

    // Compute part 2 and time it (in this case there's no extra computation)
    let t2 = Instant::now();
    let p2_crt_str = cpu.get_crt();
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 - Sum of signal strenghts: {}\n",
        part1_time, p1_signal_strength_sum
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 - CRT screen:\n{}\n",
        part2_time, p2_crt_str,
    );

    Ok(())
}
