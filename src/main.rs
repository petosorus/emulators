extern crate gameboy;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
mod disassembler;
mod display;

fn main() {
    let flags = gameboy::Flags {
        z: false,
        s: false,
        p: false,
        cy: false,
        ac: false,
        ime: false,
    };

    let mut state = gameboy::State8080 {
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
            gameboy::Memory {
                memory: vec![0; (u16::max_value() as usize) + 1],
            }
        },
        flags: flags,
        int_enable: false,
    };

    // let filename = "cpudiag.bin";
    // let filename = "gb-test-roms/cpu_instrs/cpu_instrs.gb";
    let filename = "gb-test-roms/mem_timing/mem_timing.gb";
    let filecontent = fs::read(filename).expect("Something wrong");

    for (index, data) in filecontent.iter().enumerate() {
        state.memory.memory[index] = *data;
    }

    let mut video = display::memory_to_video(&state.memory.memory[0x2400..0x4000]);

    let mut window = display::display_window(&mut video);

    let clock = Duration::from_nanos(500);

    while (state.pc as usize) < state.memory.memory.len() {
        // print!("sp: ${:04x} - ", state.sp);
        // print!("pc: ${:04x} - ", state.pc);
        disassembler::disassemble_op(&state.memory.memory, state.pc);
        gameboy::emulate_op(&mut state);
        state.pc = state.pc.wrapping_add(1);

        // println!("${:04x}", state.get(0x3000));
        // print!("hl {:04x}\t", state.get_hl());
        // println!("{:02x}, {}", state.a, state.flags.z);

        // Game boy VRAM
        // video = display::memory_to_video(&state.memory.memory[0x8000..0x9FFF]);

        // Just for fun
        video = display::memory_to_video(&state.memory.memory[0x0000..state.memory.memory.len()]);
        display::update_screen(&mut window, &mut video);

        // check for interrupts
        if state.int_enable {
            gameboy::handle_interrupt();
        }

        sleep(clock);
    }
}
