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
        let be_u16 = |i: usize| u16::from_be_bytes([buffer[i + 2], buffer[i + 1]]);
        let register = |i: usize| match buffer[i + 1] {
            0 => Register::A,
            1 => Register::B,
            2 => Register::C,
            3 => Register::D,
            _ => panic!("invalid register: {}", buffer[i + 1]),
        };

        match buffer[0] {
            0x44 => (1, Instruction::Draw),
            0x4d => (1, Instruction::Move),
            0x4c => {
                let addr = be_u16(0);
                let times = be_u16(2);
                (5, Instruction::Loop { addr, times })
            }
            0x61 => (3, Instruction::SetAngle(be_u16(0))),
            0x41 => (3, Instruction::IncAngle(be_u16(0))),
            0x53 => {
                let reg = register(0);
                let addr = be_u16(1);
                (4, Instruction::StoreRegister(reg, addr))
            }
            0x49 => (2, Instruction::IncrementRegister(register(0))),
            0x64 => (2, Instruction::DecrementRegister(register(0))),
            0x4a => {
                let reg = register(0);
                let addr = be_u16(1);
                (4, Instruction::JumpIfNonZeroRegister(reg, addr))
            }
            0x48 => (1, Instruction::Halt),
            invalid => panic!("invalid instruction: {}", invalid),
        }
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
