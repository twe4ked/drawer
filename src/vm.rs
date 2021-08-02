use std::convert::TryFrom;

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
    /// TODO
    Divide(Register, Value),
    /// Increment the register by an amount
    Add(Register, Value),
    /// Decrement the register by an amount
    Sub(Register, Value),
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
    JumpIfEqual(Register, Value, u16),
    /// TODO
    JumpIfNotEqual(Register, Value, u16),
    /// TODO
    JumpIfGreaterThan(Register, Value, u16),
    /// TODO
    JumpIfLessThan(Register, Value, u16),
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

    fn value(&mut self, is_register: bool) -> Value {
        if is_register {
            Value::Register(self.register())
        } else {
            Value::Uint(self.read_u16())
        }
    }
}

impl Instruction {
    pub fn parse_next(buffer: &[u8]) -> (usize, Instruction) {
        let mut p = Program { buffer, cursor: 0 };

        let opcode = p.read_u8();

        let high_bit_set = opcode & 0b1000_0000 != 0;

        let opcode = Opcode::try_from(opcode & 0b0111_1111)
            .unwrap_or_else(|_| panic!("invalid instruction: {:#04x}", opcode));

        use Instruction::*;
        use Opcode::*;

        let instruction = match opcode {
            DRW => Draw,
            MOV => Move,
            STO => Store(p.register(), p.value(high_bit_set)),
            INC => Increment(p.register()),
            ADD => Add(p.register(), p.value(high_bit_set)),
            SUB => Sub(p.register(), p.value(high_bit_set)),
            DEC => Decrement(p.register()),
            JNZ => JumpIfNonZero(p.register(), p.read_u16()),
            JEQ => JumpIfEqual(p.register(), p.value(high_bit_set), p.read_u16()),
            JNE => JumpIfNotEqual(p.register(), p.value(high_bit_set), p.read_u16()),
            JGT => JumpIfGreaterThan(p.register(), p.value(high_bit_set), p.read_u16()),
            JLT => JumpIfLessThan(p.register(), p.value(high_bit_set), p.read_u16()),
            HLT => Halt,
            MUL => Multiply(p.register(), p.value(high_bit_set)),
            DIV => Divide(p.register(), p.value(high_bit_set)),
        };

        (p.cursor, instruction)
    }
}

pub struct Vm<'a> {
    pc: usize,
    draw: bool,
    program: &'a [Instruction],
    terminated: bool,
    uint_registers: [u16; 8],
    float_registers: [f64; 8],
}

impl<'a> Vm<'a> {
    pub fn new(program: &'a [Instruction]) -> Self {
        Vm {
            pc: 0,
            draw: false,
            program,
            terminated: false,
            uint_registers: Default::default(),
            float_registers: Default::default(),
        }
    }

    pub fn step(&mut self) -> Option<(isize, isize, u32)> {
        match self.program[self.pc] {
            Instruction::Draw => {
                self.draw = !self.draw;
            }
            Instruction::Move => {
                let angle = (self.uint_registers[UintRegister::A as usize] % 360) as f64;
                let radians = angle.to_radians();
                self.float_registers[FloatRegister::X as usize] += radians.cos();
                self.float_registers[FloatRegister::Y as usize] += radians.sin();
            }
            Instruction::Halt => self.terminated = true,
            Instruction::Add(register, value) => match register {
                Register::UintRegister(register) => {
                    let value = self.unwrap_uint_value(value);
                    let (value, overflowed) =
                        self.uint_registers[register as usize].overflowing_add(value);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.uint_registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    self.float_registers[register as usize] += self.unwrap_float_value(value);
                }
            },
            Instruction::Sub(register, value) => match register {
                Register::UintRegister(register) => {
                    let value = self.unwrap_uint_value(value);
                    let (value, overflowed) =
                        self.uint_registers[register as usize].overflowing_sub(value);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.uint_registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    self.float_registers[register as usize] -= self.unwrap_float_value(value);
                }
            },
            Instruction::Store(r1, value) => match r1 {
                Register::UintRegister(r1) => {
                    self.uint_registers[r1 as usize] = self.unwrap_uint_value(value);
                }
                Register::FloatRegister(r1) => {
                    self.float_registers[r1 as usize] = self.unwrap_float_value(value);
                }
            },
            Instruction::Increment(register) => match register {
                Register::UintRegister(register) => {
                    let (value, overflowed) =
                        self.uint_registers[register as usize].overflowing_add(1);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.uint_registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    self.float_registers[register as usize] += 1.0;
                }
            },
            Instruction::Decrement(register) => match register {
                Register::UintRegister(register) => {
                    let (value, overflowed) =
                        self.uint_registers[register as usize].overflowing_sub(1);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.uint_registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    self.float_registers[register as usize] -= 1.0;
                }
            },
            Instruction::JumpIfNonZero(register, addr) => {
                if self.check_conditional(
                    register,
                    Value::Uint(0),
                    |a: f64, b: f64| (a - b).abs() > f64::EPSILON, // a != b
                ) {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::JumpIfEqual(register, value, addr) => {
                if self.check_conditional(
                    register,
                    value,
                    |a, b| (a - b).abs() < f64::EPSILON, // a == b
                ) {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::JumpIfNotEqual(register, value, addr) => {
                if self.check_conditional(
                    register,
                    value,
                    |a, b| (a - b).abs() > f64::EPSILON, // a != b
                ) {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::JumpIfGreaterThan(register, value, addr) => {
                if self.check_conditional(register, value, |a, b| a > b) {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::JumpIfLessThan(register, value, addr) => {
                if self.check_conditional(register, value, |a, b| a < b) {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::Multiply(register, value) => match register {
                Register::UintRegister(register) => {
                    let value = self.unwrap_uint_value(value);
                    let (value, overflowed) =
                        self.uint_registers[register as usize].overflowing_mul(value);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.uint_registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    let value = self.unwrap_float_value(value);
                    self.float_registers[register as usize] *= value;
                }
            },
            Instruction::Divide(register, value) => match register {
                Register::UintRegister(register) => {
                    let value = self.unwrap_uint_value(value);
                    let (value, overflowed) =
                        self.uint_registers[register as usize].overflowing_div(value);
                    if overflowed {
                        eprintln!("warning: {:?} overflowed", register);
                    }
                    self.uint_registers[register as usize] = value;
                }
                Register::FloatRegister(register) => {
                    let value = self.unwrap_float_value(value);
                    self.float_registers[register as usize] /= value;
                }
            },
        }

        self.pc += 1;

        if self.draw {
            Some((
                self.float_registers[FloatRegister::X as usize] as isize,
                self.float_registers[FloatRegister::Y as usize] as isize,
                0xffffff,
            ))
        } else {
            None
        }
    }

    fn check_conditional<F>(&self, register: Register, value: Value, f: F) -> bool
    where
        F: Fn(f64, f64) -> bool,
    {
        match register {
            Register::UintRegister(r) => {
                let value = self.unwrap_uint_value(value);
                // NOTE: Not ideal to be converting u16 to f64 before we run the predicate function
                // but otherwise we can't re-use the same function.
                f(self.uint_registers[r as usize] as f64, value as f64)
            }
            Register::FloatRegister(r) => {
                let value = self.unwrap_float_value(value);
                f(self.float_registers[r as usize], value)
            }
        }
    }

    fn unwrap_uint_value(&self, value: Value) -> u16 {
        match value {
            Value::Uint(v) => v,
            Value::Float(v) => v as u16,
            Value::Register(r) => match r {
                Register::UintRegister(r) => self.uint_registers[r as usize],
                Register::FloatRegister(r) => self.float_registers[r as usize] as u16,
            },
        }
    }

    fn unwrap_float_value(&self, value: Value) -> f64 {
        match value {
            Value::Uint(v) => v as f64,
            Value::Float(v) => v,
            Value::Register(r) => match r {
                Register::UintRegister(r) => self.uint_registers[r as usize] as f64,
                Register::FloatRegister(r) => self.float_registers[r as usize],
            },
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
}
