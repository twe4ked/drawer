use minifb::{Scale, Window, WindowOptions};

use std::io::{stdin, Read};
use std::sync::mpsc::channel;
use std::thread;

use drawer::buffer::Buffer;
use drawer::vm::Vm;

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

fn main() {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();

    let (tx, rx) = channel();
    let worker = thread::spawn(move || {
        let mut vm = Vm::new(&input);
        while !vm.is_terminated() {
            if let Some(pixel) = vm.step() {
                tx.send(pixel).unwrap();
            }
        }
        eprintln!("worker finished");
    });

    let mut buffer = Buffer::new(WIDTH, HEIGHT);

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
