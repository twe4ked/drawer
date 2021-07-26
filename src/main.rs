use minifb::{Scale, Window, WindowOptions};

mod buffer;
mod vm;

use buffer::Buffer;
use vm::{Instruction, Register, Vm};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

fn read_stdin() -> Vec<u8> {
    use std::io::{stdin, Read};
    let mut buffer = Vec::new();
    stdin().read_to_end(&mut buffer).unwrap();
    buffer
}

fn decode(buffer: &[u8]) -> Vec<Instruction> {
    let mut program = Vec::new();
    let mut i = 0;
    loop {
        if i >= buffer.len() {
            break;
        }

        let be_u16 = |i| u16::from_be_bytes([buffer[i + 2], buffer[i + 1]]);
        let register = |i| match buffer[i + 1] {
            0 => Register::A,
            1 => Register::B,
            2 => Register::C,
            3 => Register::D,
            _ => panic!("invalid register: {}", buffer[i + 1]),
        };

        match buffer[i] {
            0x44 => program.push(Instruction::Draw),
            0x4d => program.push(Instruction::Move),
            0x4c => {
                let addr = be_u16(i);
                i += 2;
                let times = be_u16(i);
                i += 2;
                program.push(Instruction::Loop { addr, times });
            }
            0x61 => {
                program.push(Instruction::SetAngle(be_u16(i)));
                i += 2;
            }
            0x41 => {
                program.push(Instruction::IncAngle(be_u16(i)));
                i += 2;
            }
            0x53 => {
                let reg = register(i);
                i += 1;
                let addr = be_u16(i);
                i += 2;
                program.push(Instruction::StoreRegister(reg, addr));
            }
            0x49 => {
                program.push(Instruction::IncrementRegister(register(i)));
                i += 1;
            }
            0x64 => {
                program.push(Instruction::DecrementRegister(register(i)));
                i += 1;
            }
            0x4a => {
                let reg = register(i);
                i += 1;
                let addr = be_u16(i);
                i += 2;
                program.push(Instruction::JumpIfNonZeroRegister(reg, addr));
            }
            0x48 => program.push(Instruction::Halt),
            invalid => panic!("invalid instruction: {}", invalid),
        }

        i += 1;
    }
    program
}

fn main() {
    let input = read_stdin();
    let program = decode(&input);

    let mut window = Window::new(
        "Drawer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
    .expect("unable to initialize window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut vm = Vm::new(&program);
    let mut buffer = Buffer::new(WIDTH, HEIGHT);

    while window.is_open() {
        if !vm.is_terminated() {
            if let Some((x, y, color)) = vm.step() {
                // We want 0,0 to be in the center of the buffer
                let x = (WIDTH / 2) as isize + x;
                let y = (HEIGHT / 2) as isize + y;

                use std::convert::TryFrom;

                buffer.set_pixel(
                    usize::try_from(x).expect("invalid x coordinate"),
                    usize::try_from(y).expect("invalid y coordinate"),
                    color,
                );
            }
        }

        window
            .update_with_buffer(&buffer.buffer(), WIDTH, HEIGHT)
            .expect("unable to update buffer");
    }
}
