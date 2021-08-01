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

impl Value {
    fn into_f64(self, vm: &Vm) -> f64 {
        match self {
            Value::Uint(v) => v as f64,
            Value::Float(v) => v,
            Value::Register(r) => match r {
                Register::UintRegister(r) => vm.uint_registers[r as usize] as f64,
                Register::FloatRegister(r) => vm.float_registers[r as usize],
            },
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
            Instruction::Add(register, value) => {
                let value_1 = self.get_register(register);
                self.set_register(register, value_1 + value.into_f64(&self));
            }
            Instruction::Sub(register, value) => {
                let value_1 = self.get_register(register);
                self.set_register(register, value_1 - value.into_f64(&self));
            }
            Instruction::Store(register, value) => {
                self.set_register(register, value.into_f64(&self))
            }
            Instruction::Increment(register) => {
                self.set_register(register, self.get_register(register) + 1.0);
            }
            Instruction::Decrement(register) => {
                self.set_register(register, self.get_register(register) - 1.0);
            }
            Instruction::JumpIfNonZero(register, addr) => {
                // a != b
                if (self.get_register(register) - 0.0).abs() > f64::EPSILON {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::JumpIfEqual(register, value, addr) => {
                // a == b
                if (self.get_register(register) - value.into_f64(&self)).abs() < f64::EPSILON {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::JumpIfNotEqual(register, value, addr) => {
                // a != b
                if (self.get_register(register) - value.into_f64(&self)).abs() > f64::EPSILON {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::JumpIfGreaterThan(register, value, addr) => {
                if self.get_register(register) > value.into_f64(&self) {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::JumpIfLessThan(register, value, addr) => {
                if self.get_register(register) < value.into_f64(&self) {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::Multiply(register, value) => {
                let value_1 = self.get_register(register);
                self.set_register(register, value_1 * value.into_f64(&self));
            }
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

    fn set_register(&mut self, register: Register, value: f64) {
        match register {
            Register::UintRegister(r) => self.uint_registers[r as usize] = value as u16,
            Register::FloatRegister(r) => self.float_registers[r as usize] = value,
        }
    }

    fn get_register(&self, register: Register) -> f64 {
        match register {
            Register::UintRegister(r) => self.uint_registers[r as usize] as f64,
            Register::FloatRegister(r) => self.float_registers[r as usize],
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
}
