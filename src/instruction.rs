use std::convert::TryFrom;

use crate::Opcode;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UintRegister {
    /// Angle register
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
    /// X position
    X = 5,
    /// Y position
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Address(u16);

impl From<Address> for usize {
    fn from(addr: Address) -> Self {
        addr.0 as _
    }
}

/// Instructions are ...
///
/// ```text
/// ```
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Toggle if we're drawing or not
    ///
    /// ```text
    /// DRW
    /// ```
    Draw,
    /// End the program
    ///
    /// ```text
    /// HLT
    /// ```
    Halt,
    /// Update float registers `X` and `Y` to "move" in direction of the current angle stored in
    /// register `A`.
    ///
    /// ```text
    /// FWD
    /// ```
    Forward,
    /// Set the register `Rx` to the product of `Rx` and either the immediate value `n`, or the
    /// value in the register `Ry`.
    ///
    /// ```text
    /// MUL Rx n
    /// MUL Rx Ry
    /// ```
    Multiply(Register, Value),
    /// Set the register `Rx` to the quotient of `Rx` and either the immediate value `n`, or the
    /// value in the register `Ry`.
    ///
    /// ```text
    /// DIV Rx n
    /// DIV Rx Ry
    /// ```
    Divide(Register, Value),
    /// Set the register `Rx` to the sum of `Rx` and either the immediate value `n`, or the
    /// value in the register `Ry`.
    ///
    /// ```text
    /// ADD Rx n
    /// ADD Rx Ry
    /// ```
    Add(Register, Value),
    /// Set the register `Rx` to the difference of `Rx` and either the immediate value `n`, or the
    /// value in the register `Ry`.
    ///
    /// ```text
    /// SUB Rx n
    /// SUB Rx Ry
    /// ```
    Sub(Register, Value),
    /// Set the register `Rx` to either the immediate value `n`, or the value in the register `Ry`.
    ///
    /// ```text
    /// STO Rx n
    /// STO Rx Ry
    /// ```
    Store(Register, Value),
    /// Decrement register `Rx`.
    ///
    /// ```text
    /// DEC Rx
    /// ```
    Decrement(Register),
    /// Increment register `Rx`.
    ///
    /// ```text
    /// INC Rx
    /// ```
    Increment(Register),
    /// Jump to `label:` if the register `Rx` is non-zero.
    ///
    /// ```text
    /// JNZ Rx label:
    /// ```
    JumpIfNonZero(Register, Address),
    /// Jump to `label:` if the register `Rx` is equal to the immediate value `n`, or the value in
    /// the register `Ry`.
    ///
    /// ```text
    /// JEQ Rx n label:
    /// JEQ Rx Ry label:
    /// ```
    JumpIfEqual(Register, Value, Address),
    /// Jump to `label:` if the register `Rx` is not equal to the immediate value `n`, or the value
    /// in the register `Ry`.
    ///
    /// ```text
    /// JNE Rx n label:
    /// JNE Rx Ry label:
    /// ```
    JumpIfNotEqual(Register, Value, Address),
    /// Jump to `label:` if the register `Rx` is greater than the immediate value `n`, or the value
    /// in the register `Ry`.
    ///
    /// ```text
    /// JGT Rx n label:
    /// JGT Rx Ry label:
    /// ```
    JumpIfGreaterThan(Register, Value, Address),
    /// Jump to `label:` if the register `Rx` is less than the immediate value `n`, or the value in
    /// the register `Ry`.
    ///
    /// ```text
    /// JLT Rx n label:
    /// JLT Rx Ry label:
    /// ```
    JumpIfLessThan(Register, Value, Address),
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

    fn address(&mut self) -> Address {
        Address(self.read_u16())
    }
}

fn parse_next_instruction(buffer: &[u8]) -> (usize, Instruction) {
    let mut p = Program { buffer, cursor: 0 };

    let opcode = p.read_u8();

    // If the high bit is set the second operand should be treated as a register
    let high_bit_set = opcode & 0b1000_0000 != 0;

    let opcode = Opcode::try_from(opcode & 0b0111_1111)
        .unwrap_or_else(|_| panic!("invalid instruction: {:#04x}", opcode));

    use Instruction::*;
    use Opcode::*;

    let instruction = match opcode {
        DRW => Draw,
        FWD => Forward,
        HLT => Halt,
        INC => Increment(p.register()),
        DEC => Decrement(p.register()),
        STO => Store(p.register(), p.value(high_bit_set)),
        ADD => Add(p.register(), p.value(high_bit_set)),
        SUB => Sub(p.register(), p.value(high_bit_set)),
        MUL => Multiply(p.register(), p.value(high_bit_set)),
        DIV => Divide(p.register(), p.value(high_bit_set)),
        JNZ => JumpIfNonZero(p.register(), p.address()),
        JEQ => JumpIfEqual(p.register(), p.value(high_bit_set), p.address()),
        JNE => JumpIfNotEqual(p.register(), p.value(high_bit_set), p.address()),
        JGT => JumpIfGreaterThan(p.register(), p.value(high_bit_set), p.address()),
        JLT => JumpIfLessThan(p.register(), p.value(high_bit_set), p.address()),
    };

    (p.cursor, instruction)
}

fn parse_header(buffer: &[u8]) -> (usize, u8, u16, u16) {
    let version = buffer[0];

    let width = u16::from_le_bytes([buffer[1], buffer[2]]);
    let height = u16::from_le_bytes([buffer[3], buffer[4]]);

    // We've read 5 bytes
    let read = 5;

    (read, version, width, height)
}

pub fn decode(buffer: &[u8]) -> (u16, u16, Vec<Instruction>) {
    let (mut i, version, width, height) = parse_header(&buffer);

    assert_eq!(0x01, version);

    let mut program = Vec::new();
    loop {
        if i >= buffer.len() {
            break;
        }

        let (bytes, instruction) = parse_next_instruction(&buffer[i..]);
        i += bytes;

        program.push(instruction);
    }

    (width, height, program)
}
