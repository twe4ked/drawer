use minifb::{Scale, Window, WindowOptions};
use std::sync::mpsc::channel;
use std::thread;

use drawer::buffer::Buffer;
use drawer::vm::{Instruction, Vm};

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

    let (tx, rx) = channel();
    let worker = thread::spawn(move || {
        let mut vm = Vm::new(&program);
        while !vm.is_terminated() {
            if let Some((x, y, color)) = vm.step() {
                tx.send((x, y, color)).unwrap();
            }
        }

        eprintln!("worker finished");
    });

    let mut buffer = Buffer::new(WIDTH, HEIGHT);

    while window.is_open() {
        for (x, y, color) in rx.try_iter() {
            // We want 0,0 to be in the center of the buffer
            let x = (WIDTH as isize / 2) + x;
            let y = (HEIGHT as isize / 2) + y;

            use std::convert::TryFrom;

            let x = usize::try_from(x);
            if x.is_err() {
                eprintln!("invalid x coordinate");
                break;
            }

            let y = usize::try_from(y);
            if y.is_err() {
                eprintln!("invalid y coordinate");
                break;
            }

            buffer.set_pixel(x.unwrap(), y.unwrap(), color);
        }

        window
            .update_with_buffer(&buffer.buffer(), WIDTH, HEIGHT)
            .expect("unable to update buffer");
    }

    worker.join().unwrap()
}
