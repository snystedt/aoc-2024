use std::sync::LazyLock;

use bitvec::{bitvec, order::Lsb0, slice::BitSlice, vec::BitVec};

use aoc_2024::input::read_lines;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, thiserror::Error)]
pub enum ParseEntityError {
    #[error("unexpected EOF while parsing input")]
    UnexpectedEndOfFile,
    #[error("expected register at line {0}")]
    ExpectedRegister(usize),
    #[error("invalid program format: {0}")]
    InvalidProgramFormat(String),
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

#[derive(Debug, thiserror::Error)]
pub enum ThreeBitError {
    #[error("Invalid bit count for op code")]
    InvalidBitCount,
}

impl TryFrom<&BitSlice<u8>> for Opcode {
    type Error = ThreeBitError;

    fn try_from(value: &BitSlice<u8>) -> Result<Self, Self::Error> {
        let v = u8_from_3bit_value(value)?;

        Ok(match v {
            0 => Opcode::ADV,
            1 => Opcode::BXL,
            2 => Opcode::BST,
            3 => Opcode::JNZ,
            4 => Opcode::BXC,
            5 => Opcode::OUT,
            6 => Opcode::BDV,
            7 => Opcode::CDV,
            _ => unreachable!(),
        })
    }
}

fn u8_from_3bit_value(value: &BitSlice<u8, Lsb0>) -> Result<u8, ThreeBitError> {
    if value.len() != 3 {
        Err(ThreeBitError::InvalidBitCount)
    } else {
        Ok((value[0] as u8) | ((value[1] as u8) << 1) | ((value[2] as u8) << 2))
    }
}

#[derive(Debug)]
struct Computer {
    registers: [usize; 3],
    program: BitVec<u8, Lsb0>,
    pc: usize,
    output: Vec<u8>,
}

impl Computer {
    pub fn new(registers: [usize; 3], program: BitVec<u8, Lsb0>) -> Self {
        Self {
            registers,
            program,
            pc: 0,
            output: vec![],
        }
    }

    pub fn reset(&mut self, a: usize) {
        self.pc = 0;
        self.registers[0] = a;
        self.registers[1] = 0;
        self.registers[2] = 0;
        self.output.clear();
    }

    fn operand_to_combo(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        let combo = self.operand_to_combo(operand);
        self.registers[0] = self.registers[0] / 2usize.pow(combo as u32);
        self.pc += 6;
    }

    fn bxl(&mut self, operand: u8) {
        self.registers[1] ^= operand as usize;
        self.pc += 6;
    }

    fn bst(&mut self, operand: u8) {
        let combo = self.operand_to_combo(operand);
        self.registers[1] = (combo & 0b111) as usize;
        self.pc += 6;
    }

    fn jnz(&mut self, operand: u8) {
        if self.registers[0] != 0 {
            self.pc = operand as usize;
        } else {
            self.pc += 6;
        }
    }

    fn bxc(&mut self, _: u8) {
        self.registers[1] = self.registers[1] ^ self.registers[2];
        self.pc += 6;
    }

    fn out(&mut self, operand: u8) {
        let combo = (self.operand_to_combo(operand) & 0b111) as u8;
        self.output.push(combo);
        self.pc += 6;
    }

    fn bdv(&mut self, operand: u8) {
        let combo = self.operand_to_combo(operand);
        self.registers[1] = self.registers[0] / 2usize.pow(combo as u32);
        self.pc += 6;
    }

    fn cdv(&mut self, operand: u8) {
        let combo = self.operand_to_combo(operand);
        self.registers[2] = (self.registers[0] / 2usize.pow(combo as u32)) & 0b111;
        self.pc += 6;
    }

    pub fn get_output(&self) -> &[u8] {
        &self.output
    }

    pub fn run(&mut self) -> Result<bool, ThreeBitError> {
        let opcode: Opcode = self.program[self.pc..self.pc + 3].try_into()?;
        let operand = u8_from_3bit_value(&self.program[self.pc + 3..self.pc + 6])?;

        match opcode {
            Opcode::ADV => self.adv(operand),
            Opcode::BXL => self.bxl(operand),
            Opcode::BST => self.bst(operand),
            Opcode::JNZ => self.jnz(operand),
            Opcode::BXC => self.bxc(operand),
            Opcode::OUT => self.out(operand),
            Opcode::BDV => self.bdv(operand),
            Opcode::CDV => self.cdv(operand),
        }

        if self.pc == self.program.len() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn parse_register(line: String) -> Result<usize, ParseEntityError> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Register [A-C]: (\d+)").unwrap());

    RE.captures(&line)
        .ok_or(ParseEntityError::ExpectedRegister(0))
        .map(|c| c.extract())
        .map(|(_, [value])| value.parse::<usize>().unwrap()) // Matches \d+ so can't fail
}

fn parse_program(line: String) -> Result<BitVec<u8, Lsb0>, ParseEntityError> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Program: ([\d,]+)").unwrap());

    let op_codes = RE
        .captures(&line)
        .ok_or(ParseEntityError::InvalidProgramFormat(line.clone()))
        .map(|c| c.extract())
        .map(|(_, [value])| value)?;

    let store_instruction = |store: &mut BitVec<u8>, opcode: u8| {
        [0b1u8, 0b10, 0b100]
            .into_iter()
            .for_each(|b| store.push(opcode & b == b))
    };

    let mut code = bitvec![u8, Lsb0;];
    for op_code in op_codes.split(",") {
        let instruction = op_code
            .parse::<u8>()
            .map_err(|_| ParseEntityError::InvalidProgramFormat(op_codes.to_string()))?;
        store_instruction(&mut code, instruction);
    }

    Ok(code)
}

fn parse_input(mut lines: impl Iterator<Item = String>) -> Result<Computer, ParseEntityError> {
    let a = lines
        .next()
        .ok_or(ParseEntityError::UnexpectedEndOfFile)
        .and_then(parse_register)?;

    let b = lines
        .next()
        .ok_or(ParseEntityError::UnexpectedEndOfFile)
        .and_then(parse_register)?;

    let c = lines
        .next()
        .ok_or(ParseEntityError::UnexpectedEndOfFile)
        .and_then(parse_register)?;

    assert!(lines.next().is_some_and(|l| l.is_empty()));

    let program = lines
        .next()
        .ok_or(ParseEntityError::UnexpectedEndOfFile)
        .and_then(parse_program)?;

    Ok(Computer::new([a, b, c], program))
}

#[inline(always)]
fn pseudo_computer(mut a: usize, output: &mut Vec<u8>) {
    while a > 0 {
        let mut b = a & 0b111;
        b = b ^ 0b010;
        let c = a >> b;
        b = b ^ c;
        b = b ^ 0b011;
        output.push((b & 0b111) as u8);
        a = a >> 3;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_lines("./inputs/day17/input.txt")?;
    let mut computer = parse_input(lines.flatten().into_iter())?;

    while !computer.run()? {}

    println!(
        "Output: {}",
        computer
            .get_output()
            .iter()
            .map(|v| v.to_string())
            .join(",")
    );

    let mut options = (0usize..128).collect_vec();
    let mut old_options = Vec::with_capacity(128);
    let mut new_out = Vec::with_capacity(32);

    let code = vec![2, 4, 1, 2, 7, 5, 4, 7, 1, 3, 5, 5, 0, 3, 3, 0];

    let mut total_iterations = 0usize;

    for idx in 0..code.len() {
        old_options.clear();
        old_options.extend(options.drain(..));
        for &option in old_options.iter() {
            for i in 0..8 {
                total_iterations += 1;
                let a = option + (i << (7 + (3 * idx)));
                new_out.clear();
                pseudo_computer(a, &mut new_out);
                if new_out.len() > idx
                    && new_out
                        .iter()
                        .zip(code.iter())
                        .take(idx + 1)
                        .all(|(a, b)| *a == *b)
                {
                    options.push(a);
                }
            }
        }
    }

    println!("Total iterations: {}", total_iterations);

    let ans = options.into_iter().min().unwrap();

    println!("Initial register value: {}", ans);

    computer.reset(ans);
    while !computer.run()? {}

    println!(
        "New output: {}",
        computer
            .get_output()
            .iter()
            .map(|v| v.to_string())
            .join(",")
    );

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn diff_a() {
        for a in 0usize..8 {
            let mut b = a & 0b111;
            b = b ^ 0b010;
            let c = (a >> b) & 0b111;
            b = b ^ c;
            b = b ^ 0b011;

            println!("a = {}, b = {}, c = {}", a, b, c);
        }
    }
}
