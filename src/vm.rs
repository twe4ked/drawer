use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Toggle if we're drawing or not
    Draw,
    /// End the program
    Halt,
    /// Move in direction of the current angle stored in register A
    Move,
    /// Multiply the value in Register(input) by `value` and store in Register(output)
    Mul {
        input: Register,
        output: Register,
        value: u16,
    },
    /// Loop to back to the address a number of times
    Loop { addr: u16, times: u16 },
    /// Set register
    StoreRegister(Register, u16),
    /// Increment the register by an amount
    IncrementRegisterBy(Register, u16),
    /// Set register1 to the value of register2
    StoreRegReg(Register, Register),
    /// Decrement register
    DecrementRegister(Register),
    /// Increment register
    IncrementRegister(Register),
    /// Jump if register is non-zero
    JumpIfNonZeroRegister(Register, u16),
}

impl Instruction {
    pub fn parse_next(buffer: &[u8]) -> (usize, Instruction) {
        // We always read at least one byte
        let i = RefCell::new(1);

        let be_u16 = || {
            let x = u16::from_be_bytes([buffer[*i.borrow() + 1], buffer[*i.borrow()]]);
            *i.borrow_mut() += 2;
            x
        };

        let register = || {
            let x = match buffer[*i.borrow()] {
                0 => Register::A,
                1 => Register::B,
                2 => Register::C,
                3 => Register::D,
                4 => Register::E,
                5 => Register::F,
                6 => Register::G,
                7 => Register::H,
                _ => panic!("invalid register: {}", buffer[*i.borrow()]),
            };
            *i.borrow_mut() += 1;
            x
        };

        let instruction = match buffer[0] {
            0x44 => Instruction::Draw,
            0x4d => Instruction::Move,
            0x4c => Instruction::Loop {
                addr: be_u16(),
                times: be_u16(),
            },
            0x53 => Instruction::StoreRegister(register(), be_u16()),
            0x49 => Instruction::IncrementRegister(register()),
            0x69 => Instruction::IncrementRegisterBy(register(), be_u16()),
            0x64 => Instruction::DecrementRegister(register()),
            0x4a => Instruction::JumpIfNonZeroRegister(register(), be_u16()),
            0x48 => Instruction::Halt,
            0x6d => Instruction::Mul {
                input: register(),
                output: register(),
                value: be_u16(),
            },
            0x32 => Instruction::StoreRegReg(register(), register()),
            invalid => panic!("invalid instruction: {}", invalid),
        };

        (i.into_inner(), instruction)
    }
}

pub struct Vm<'a> {
    pc: usize,
    draw: bool,
    x: f64,
    y: f64,
    loops: HashMap<usize, u16>,
    program: &'a [Instruction],
    terminated: bool,
    registers: [u16; 8],
}

impl<'a> Vm<'a> {
    pub fn new(program: &'a [Instruction]) -> Self {
        Vm {
            pc: 0,
            draw: false,
            x: 0.0,
            y: 0.0,
            loops: HashMap::new(),
            program,
            terminated: false,
            registers: Default::default(),
        }
    }

    pub fn step(&mut self) -> Option<(isize, isize, u32)> {
        match self.program[self.pc] {
            Instruction::Draw => {
                self.draw = !self.draw;
                self.pc += 1;
            }
            Instruction::Move => {
                let angle = (self.registers[Register::A as usize] % 360) as f64;

                // Convert to radians
                let radians = angle * (PI / 180.0);

                self.x += radians.cos();
                self.y += radians.sin();

                self.pc += 1;
            }
            Instruction::Halt => self.terminated = true,
            Instruction::Loop { addr, times } => {
                // We've already run the loop once to get to the loop instruction
                let times = times - 1;

                let count = self.loops.entry(self.pc).or_insert(times);

                if *count == 0 {
                    // If we've reached the end of the loop, reset the loop counter and move on
                    self.loops.remove(&self.pc);
                    self.pc += 1;
                } else {
                    // Otherwise, decrement the loop counter and jump
                    *count -= 1;
                    self.pc = addr as usize;
                }

                return None;
            }
            Instruction::StoreRegister(register, value) => {
                self.registers[register as usize] = value;
                self.pc += 1;
            }
            Instruction::IncrementRegisterBy(register, value) => {
                self.registers[register as usize] += value;
                self.pc += 1;
            }
            Instruction::StoreRegReg(r1, r2) => {
                self.registers[r1 as usize] = self.registers[r2 as usize];
                self.pc += 1;
            }
            Instruction::IncrementRegister(register) => {
                self.registers[register as usize] += 1;
                self.pc += 1;
            }
            Instruction::DecrementRegister(register) => {
                self.registers[register as usize] -= 1;
                self.pc += 1;
            }
            Instruction::JumpIfNonZeroRegister(register, addr) => {
                if self.registers[register as usize] == 0 {
                    self.pc += 1;
                } else {
                    self.pc = addr as usize;
                }

                return None;
            }
            Instruction::Mul {
                input,
                output,
                value,
            } => {
                let v1 = self.registers[input as usize];
                self.registers[output as usize] = v1 * value;
                self.pc += 1;
            }
        }

        if self.draw {
            Some((self.x as isize, self.y as isize, 0xffffff))
        } else {
            None
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
}
