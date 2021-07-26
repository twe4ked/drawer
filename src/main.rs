use minifb::{Scale, Window, WindowOptions};

mod buffer;
mod vm;

use buffer::Buffer;
use vm::{Instruction, Vm};

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

        let (bytes, instruction) = Instruction::parse_next(&buffer[i..]);
        i += bytes;

        program.push(instruction);
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
