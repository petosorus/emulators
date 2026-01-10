use std::{thread::sleep, time::Duration};

use minifb::{Window, Key, Scale, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

fn main() {
    let mut buffer: Vec<u32> = vec![0x11111111; WIDTH * HEIGHT];

    let mut window = match Window::new("Buffer test",
                                        WIDTH,
                                        HEIGHT,
                                        WindowOptions {
                                            resize: false,
                                            scale: Scale::X2,
                                            ..WindowOptions::default()
                                        }) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        },
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[(y * WIDTH) + x] += 0xFFFF;
                
            }
        }
        sleep(Duration::from_secs(1));

        window.update_with_buffer(&buffer).unwrap();
    }
}