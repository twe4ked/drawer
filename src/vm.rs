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

impl Register {
    fn from_u8(r: u8) -> Self {
        match r {
            0 => Register::A,
            1 => Register::B,
            2 => Register::C,
            3 => Register::D,
            4 => Register::E,
            5 => Register::F,
            6 => Register::G,
            7 => Register::H,
            _ => panic!("invalid register: {}", r),
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
    /// Multiply the value in R1 by `value` and store in R2
    MultiplyImmediate(Register, Register, u16),
    /// Set register
    StoreImmediate(Register, u16),
    /// Increment the register by an amount
    AddImmediate(Register, u16),
    /// Set register1 to the value of register2
    Store(Register, Register),
    /// Decrement register
    Decrement(Register),
    /// Increment register
    Increment(Register),
    /// Jump if register is non-zero
    JumpIfNonZero(Register, u16),
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
        let b1 = self.read_u8();
        let b2 = self.read_u8();
        u16::from_be_bytes([b2, b1])
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

        let instruction = match opcode & 0b0111_1111 {
            0x01 => Instruction::Draw,
            0x02 => Instruction::Move,
            0x03 => {
                if high_bit_set {
                    Instruction::Store(p.register(), p.register())
                } else {
                    Instruction::StoreImmediate(p.register(), p.read_u16())
                }
            }
            0x04 => Instruction::Increment(p.register()),
            0x05 => {
                if high_bit_set {
                    todo!("ADD Rx Ry")
                } else {
                    Instruction::AddImmediate(p.register(), p.read_u16())
                }
            }
            0x06 => Instruction::Decrement(p.register()),
            0x07 => Instruction::JumpIfNonZero(p.register(), p.read_u16()),
            0x08 => Instruction::Halt,
            0x09 => {
                if high_bit_set {
                    todo!("MUL Rx Ry Rz")
                } else {
                    Instruction::MultiplyImmediate(p.register(), p.register(), p.read_u16())
                }
            }
            invalid => panic!("invalid instruction: {:#04x}", invalid),
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
        }
    }

    pub fn step(&mut self) -> Option<(isize, isize, u32)> {
        let mut pixel = None;

        match self.program[self.pc] {
            Instruction::Draw => {
                self.draw = !self.draw;
            }
            Instruction::Move => {
                let angle = (self.registers[Register::A as usize] % 360) as f64;

                // Convert to radians
                let radians = angle * (PI / 180.0);

                self.x += radians.cos();
                self.y += radians.sin();

                if self.draw {
                    pixel = Some((self.x as isize, self.y as isize, 0xffffff));
                }
            }
            Instruction::Halt => self.terminated = true,
            Instruction::StoreImmediate(register, value) => {
                self.registers[register as usize] = value;
            }
            Instruction::AddImmediate(register, value) => {
                self.registers[register as usize] += value;
            }
            Instruction::Store(r1, r2) => {
                self.registers[r1 as usize] = self.registers[r2 as usize];
            }
            Instruction::Increment(register) => {
                self.registers[register as usize] += 1;
            }
            Instruction::Decrement(register) => {
                self.registers[register as usize] -= 1;
            }
            Instruction::JumpIfNonZero(register, addr) => {
                if self.registers[register as usize] != 0 {
                    self.pc = addr as usize;
                    return None;
                }
            }
            Instruction::MultiplyImmediate(r1, r2, value) => {
                self.registers[r2 as usize] = self.registers[r1 as usize] * value;
            }
        }

        self.pc += 1;
        pixel
    }

    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
}
