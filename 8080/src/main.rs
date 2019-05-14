use std::fs;
mod disassembler;

struct Flags {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
    pad: u8,
}

struct Memory {
    memory: Vec<u8>
}

impl Memory {
    fn get(&self, index: usize) -> u8 {
        self.memory[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.memory[index]
    }

    fn set(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }
}

struct State8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: Memory,
    flags: Flags,
    int_enable: u8,
}

impl State8080 {
    fn get(&self, index: u16) -> u8 {
        self.memory.get(index as usize)
    }

    fn get_mut(&mut self, index: u16) -> &mut u8 {
        self.memory.get_mut(index as usize)
    }

    fn set(&mut self, index: u16, value: u8) {
        self.memory.set(index as usize, value);
    }

    fn get_hl(self) -> u16 {
        get16bit(self.l, self.h)
    }
}

fn get16bit(lower_byte: u8, higher_byte: u8) -> u16 {
    let result: u16 = (higher_byte as u16) << 8 | ( lower_byte as u16);
    result
}

fn get_lower8(value: u16) -> u8 {
    (value as u8)
}

fn get_higher8(value: u16) -> u8 {
    (value >> 8) as u8
}

fn zero(value: u8, flags: &mut Flags) {
    if value == 0 {
        flags.z = true;
    } else {
        flags.z = false;
    }
}

fn sign(value: u8, flags: &mut Flags) {
    if value | 128 == 0 {
        flags.s = false;
    } else {
        flags.s = true;
    }
}

fn parity(value: u8, flags: &mut Flags) {
    if value % 2 == 0 {
        flags.p = true;
    } else {
        flags.p = false;
    }
}


fn handle_condition_codes(value: u8, flags: &mut Flags) {
    zero(value, flags);
    sign(value, flags);
    parity(value, flags);
}

fn inr(pc: &mut u16, register: &mut u8, flags: &mut Flags) {
    *register += 1;
    *pc += 1;

    handle_condition_codes(*register, flags);
}

fn dcr(pc: &mut u16, register: &mut u8, flags: &mut Flags) {
    *register -= 1;
    *pc += 1;

    handle_condition_codes(*register, flags);
}

fn dcx(pc: &mut u16, higher_register: &mut u8, lower_register: &mut u8) {
    let mut registerPair = get16bit(*higher_register, *lower_register);
    registerPair -= 1;
    *lower_register = get_lower8(registerPair);
    *higher_register = get_higher8(registerPair);
    *pc += 1;
}

fn inx(pc: &mut u16, higher_register: &mut u8, lower_register: &mut u8) {
    let mut registerPair = get16bit(*higher_register, *lower_register);
    registerPair += 1;
    *lower_register = get_lower8(registerPair);
    *higher_register = get_higher8(registerPair);
    *pc += 1;
}

fn inx_sp(pc: &mut u16, sp: &mut u16) {
    *sp += 1;
    *pc += 1;
}

fn dcx_sp(pc: &mut u16, sp: &mut u16) {
    *sp -= 1;
    *pc += 1;
}

fn call(pc: &mut u16, sp: &mut u16, adr_low: u8, adr_high: u8) {
    *sp = *pc;
    *pc = get16bit(adr_low, adr_high);
}

fn mov(target: &mut u8, source: u8) {
    *target = source;
}

fn emulate8080_op(state: &mut State8080) {
    let code: u8 = state.get(state.pc);

    match code {
        0x00 => {}
        0x01 => {
            state.pc += 1;
            state.c = state.get(state.pc);
            state.pc += 1;
            state.b = state.get(state.pc);
        }
        0x02 => unimplemented!(),
        0x03 => unimplemented!(),
        0x04 => {
            inr(&mut state.pc, &mut state.b, &mut state.flags);
        }
        0x05 => {
            dcr(&mut state.pc, &mut state.b, &mut state.flags);
        },
        0x06 => unimplemented!(),
        0x07 => unimplemented!(),
        0x08 => unimplemented!(),
        0x09 => unimplemented!(),
        0x0a => unimplemented!(),
        0x0b => {
            dcx(&mut state.pc, &mut state.b, &mut state.c);
        }
        0x0c => {
            inr(&mut state.pc, &mut state.c, &mut state.flags);
        }
        0x0d => {
            dcr(&mut state.pc, &mut state.c, &mut state.flags);
        },
        0x0e => unimplemented!(),
        0x0f => unimplemented!(),
        0x10 => unimplemented!(),
        0x11 => unimplemented!(),
        0x12 => unimplemented!(),
        0x13 => unimplemented!(),
        0x14 => {
            inr(&mut state.pc, &mut state.d, &mut state.flags);
        }
        0x15 => {
            dcr(&mut state.pc, &mut state.d, &mut state.flags);
        },
        0x16 => unimplemented!(),
        0x17 => unimplemented!(),
        0x18 => unimplemented!(),
        0x19 => unimplemented!(),
        0x1a => unimplemented!(),
        0x1b => {
            dcx(&mut state.pc, &mut state.d, &mut state.e);
        }
        0x1c => {
            inr(&mut state.pc, &mut state.e, &mut state.flags);
        }
        0x1d => {
            dcr(&mut state.pc, &mut state.e, &mut state.flags);
        },
        0x1e => unimplemented!(),
        0x1f => unimplemented!(),
        0x20 => {},
        0x21 => unimplemented!(),
        0x22 => unimplemented!(),
        0x23 => {
            inx(&mut state.pc, &mut state.h, &mut state.l);
        },
        0x24 => {
            inr(&mut state.pc, &mut state.h, &mut state.flags);
        }
        0x25 => {
            dcr(&mut state.pc, &mut state.h, &mut state.flags);
        },
        0x26 => unimplemented!(),
        0x27 => unimplemented!(),
        0x28 => unimplemented!(),
        0x29 => unimplemented!(),
        0x2a => unimplemented!(),
        0x2b => {
            dcx(&mut state.pc, &mut state.h, &mut state.l);
        }
        0x2c => {
            inr(&mut state.pc, &mut state.l, &mut state.flags);
        }
        0x2d => {
            dcr(&mut state.pc, &mut state.l, &mut state.flags);
        },
        0x2e => unimplemented!(),
        0x2f => unimplemented!(),
        0x30 => unimplemented!(),
        0x31 => unimplemented!(),
        0x32 => {
            state.pc += 1;
            let lower_byte = state.get(state.pc);
            state.pc += 1;
            let higher_byte = state.get(state.pc);
            let adressed_memory = state.memory.get_mut(get16bit(lower_byte, higher_byte) as usize);
            mov(adressed_memory, state.a)
        }
        0x33 => unimplemented!(),
        0x34 => {
            let hl = get16bit(state.l, state.h);
            let register = state.memory.get_mut(hl as usize);
            inr(&mut state.pc, register, &mut state.flags);
        }
        0x35 => {
            let hl = get16bit(state.l, state.h);
            let register = state.memory.get_mut(hl as usize);
            dcr(&mut state.pc, register, &mut state.flags);
        },
        0x36 => unimplemented!(),
        0x37 => unimplemented!(),
        0x38 => unimplemented!(),
        0x39 => unimplemented!(),
        0x3a => unimplemented!(),
        0x3b => {
            dcx_sp(&mut state.pc, &mut state.sp);
        }
        0x3c => {
            inr(&mut state.pc, &mut state.a, &mut state.flags);
        }
        0x3d => unimplemented!(),
        0x3e => unimplemented!(),
        0x3f => unimplemented!(),
        0x40 => unimplemented!(),
        0x41 => unimplemented!(),
        0x42 => unimplemented!(),
        0x43 => unimplemented!(),
        0x44 => unimplemented!(),
        0x45 => unimplemented!(),
        0x46 => unimplemented!(),
        0x47 => unimplemented!(),
        0x48 => unimplemented!(),
        0x49 => unimplemented!(),
        0x4a => unimplemented!(),
        0x4b => unimplemented!(),
        0x4c => unimplemented!(),
        0x4d => unimplemented!(),
        0x4e => unimplemented!(),
        0x4f => unimplemented!(),
        0x50 => unimplemented!(),
        0x51 => unimplemented!(),
        0x52 => unimplemented!(),
        0x53 => unimplemented!(),
        0x54 => unimplemented!(),
        0x55 => unimplemented!(),
        0x56 => unimplemented!(),
        0x57 => unimplemented!(),
        0x58 => unimplemented!(),
        0x59 => unimplemented!(),
        0x5a => unimplemented!(),
        0x5b => unimplemented!(),
        0x5c => unimplemented!(),
        0x5d => unimplemented!(),
        0x5e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            mov(&mut state.e, value)
        }
        0x5f => unimplemented!(),
        0x60 => unimplemented!(),
        0x61 => {
            mov(&mut state.h, state.c)
        }
        0x62 => unimplemented!(),
        0x63 => unimplemented!(),
        0x64 => unimplemented!(),
        0x65 => unimplemented!(),
        0x66 => unimplemented!(),
        0x67 => unimplemented!(),
        0x68 => unimplemented!(),
        0x69 => unimplemented!(),
        0x6a => unimplemented!(),
        0x6b => unimplemented!(),
        0x6c => unimplemented!(),
        0x6d => unimplemented!(),
        0x6e => unimplemented!(),
        0x6f => mov(&mut state.l, state.a),
        0x70 => unimplemented!(),
        0x71 => unimplemented!(),
        0x72 => unimplemented!(),
        0x73 => unimplemented!(),
        0x74 => unimplemented!(),
        0x75 => unimplemented!(),
        0x76 => unimplemented!(),
        0x77 => {
            let a = state.a.clone();
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            mov(register, a)
        },
        0x78 => unimplemented!(),
        0x79 => unimplemented!(),
        0x7a => unimplemented!(),
        0x7b => unimplemented!(),
        0x7c => unimplemented!(),
        0x7d => unimplemented!(),
        0x7e => unimplemented!(),
        0x7f => unimplemented!(),
        0x80 => unimplemented!(),
        0x81 => unimplemented!(),
        0x82 => unimplemented!(),
        0x83 => unimplemented!(),
        0x84 => unimplemented!(),
        0x85 => unimplemented!(),
        0x86 => unimplemented!(),
        0x87 => unimplemented!(),
        0x88 => unimplemented!(),
        0x89 => unimplemented!(),
        0x8a => unimplemented!(),
        0x8b => unimplemented!(),
        0x8c => unimplemented!(),
        0x8d => unimplemented!(),
        0x8e => unimplemented!(),
        0x8f => unimplemented!(),
        0x90 => unimplemented!(),
        0x91 => unimplemented!(),
        0x92 => unimplemented!(),
        0x93 => unimplemented!(),
        0x94 => unimplemented!(),
        0x95 => unimplemented!(),
        0x96 => unimplemented!(),
        0x97 => unimplemented!(),
        0x98 => unimplemented!(),
        0x99 => unimplemented!(),
        0x9a => unimplemented!(),
        0x9b => unimplemented!(),
        0x9c => unimplemented!(),
        0x9d => unimplemented!(),
        0x9e => unimplemented!(),
        0x9f => unimplemented!(),
        0xa0 => unimplemented!(),
        0xa1 => unimplemented!(),
        0xa2 => unimplemented!(),
        0xa3 => unimplemented!(),
        0xa4 => unimplemented!(),
        0xa5 => unimplemented!(),
        0xa6 => unimplemented!(),
        0xa7 => unimplemented!(),
        0xa8 => unimplemented!(),
        0xa9 => unimplemented!(),
        0xaa => unimplemented!(),
        0xab => unimplemented!(),
        0xac => unimplemented!(),
        0xad => unimplemented!(),
        0xae => unimplemented!(),
        0xaf => unimplemented!(),
        0xb0 => unimplemented!(),
        0xb1 => unimplemented!(),
        0xb2 => unimplemented!(),
        0xb3 => unimplemented!(),
        0xb4 => unimplemented!(),
        0xb5 => unimplemented!(),
        0xb6 => unimplemented!(),
        0xb7 => unimplemented!(),
        0xb8 => unimplemented!(),
        0xb9 => unimplemented!(),
        0xba => unimplemented!(),
        0xbb => unimplemented!(),
        0xbc => unimplemented!(),
        0xbd => unimplemented!(),
        0xbe => unimplemented!(),
        0xbf => unimplemented!(),
        0xc0 => unimplemented!(),
        0xc1 => unimplemented!(),
        0xc2 => unimplemented!(),
        0xc3 => {
            state.pc += 1;
            let lower_byte = state.get(state.pc);
            state.pc += 1;
            let higher_byte = state.get(state.pc);
            state.pc = get16bit(lower_byte, higher_byte);
        }
        0xc4 => unimplemented!(),
        0xc5 => unimplemented!(),
        0xc6 => unimplemented!(),
        0xc7 => unimplemented!(),
        0xc8 => unimplemented!(),
        0xc9 => unimplemented!(),
        0xca => unimplemented!(),
        0xcb => unimplemented!(),
        0xcc => unimplemented!(),
        0xcd => {
            state.pc += 1;
            let lower_byte = state.get(state.pc);
            state.pc += 1;
            let higher_byte = state.get(state.pc);
            call(&mut state.pc, &mut state.sp, lower_byte, higher_byte);
        }
        0xce => unimplemented!(),
        0xcf => unimplemented!(),
        0xd0 => unimplemented!(),
        0xd1 => unimplemented!(),
        0xd2 => unimplemented!(),
        0xd3 => unimplemented!(),
        0xd4 => unimplemented!(),
        0xd5 => unimplemented!(),
        0xd6 => unimplemented!(),
        0xd7 => unimplemented!(),
        0xd8 => unimplemented!(),
        0xd9 => unimplemented!(),
        0xda => unimplemented!(),
        0xdb => unimplemented!(),
        0xdc => unimplemented!(),
        0xdd => unimplemented!(),
        0xde => unimplemented!(),
        0xdf => unimplemented!(),
        0xe0 => unimplemented!(),
        0xe1 => unimplemented!(),
        0xe2 => unimplemented!(),
        0xe3 => unimplemented!(),
        0xe4 => unimplemented!(),
        0xe5 => unimplemented!(),
        0xe6 => unimplemented!(),
        0xe7 => unimplemented!(),
        0xe8 => unimplemented!(),
        0xe9 => unimplemented!(),
        0xea => unimplemented!(),
        0xeb => unimplemented!(),
        0xec => unimplemented!(),
        0xed => unimplemented!(),
        0xee => unimplemented!(),
        0xef => unimplemented!(),
        0xf0 => unimplemented!(),
        0xf1 => unimplemented!(),
        0xf2 => unimplemented!(),
        0xf3 => unimplemented!(),
        0xf4 => unimplemented!(),
        0xf5 => unimplemented!(),
        0xf6 => unimplemented!(),
        0xf7 => unimplemented!(),
        0xf8 => unimplemented!(),
        0xf9 => unimplemented!(),
        0xfa => unimplemented!(),
        0xfb => unimplemented!(),
        0xfc => unimplemented!(),
        0xfd => unimplemented!(),
        0xfe => unimplemented!(),
        0xff => unimplemented!()
    }
}

fn main() {

    let flags = Flags {
        z: false,
        s: false,
        p: false,
        cy: false,
        ac: false,
        pad: 0,
    };

    let mut state = State8080 {
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
            Memory {
                memory: vec![0; u16::max_value() as usize]
            }
        },
        flags: flags,
        int_enable: 0,
    };

    let filename = "invaders.rom";
    let filecontent = fs::read(filename).expect("Something wrong");

    for (index, data) in filecontent.iter().enumerate() {
        state.memory.memory[index] = *data;
    }
    
    while (state.pc as usize) < state.memory.memory.len() {
        disassembler::disassemble8080op(&state.memory.memory, state.pc);
        emulate8080_op(&mut state);
        state.pc += 1;
    }

    println!("b {}, c {}, pc {}", state.b, state.c, state.pc);
}
