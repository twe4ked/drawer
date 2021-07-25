use std::f64::consts::PI;

#[derive(Debug)]
pub enum Instruction {
    Draw,
    Halt,
    Move(u16),
    Angle(u16),
}

pub struct Vm<'a> {
    pc: usize,
    angle: u16,
    draw: bool,
    x: f64,
    y: f64,
    move_counter: Option<u16>,
    program: &'a [Instruction],
    terminated: bool,
}

impl<'a> Vm<'a> {
    pub fn new(program: &'a [Instruction]) -> Self {
        Vm {
            pc: 0,
            angle: 0,
            draw: false,
            x: 0.0,
            y: 0.0,
            move_counter: None,
            program,
            terminated: false,
        }
    }

    pub fn step(&mut self) -> Option<(isize, isize, u32)> {
        match self.program[self.pc] {
            Instruction::Draw => {
                self.draw = !self.draw;
                self.pc += 1;
            }
            Instruction::Move(i) => {
                if let Some(ref mut c) = self.move_counter {
                    if *c > 0 {
                        *c -= 1;
                    } else {
                        self.move_counter = None;
                        self.pc += 1;
                    }
                } else {
                    self.move_counter = Some(i);
                }

                // Convert to radians
                let radians = (self.angle as f64) * (PI / 180.0);

                self.x += radians.cos();
                self.y += radians.sin();
            }
            Instruction::Halt => self.terminated = true,
            Instruction::Angle(angle) => {
                self.angle = angle;
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
