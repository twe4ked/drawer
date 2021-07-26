use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Toggle if we're drawing or not
    Draw,
    /// End the program
    Halt,
    /// Move in direction of the current angle
    Move,
    /// Set the angle
    SetAngle(u16),
    /// Increment the angle
    IncAngle(u16),
    /// Loop to back to the address a number of times
    Loop { addr: u16, times: u16 },
    /// Set register
    StoreRegister(Register, u16),
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
            0x61 => Instruction::SetAngle(be_u16()),
            0x41 => Instruction::IncAngle(be_u16()),
            0x53 => Instruction::StoreRegister(register(), be_u16()),
            0x49 => Instruction::IncrementRegister(register()),
            0x64 => Instruction::DecrementRegister(register()),
            0x4a => Instruction::JumpIfNonZeroRegister(register(), be_u16()),
            0x48 => Instruction::Halt,
            invalid => panic!("invalid instruction: {}", invalid),
        };

        (i.into_inner(), instruction)
    }
}

pub struct Vm<'a> {
    pc: usize,
    angle: u16,
    draw: bool,
    x: f64,
    y: f64,
    loops: HashMap<usize, u16>,
    program: &'a [Instruction],
    terminated: bool,
    registers: [u16; 4],
}

impl<'a> Vm<'a> {
    pub fn new(program: &'a [Instruction]) -> Self {
        Vm {
            pc: 0,
            angle: 0,
            draw: false,
            x: 0.0,
            y: 0.0,
            loops: HashMap::new(),
            program,
            terminated: false,
            registers: [0, 0, 0, 0],
        }
    }

    pub fn step(&mut self) -> Option<(isize, isize, u32)> {
        match self.program[self.pc] {
            Instruction::Draw => {
                self.draw = !self.draw;
                self.pc += 1;
            }
            Instruction::Move => {
                // Convert to radians
                let radians = (self.angle as f64) * (PI / 180.0);

                self.x += radians.cos();
                self.y += radians.sin();

                self.pc += 1;
            }
            Instruction::Halt => self.terminated = true,
            Instruction::SetAngle(angle) => {
                self.angle = angle;
                self.pc += 1;
            }
            Instruction::IncAngle(angle) => {
                self.angle = (self.angle + angle) % 360;
                self.pc += 1;
            }
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
