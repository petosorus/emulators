extern crate em8080;
use std::fs;
mod display;
mod disassembler;

fn main() {
    let flags = em8080::Flags {
        z: false,
        s: false,
        p: false,
        cy: false,
        ac: false
    };

    let mut state = em8080::State8080 {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        sp: 0,
        pc: 0,
        memory: {
            em8080::Memory {
                memory: vec![0; (u16::max_value() as usize) + 1],
            }
        },
        flags: flags,
        int_enable: false,
    };

    let filename = "invaders.rom";
    let filecontent = fs::read(filename).expect("Something wrong");

    for (index, data) in filecontent.iter().enumerate() {
        state.memory.memory[index] = *data;
    }

    let mut video = display::memory_to_video(&state.memory.memory[0x2400..0x4000]);

    let mut window = display::display_window(&mut video);

    while (state.pc as usize) < state.memory.memory.len() {
        // print!("sp: ${:04x} - ", state.sp);
        // print!("pc: ${:04x} - ", state.pc);
        disassembler::disassemble8080op(&state.memory.memory, state.pc);
        em8080::emulate8080_op(&mut state);
        state.pc += 1;
        // println!("${:04x}", state.get(0x3000));
        // print!("hl {:04x}\t", state.get_hl());
        // println!("{:02x}, {}", state.a, state.flags.z);

        video = display::memory_to_video(&state.memory.memory[0x0000..state.memory.memory.len()]);
        display::update_screen(&mut window, &mut video);
    }
}