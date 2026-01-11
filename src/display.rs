use minifb::{Scale, Window, WindowOptions};

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 224;

pub fn display_window(buffer: &mut Vec<u32>) -> Window {
    let mut window = Window::new(
        "Buffer test",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    window.update_with_buffer(&buffer).unwrap();
    window
}

pub fn update_screen(window: &mut Window, buffer: &mut Vec<u32>) {
    window.update_with_buffer(&buffer).unwrap();
}

pub fn memory_to_video(memory: &[u8]) -> Vec<u32> {
    let mut video_buffer = vec![0; WIDTH * HEIGHT];
    let mut pixel_wrap = 0;
    let mut video_index = 0;
    let mut pixel: u32 = 0;

    for memory_index in 0..memory.len() {
        pixel = pixel | (memory[memory_index] as u32);

        if pixel_wrap >= 2 {
            video_buffer[video_index] = pixel;
            pixel = 0;
            pixel_wrap = 0;
            video_index += 1;
        } else {
            pixel = pixel << 8;
            pixel_wrap += 1;
        }
    }

    video_buffer
}
