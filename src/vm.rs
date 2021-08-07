use crate::instruction::{FloatRegister, Instruction, Register, UintRegister, Value};

#[derive(Default)]
pub struct Vm {
    pc: usize,
    draw: bool,
    terminated: bool,
    uint_registers: [u16; 8],
    float_registers: [f64; 8],
}

impl Vm {
    pub fn step(&mut self, program: &[Instruction]) -> Option<(isize, isize, u32)> {
        match program[self.pc] {
            Instruction::Draw => {
                self.draw = !self.draw;
            }
            Instruction::Forward => {
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
                    self.pc = addr.into();
                    return None;
                }
            }
            Instruction::JumpIfEqual(register, value, addr) => {
                if self.check_conditional(
                    register,
                    value,
                    |a, b| (a - b).abs() < f64::EPSILON, // a == b
                ) {
                    self.pc = addr.into();
                    return None;
                }
            }
            Instruction::JumpIfNotEqual(register, value, addr) => {
                if self.check_conditional(
                    register,
                    value,
                    |a, b| (a - b).abs() > f64::EPSILON, // a != b
                ) {
                    self.pc = addr.into();
                    return None;
                }
            }
            Instruction::JumpIfGreaterThan(register, value, addr) => {
                if self.check_conditional(register, value, |a, b| a > b) {
                    self.pc = addr.into();
                    return None;
                }
            }
            Instruction::JumpIfLessThan(register, value, addr) => {
                if self.check_conditional(register, value, |a, b| a < b) {
                    self.pc = addr.into();
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
