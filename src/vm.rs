use std::convert::TryFrom;
use std::f64::consts::PI;

use crate::Opcode;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UintRegister {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FloatRegister {
    S = 0,
    T = 1,
    U = 2,
    V = 3,
    W = 4,
    X = 5,
    Y = 6,
    Z = 7,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Register {
    UintRegister(UintRegister),
    FloatRegister(FloatRegister),
}

impl Register {
    fn from_u8(r: u8) -> Self {
        match r {
            0x0 => Register::UintRegister(UintRegister::A),
            0x1 => Register::UintRegister(UintRegister::B),
            0x2 => Register::UintRegister(UintRegister::C),
            0x3 => Register::UintRegister(UintRegister::D),
            0x4 => Register::UintRegister(UintRegister::E),
            0x5 => Register::UintRegister(UintRegister::F),
            0x6 => Register::UintRegister(UintRegister::G),
            0x7 => Register::UintRegister(UintRegister::H),
            0x8 => Register::FloatRegister(FloatRegister::S),
            0x9 => Register::FloatRegister(FloatRegister::T),
            0xa => Register::FloatRegister(FloatRegister::U),
            0xb => Register::FloatRegister(FloatRegister::V),
            0xc => Register::FloatRegister(FloatRegister::W),
            0xd => Register::FloatRegister(FloatRegister::X),
            0xe => Register::FloatRegister(FloatRegister::Y),
            0xf => Register::FloatRegister(FloatRegister::Z),
            _ => panic!("invalid register: {}", r),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Value {
    Uint(u16),
    Float(f64),
    Register(Register),
}

impl Value {
    fn unwrap_or_else<F>(&self, f: F) -> u16
    where
        F: Fn(usize) -> u16,
    {
        match self {
            Value::Uint(v) => *v,
            Value::Register(r) => match r {
                Register::UintRegister(r) => f(*r as usize),
                _ => todo!("unhandled: {:?}", self),
            },
            _ => todo!("unhandled: {:?}", self),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Toggle if we're drawing or not
    Draw,
    /// End the program
    Halt,
    /// Move in direction of the current angle stored in register A
    Move,
    /// Set the register `Rx` to the product of `Rx` and the immediate value `n`.
    ///
    /// ```text
    /// STO Rx n
    /// ```
    Multiply(Register, Value),
    /// Increment the register by an amount
    Add(Register, Value),
    /// Set the register `Rx` to either the immediate value `n`, or the value in the register `Ry`.
    ///
    /// ```text
    /// STO Rx n
    /// STO Rx Ry
    /// ```
    Store(Register, Value),
    /// Decrement register
    Decrement(Register),
    /// Increment register
    Increment(Register),
    /// Jump if register is non-zero
    JumpIfNonZero(Register, u16),
    /// TODO
    JumpIfGreaterThan(Register, Value, u16),
}

struct Program<'a> {
    buffer: &'a [u8],
    cursor: usize,
}

impl<'a> Program<'a> {
    fn read_u8(&mut self) -> u8 {
        let item = self.buffer[self.cursor];
        self.cursor += 1;
        item
    }

    fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.read_u8(), self.read_u8()])
    }

    fn register(&mut self) -> Register {
        Register::from_u8(self.read_u8())
    }
}

impl Instruction {
    pub fn parse_next(buffer: &[u8]) -> (usize, Instruction) {
        let mut p = Program { buffer, cursor: 0 };

        let opcode = p.read_u8();

        let high_bit_set = opcode & 0b1000_0000 != 0;

        let opcode = Opcode::try_from(opcode & 0b0111_1111)
            .unwrap_or_else(|_| panic!("invalid instruction: {:#04x}", opcode));

        let instruction = match opcode {
            Opcode::DRW => Instruction::Draw,
            Opcode::MOV => Instruction::Move,
            Opcode::STO => Instruction::Store(
                p.register(),
                if high_bit_set {
                    Value::Register(p.register())
                } else {
                    Value::Uint(p.read_u16())
                },
            ),
            Opcode::INC => Instruction::Increment(p.register()),
            Opcode::ADD => Instruction::Add(
                p.register(),
                if high_bit_set {
                    Value::Register(p.register())
                } else {
                    Value::Uint(p.read_u16())
                },
            ),
            Opcode::DEC => Instruction::Decrement(p.register()),
            Opcode::JNZ => Instruction::JumpIfNonZero(p.register(), p.read_u16()),
            Opcode::JGT => Instruction::JumpIfGreaterThan(
                p.register(),
                if high_bit_set {
                    todo!()
                } else {
                    Value::Uint(p.read_u16())
                },
                p.read_u16(),
            ),
            Opcode::HLT => Instruction::Halt,
            Opcode::MUL => Instruction::Multiply(
                p.register(),
                if high_bit_set {
                    todo!("MUL Rx Ry")
                } else {
                    Value::Uint(p.read_u16())
                },
            ),
        };

        (p.cursor, instruction)
    }
}

pub struct Vm<'a> {
    pc: usize,
    draw: bool,
    x: f64,
    y: f64,
    program: &'a [Instruction],
    terminated: bool,
    registers: [u16; 8],
    float_registers: [f64; 8],
}

impl<'a> Vm<'a> {
    pub fn new(program: &'a [Instruction]) -> Self {
        Vm {
            pc: 0,
            draw: false,
            x: 0.0,
            y: 0.0,
            program,
            terminated: false,
            registers: Default::default(),
            float_registers: Default::default(),
        }
    }

    pub fn step(&mut self) -> Option<(isize, isize, u32)> {
        let mut pixel = None;

        match self.program[self.pc] {
            Instruction::Draw => {
                self.draw = !self.draw;
            }
            Instruction::Move => {
                let angle = (self.registers[UintRegister::A as usize] % 360) as f64;

                // Convert to radians
                let radians = angle * (PI / 180.0);

                self.x += radians.cos();
                self.y += radians.sin();

                if self.draw {
                    pixel = Some((self.x as isize, self.y as isize, 0xffffff));
                }
            }
            Instruction::Halt => self.terminated = true,
            Instruction::Add(register, value) => match register {
                Register::UintRegister(register) => {
                    let value = self.unwrap_uint_value(value);
                    let (value, overflowed) =
                        self.registers[register as usize].overflowing_add(value);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    self.float_registers[register as usize] += self.unwrap_float_value(value);
                }
            },
            Instruction::Store(r1, value) => match r1 {
                Register::UintRegister(r1) => {
                    self.registers[r1 as usize] = self.unwrap_uint_value(value);
                }
                Register::FloatRegister(r1) => {
                    self.float_registers[r1 as usize] = self.unwrap_float_value(value);
                }
            },
            Instruction::Increment(register) => match register {
                Register::UintRegister(register) => {
                    let (value, overflowed) = self.registers[register as usize].overflowing_add(1);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    self.float_registers[register as usize] += 1.0;
                }
            },
            Instruction::Decrement(register) => match register {
                Register::UintRegister(register) => {
                    let (value, overflowed) = self.registers[register as usize].overflowing_sub(1);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    self.float_registers[register as usize] -= 1.0;
                }
            },
            Instruction::JumpIfNonZero(register, addr) => match register {
                Register::UintRegister(register) => {
                    if self.registers[register as usize] != 0 {
                        self.pc = addr as usize;
                        return None;
                    }
                }
                Register::FloatRegister(register) => {
                    if self.float_registers[register as usize] != 0.0 {
                        self.pc = addr as usize;
                        return None;
                    }
                }
            },
            Instruction::JumpIfGreaterThan(register, value, addr) => match register {
                Register::UintRegister(register) => {
                    let value = self.unwrap_uint_value(value);
                    if self.registers[register as usize] > value {
                        self.pc = addr as usize;
                        return None;
                    }
                }
                Register::FloatRegister(register) => {
                    let value = self.unwrap_float_value(value);
                    if self.float_registers[register as usize] > value {
                        self.pc = addr as usize;
                        return None;
                    }
                }
            },
            Instruction::Multiply(register, value) => match register {
                Register::UintRegister(register) => {
                    let value = value.unwrap_or_else(|_| todo!());
                    let (value, overflowed) =
                        self.registers[register as usize].overflowing_mul(value);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.registers[register as usize] = value;
                }
                Register::FloatRegister(_r1) => {
                    todo!()
                }
            },
        }

        self.pc += 1;
        pixel
    }

    fn unwrap_uint_value(&self, value: Value) -> u16 {
        match value {
            Value::Uint(v) => v,
            Value::Float(v) => v as u16,
            Value::Register(r) => match r {
                Register::UintRegister(r) => self.registers[r as usize],
                Register::FloatRegister(r) => self.float_registers[r as usize] as u16,
            },
        }
    }

    fn unwrap_float_value(&self, value: Value) -> f64 {
        match value {
            Value::Uint(v) => v as f64,
            Value::Float(v) => v,
            Value::Register(r) => match r {
                Register::UintRegister(r) => self.registers[r as usize] as f64,
                Register::FloatRegister(r) => self.float_registers[r as usize],
            },
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
}
