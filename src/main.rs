use minifb::{Scale, Window, WindowOptions};

use std::io::{stdin, Read};
use std::sync::mpsc::channel;
use std::thread;

use drawer::buffer::Buffer;
use drawer::instruction::decode;
use drawer::vm::Vm;

enum Event {
    Pixel((isize, isize, u32)),
    Terminated,
}

fn main() {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();

    let (width, height, program) = decode(&input);

    let mut vm = Vm::default();

    let width = width as usize;
    let height = height as usize;

    let (tx, rx) = channel();
    let worker = thread::spawn(move || {
        while !vm.is_terminated() {
            if let Some(pixel) = vm.step(&program) {
                tx.send(Event::Pixel(pixel)).unwrap();
            }
        }
        tx.send(Event::Terminated).unwrap();
        eprintln!("worker finished");
    });

    let mut buffer = Buffer::new(width, height);

    let mut window = Window::new(
        "Drawer",
        width,
        height,
        WindowOptions {
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
    .expect("unable to initialize window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let quit_on_vm_term = std::env::var("QUIT_ON_VM_TERM")
        .map(|_| true)
        .unwrap_or(false);
    let mut terminated = false;

    while window.is_open() {
        if quit_on_vm_term && terminated {
            break;
        }

        if !terminated {
            for event in rx.try_iter() {
                match event {
                    Event::Pixel((x, y, color)) => {
                        // We want 0,0 to be in the center of the buffer
                        let x = (width as isize / 2) + x;
                        let y = (height as isize / 2) + y;

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
                    Event::Terminated => {
                        terminated = true;
                        break;
                    }
                }
            }
        }

        window
            .update_with_buffer(&buffer.buffer(), width, height)
            .expect("unable to update buffer");
    }

    worker.join().unwrap()
}
