use std::{num::Wrapping, ops::Add};

pub struct Flags {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
    pub ime: bool,
}

impl Flags {
    pub fn save(&self) -> u8 {
        let mut saved: u8 = 0;
        if self.s {
            saved |= 0x80;
        }
        if self.z {
            saved |= 0x40;
        }
        if self.ac {
            saved |= 0x10;
        }
        if self.p {
            saved |= 0x04;
        }
        if self.cy {
            saved |= 0x01;
        }
        saved
    }

    pub fn load(&mut self, saved: u8) {
        if (saved & 0x80) != 0 {
            self.s = true;
        }
        if (saved & 0x40) != 0 {
            self.z = true;
        }
        if (saved & 0x10) != 0 {
            self.ac = true;
        }
        if (saved & 0x04) != 0 {
            self.p = true;
        }
        if (saved & 0x01) != 0 {
            self.cy = true;
        }
    }
}

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn get(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.memory[index]
    }

    pub fn set(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }
}

pub struct State8080 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub memory: Memory,
    pub flags: Flags,
    pub int_enable: bool,
}

impl State8080 {
    fn get(&self, index: u16) -> u8 {
        self.memory.get(index as usize)
    }

    fn get_mut(&mut self, index: u16) -> &mut u8 {
        self.memory.get_mut(index as usize)
    }

    pub fn set(&mut self, index: u16, value: u8) {
        self.memory.set(index as usize, value);
    }

    fn get_hl(&self) -> u16 {
        get16bit(self.l, self.h)
    }
}

fn get16bit(lower_byte: u8, higher_byte: u8) -> u16 {
    let result: u16 = (higher_byte as u16) << 8 | (lower_byte as u16);
    result
}

fn get_lower8(value: u16) -> u8 {
    value as u8
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
    if value & 128 == 0 {
        flags.s = false;
    } else {
        flags.s = true;
    }
}

fn parity(value: u8, flags: &mut Flags) {
    let mut bits: u8 = 0;
    for i in 0..8 {
        bits += (value >> i) & 1;
    }
    flags.p = (bits & 1) == 0;
}

fn auxiliary_carry(value: u8, flags: &mut Flags) {
    // TODO
}

fn handle_condition_codes(value: u8, flags: &mut Flags) {
    zero(value, flags);
    sign(value, flags);
    parity(value, flags);
    auxiliary_carry(value, flags);
}

fn inr(register: &mut u8, flags: &mut Flags) {
    *register = register.wrapping_add(1);

    handle_condition_codes(*register, flags);
}

fn dcr(register: &mut u8, flags: &mut Flags) {
    *register = register.wrapping_sub(1);

    handle_condition_codes(*register, flags);
}

fn dcx(higher_register: &mut u8, lower_register: &mut u8) {
    let mut register_pair = get16bit(*lower_register, *higher_register);
    register_pair -= 1;
    *lower_register = get_lower8(register_pair);
    *higher_register = get_higher8(register_pair);
}

fn inx(higher_register: &mut u8, lower_register: &mut u8) {
    let mut register_pair = get16bit(*lower_register, *higher_register);
    // print!("rp {:04x}, ", register_pair);
    register_pair += 1;
    *lower_register = get_lower8(register_pair);
    *higher_register = get_higher8(register_pair);
}

fn dcx_sp(sp: &mut u16) {
    *sp -= 1;
}

fn call(pc: &mut u16, sp: &mut u16, memory: &mut Memory, adr_low: u8, adr_high: u8) {
    // Store on stack

    let next_pc = *pc + 3;
    *sp = sp.wrapping_sub(1);
    *(memory.get_mut(*sp as usize)) = get_higher8(next_pc);
    *sp = sp.wrapping_sub(1);
    *(memory.get_mut(*sp as usize)) = get_lower8(next_pc);

    // Move to function
    *pc = get16bit(adr_low, adr_high);
}

fn ret(pc: &mut u16, sp: &mut u16, memory: &Memory) {
    // Get from stack
    let adr_low = memory.get(*sp as usize);
    *sp += 1;
    let adr_high = memory.get(*sp as usize);
    *sp += 1;
    let adr = get16bit(adr_low, adr_high);

    // Go back to call instruction
    *pc = adr;
}

fn lxi_sp(sp: &mut u16, low: u8, high: u8) {
    *sp = get16bit(low, high);
}

fn load(target: &mut u8, source: u8) {
    *target = source;
}

fn load_registers(register: &mut u8, memory: &Memory, adr_high: u8, adr_low: u8) {
    let adr = get16bit(adr_low, adr_high);
    load_adr(register, memory, adr);
}

fn load_adr(register: &mut u8, memory: &Memory, adr: u16) {
    load(register,memory.get(adr as usize));
}

fn add(target: &mut u8, source: u8, flags: &mut Flags) {
    let old_value = *target;
    *target = target.wrapping_add(source);

    handle_condition_codes(*target, flags);
    if *target < old_value {
        flags.cy = true;
    }
}

fn adc(target: &mut u8, source: u8, flags: &mut Flags) {
    *target = source;

    if flags.cy {
        *target += 1;
    }
    handle_condition_codes(*target, flags);
}

fn sub(target: &mut u8, source: u8, flags: &mut Flags) {
    let mut wrapped = Wrapping(*target);
    wrapped -= source;

    handle_condition_codes(*target, flags);
}

fn sbc(target: &mut u8, source: u8, flags: &mut Flags) {
    let mut wrapped = Wrapping(*target);
    wrapped -= source;

    if flags.cy {
        wrapped += 1;
    }
    handle_condition_codes(wrapped.0, flags);
}

fn ani(register: &mut u8, data: u8, flags: &mut Flags) {
    *register = *register & data;

    flags.cy = false;
    handle_condition_codes(*register, flags);
}

fn dad(target_left: &mut u8, target_right: &mut u8, source: u16, flags: &mut Flags) {
    let mut target = get16bit(*target_right, *target_left);
    let targetcheck = target.clone();

    target = target.wrapping_add(source);
    *target_left = get_higher8(target);
    *target_right = get_lower8(target);

    if target < targetcheck {
        flags.cy = true;
    }
}

fn xchg(state: &mut State8080) {
    let htemp = state.h;
    state.h = state.d;
    state.d = htemp;

    let ltemp = state.l;
    state.l = state.e;
    state.e = ltemp;
}

fn push(sp: &mut u16, memory: &mut Memory, left: u8, right: u8) {
    *memory.get_mut((*sp - 1) as usize) = left;
    *memory.get_mut((*sp - 2) as usize) = right;
    *sp -= 2;
}

fn pop(sp: &mut u16, memory: &Memory, left: &mut u8, right: &mut u8) {
    *right = memory.get((*sp) as usize);
    *left = memory.get((*sp + 1) as usize);

    memory.get((*sp) as usize);
    *sp += 2;
}

fn rar(register: &mut u8, flags: &mut Flags) {
    let carry = flags.cy;
    flags.cy = *register % 2 != 0;

    *register = *register >> 1;

    if carry {
        *register = *register | 0x80;
    } else {
        *register = *register & 0x7f;
    }
}

fn rr(register: &mut u8, flags: &mut Flags) {
    let lsb = *register & 0x01;
    *register = register.rotate_right(1);

    if lsb == 0 {
        flags.cy = false;
        *register = *register & 0x7F;
    } else {
        flags.cy = true;
        *register = *register | 0x80;
    }
}

fn rrc(register: &mut u8, flags: &mut Flags) {
    *register = register.rotate_right(1);

    handle_condition_codes(*register, flags);
}

fn rl(register: &mut u8, flags: &mut Flags) {
    let msb = *register & 0x80;
    *register = register.rotate_left(1);

    if msb == 0 {
        flags.cy = false;
        *register = *register & 0xFE;
    } else {
        flags.cy = true;
        *register = *register | 0x01;
    }
}

fn or(register: &mut u8, data: u8, flags: &mut Flags) {
    *register = *register | data;

    handle_condition_codes(*register, flags);
}

fn xor(register: &mut u8, data: u8, flags: &mut Flags) {
    *register = *register ^ data;

    handle_condition_codes(*register, flags);
}

fn cma(register: &mut u8) {
    *register = !*register;
}

fn ana(register: &mut u8, source: u8, flags: &mut Flags) {
    flags.cy = false;

    *register = *register & source;
    handle_condition_codes(*register, flags);
}

fn xra(register: &mut u8, source: u8, flags: &mut Flags) {
    flags.cy = false;

    *register = *register ^ source;
    handle_condition_codes(*register, flags);
}

fn cpi(register: u8, data: u8, flags: &mut Flags) {
    let result = register.wrapping_sub(data);
    if result > register {
        flags.cy = true;
    } else {
        flags.cy = false;
    }

    //println!("{}, {}, {}", register, data, result);

    handle_condition_codes(result, flags);
}

fn cmp(accumulator: u8, register: u8, flags: &mut Flags) {
    let result = accumulator.wrapping_sub(register);
    flags.cy = register > accumulator;
    handle_condition_codes(result, flags);
}

pub fn handle_interrupt() {}

pub fn emulate_op(state: &mut State8080) {
    let code: u8 = state.get(state.pc);

    match code {
        0x00 => {}
        0x01 => {
            state.c = state.get(state.pc + 1);
            state.b = state.get(state.pc + 2);
            state.pc = state.pc.wrapping_add(2);
        }
        0x02 => {}
        0x03 => {
            inx(&mut state.b, &mut state.c);
        }
        0x04 => {
            inr(&mut state.b, &mut state.flags);
        }
        0x05 => {
            dcr(&mut state.b, &mut state.flags);
        }
        0x06 => {
            let source = state.get(state.pc + 1);
            load(&mut state.b, source);
            state.pc = state.pc.wrapping_add(1);
        }
        0x07 => {
            rlc(&mut state.a, &mut state.flags);
        }
        0x08 => {}
        0x09 => {
            let bc = get16bit(state.b, state.c);
            dad(&mut state.h, &mut state.l, bc, &mut state.flags);
        }
        0x0a => {
            load_registers(&mut state.a, &state.memory, state.b, state.c);
        }
        0x0b => {
            dcx(&mut state.b, &mut state.c);
        }
        0x0c => {
            inr(&mut state.c, &mut state.flags);
        }
        0x0d => {
            dcr(&mut state.c, &mut state.flags);
        }
        0x0e => {
            let source = state.get(state.pc + 1);
            load(&mut state.c, source);
            state.pc = state.pc.wrapping_add(1);
        }
        0x0f => rrc(&mut state.a, &mut state.flags),
        0x10 => {}
        0x11 => {
            state.e = state.get(state.pc + 1);
            state.d = state.get(state.pc + 2);
            state.pc = state.pc.wrapping_add(2);
        }
        0x12 => {
            state.set(get16bit(state.e, state.d), state.a);
        }
        0x13 => inx(&mut state.d, &mut state.e),
        0x14 => {
            inr(&mut state.d, &mut state.flags);
        }
        0x15 => {
            dcr(&mut state.d, &mut state.flags);
        }
        0x16 => {
            let source = state.get(state.pc + 1);
            load(&mut state.d, source);
            state.pc = state.pc.wrapping_add(1);
        }
        0x17 => rl(&mut state.a, &mut state.flags),
        0x18 => {}
        0x19 => {
            let de = get16bit(state.d, state.e);
            dad(&mut state.h, &mut state.l, de, &mut state.flags);
        }
        0x1a => {
            load_registers(&mut state.a, &state.memory, state.d, state.e);
        }
        0x1b => {
            dcx(&mut state.d, &mut state.e);
        }
        0x1c => {
            inr(&mut state.e, &mut state.flags);
        }
        0x1d => {
            dcr(&mut state.e, &mut state.flags);
        }
        0x1e => {
            let source = state.get(state.pc + 1);
            load(&mut state.e, source);
            state.pc = state.pc.wrapping_add(1);
        }
        0x1f => {
            rar(&mut state.a, &mut state.flags);
        }
        0x20 => {}
        0x21 => {
            state.l = state.get(state.pc + 1);
            state.h = state.get(state.pc + 2);
            state.pc = state.pc.wrapping_add(2);
        }
        0x22 => {
            load(&mut state.get(state.pc), state.a);
            state.pc = state.pc.wrapping_add(1);
        }
        0x23 => {
            inx(&mut state.h, &mut state.l);
        }
        0x24 => {
            inr(&mut state.h, &mut state.flags);
        }
        0x25 => {
            dcr(&mut state.h, &mut state.flags);
        }
        0x26 => {
            let source = state.get(state.pc + 1);
            load(&mut state.h, source);
            state.pc = state.pc.wrapping_add(1);
        }
        0x27 => unimplemented!(),
        0x28 => {}
        0x29 => {
            let hl = state.get_hl();
            dad(&mut state.h, &mut state.l, hl, &mut state.flags);
        }
        0x2a => {
            let low = state.get(state.pc + 1);
            let high = state.get(state.pc + 2);
            let adr = get16bit(low, high);

            load_adr(&mut state.l, &state.memory, adr);
            load_adr(&mut state.h, &state.memory, adr + 1);
        }
        0x2b => {
            dcx(&mut state.h, &mut state.l);
        }
        0x2c => {
            inr(&mut state.l, &mut state.flags);
        }
        0x2d => {
            dcr(&mut state.l, &mut state.flags);
        }
        0x2e => {
            let source = state.get(state.pc + 1);
            load(&mut state.l, source);
            state.pc = state.pc.wrapping_add(1);
        }
        0x2f => {
            cma(&mut state.a);
        }
        0x30 => {}
        0x31 => {
            let low = state.get(state.pc + 1);
            let high = state.get(state.pc + 2);
            lxi_sp(&mut state.sp, low, high);
            state.pc = state.pc.wrapping_add(2);
        }
        0x32 => {
            let lower_byte = state.get(state.pc + 1);
            let higher_byte = state.get(state.pc + 2);
            let adressed_memory = state
                .memory
                .get_mut(get16bit(lower_byte, higher_byte) as usize);
            load(adressed_memory, state.a);
            state.pc = state.pc.wrapping_add(2);
        }
        0x33 => inr(&mut state.get(state.sp), &mut state.flags),
        0x34 => {
            let hl = get16bit(state.l, state.h);
            let register = state.memory.get_mut(hl as usize);
            inr(register, &mut state.flags);
        }
        0x35 => {
            let hl = get16bit(state.l, state.h);
            let register = state.memory.get_mut(hl as usize);
            dcr(register, &mut state.flags);
        }
        0x36 => {
            let source = state.get(state.pc + 1);
            let hl = state.get_hl();
            let register = state.get_mut(hl);
            load(register, source);
            state.pc = state.pc.wrapping_add(1);
        }
        0x37 => state.flags.cy = true,
        0x38 => {}
        0x39 => {
            dad(&mut state.h, &mut state.l, state.sp, &mut state.flags);
        }
        0x3a => {
            let low = state.get(state.pc + 1);
            let high = state.get(state.pc + 2);
            state.pc = state.pc.wrapping_add(2);
            load_registers(&mut state.a, &state.memory, high, low);
        }
        0x3b => {
            dcx_sp(&mut state.sp);
        }
        0x3c => {
            inr(&mut state.a, &mut state.flags);
        }
        0x3d => dcr(&mut state.a, &mut state.flags),
        0x3e => {
            state.a = state.get(state.pc + 1);
            state.pc = state.pc.wrapping_add(1);
        }
        0x3f => state.flags.cy = !state.flags.cy,
        0x40 => {
            let b = state.b;
            load(&mut state.b, b);
        }
        0x41 => {
            load(&mut state.b, state.c);
        }
        0x42 => {
            load(&mut state.b, state.d);
        }
        0x43 => {
            load(&mut state.b, state.e);
        }
        0x44 => {
            load(&mut state.b, state.h);
        }
        0x45 => {
            load(&mut state.b, state.l);
        }
        0x46 => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            load(&mut state.b, value)
        }
        0x47 => {
            load(&mut state.b, state.a);
        }
        0x48 => {
            load(&mut state.c, state.b);
        }
        0x49 => {
            let c = state.c;
            load(&mut state.c, c);
        }
        0x4a => {
            load(&mut state.c, state.d);
        }
        0x4b => {
            load(&mut state.c, state.e);
        }
        0x4c => {
            load(&mut state.c, state.h);
        }
        0x4d => {
            load(&mut state.c, state.l);
        }
        0x4e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            load(&mut state.c, value)
        }
        0x4f => {
            load(&mut state.c, state.a);
        }
        0x50 => {
            load(&mut state.d, state.b);
        }
        0x51 => {
            load(&mut state.d, state.c);
        }
        0x52 => {
            let d = state.d;
            load(&mut state.d, d);
        }
        0x53 => {
            load(&mut state.d, state.e);
        }
        0x54 => {
            load(&mut state.d, state.h);
        }
        0x55 => {
            load(&mut state.d, state.l);
        }
        0x56 => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            load(&mut state.d, value)
        }
        0x57 => {
            load(&mut state.d, state.a);
        }
        0x58 => {
            load(&mut state.e, state.b);
        }
        0x59 => {
            load(&mut state.e, state.c);
        }
        0x5a => {
            load(&mut state.e, state.d);
        }
        0x5b => {
            let e = state.e;
            load(&mut state.e, e);
        }
        0x5c => {
            load(&mut state.e, state.h);
        }
        0x5d => {
            load(&mut state.e, state.l);
        }
        0x5e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            load(&mut state.e, value)
        }
        0x5f => {
            load(&mut state.e, state.a);
        }
        0x60 => {
            load(&mut state.h, state.b);
        }
        0x61 => load(&mut state.h, state.c),
        0x62 => {
            load(&mut state.h, state.d);
        }
        0x63 => {
            load(&mut state.h, state.e);
        }
        0x64 => {
            let h = state.h;
            load(&mut state.h, h);
        }
        0x65 => {
            load(&mut state.h, state.l);
        }
        0x66 => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            load(&mut state.h, value);
        }
        0x67 => {
            load(&mut state.h, state.a);
        }
        0x68 => {
            load(&mut state.l, state.b);
        }
        0x69 => {
            load(&mut state.l, state.c);
        }
        0x6a => {
            load(&mut state.l, state.d);
        }
        0x6b => {
            load(&mut state.l, state.e);
        }
        0x6c => {
            load(&mut state.l, state.h);
        }
        0x6d => {
            let l = state.l;
            load(&mut state.l, l);
        }
        0x6e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            load(&mut state.d, value);
        }
        0x6f => load(&mut state.l, state.a),
        0x70 => {
            let b = state.b;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            load(register, b)
        }
        0x71 => {
            let c = state.c;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            load(register, c)
        }
        0x72 => {
            let d = state.d;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            load(register, d)
        }
        0x73 => {
            let e = state.e;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            load(register, e)
        }
        0x74 => {
            let h = state.h;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            load(register, h)
        }
        0x75 => {
            let l = state.l;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            load(register, l)
        }
        0x76 => unimplemented!(),
        0x77 => {
            let a = state.a;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            load(register, a)
        }
        0x78 => {
            load(&mut state.a, state.b);
        }
        0x79 => {
            load(&mut state.a, state.c);
        }
        0x7a => {
            load(&mut state.a, state.d);
        }
        0x7b => {
            load(&mut state.a, state.e);
        }
        0x7c => {
            load(&mut state.a, state.h);
        }
        0x7d => {
            load(&mut state.a, state.l);
        }
        0x7e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            load(&mut state.a, value);
        }
        0x7f => {
            let a = state.a;
            load(&mut state.a, a);
        }
        0x80 => {
            add(&mut state.a, state.b, &mut state.flags);
        }
        0x81 => {
            add(&mut state.a, state.c, &mut state.flags);
        }
        0x82 => {
            add(&mut state.a, state.d, &mut state.flags);
        }
        0x83 => {
            add(&mut state.a, state.e, &mut state.flags);
        }
        0x84 => {
            add(&mut state.a, state.h, &mut state.flags);
        }
        0x85 => {
            add(&mut state.a, state.l, &mut state.flags);
        }
        0x86 => {
            let hl = state.get_hl();
            let register = state.get(hl);
            add(&mut state.a, register, &mut state.flags);
        }
        0x87 => {
            let a = state.a;
            add(&mut state.a, a, &mut state.flags);
        }
        0x88 => {
            adc(&mut state.a, state.b, &mut state.flags);
        }
        0x89 => {
            adc(&mut state.a, state.c, &mut state.flags);
        }
        0x8a => {
            adc(&mut state.a, state.d, &mut state.flags);
        }
        0x8b => {
            adc(&mut state.a, state.e, &mut state.flags);
        }
        0x8c => {
            adc(&mut state.a, state.h, &mut state.flags);
        }
        0x8d => {
            adc(&mut state.a, state.l, &mut state.flags);
        }
        0x8e => {
            let hl = state.get(state.pc);
            adc(&mut state.a, hl, &mut state.flags);
        }
        0x8f => {
            let a = state.a;
            adc(&mut state.a, a, &mut state.flags);
        }
        0x90 => {
            sub(&mut state.a, state.b, &mut state.flags);
        }
        0x91 => {
            sub(&mut state.a, state.c, &mut state.flags);
        }
        0x92 => {
            sub(&mut state.a, state.d, &mut state.flags);
        }
        0x93 => {
            sub(&mut state.a, state.e, &mut state.flags);
        }
        0x94 => {
            sub(&mut state.a, state.h, &mut state.flags);
        }
        0x95 => {
            sub(&mut state.a, state.l, &mut state.flags);
        }
        0x96 => {
            let data = state.get(state.get_hl());
            sub(&mut state.a, data, &mut state.flags)
        }
        0x97 => {
            let a = state.a;
            sub(&mut state.a, a, &mut state.flags);
        }
        0x98 => {
            sbc(&mut state.a, state.b, &mut state.flags);
        }
        0x99 => {
            sbc(&mut state.a, state.c, &mut state.flags);
        }
        0x9a => {
            sbc(&mut state.a, state.d, &mut state.flags);
        }
        0x9b => {
            sbc(&mut state.a, state.e, &mut state.flags);
        }
        0x9c => {
            sbc(&mut state.a, state.h, &mut state.flags);
        }
        0x9d => {
            sbc(&mut state.a, state.l, &mut state.flags);
        }
        0x9e => {
            let data = state.get(state.get_hl());
            sbc(&mut state.a, data, &mut state.flags)
        }
        0x9f => {
            let a = state.a;
            sbc(&mut state.a, a, &mut state.flags);
        }
        0xa0 => {
            ana(&mut state.a, state.b, &mut state.flags);
        }
        0xa1 => {
            ana(&mut state.a, state.e, &mut state.flags);
        }
        0xa2 => {
            ana(&mut state.a, state.d, &mut state.flags);
        }
        0xa3 => {
            ana(&mut state.a, state.e, &mut state.flags);
        }
        0xa4 => {
            ana(&mut state.a, state.h, &mut state.flags);
        }
        0xa5 => {
            ana(&mut state.a, state.l, &mut state.flags);
        }
        0xa6 => {
            let hl = state.get_hl();
            let register = state.get(hl);
            ana(&mut state.a, register, &mut state.flags);
        }
        0xa7 => {
            let a = state.a;
            ana(&mut state.a, a, &mut state.flags);
        }
        0xa8 => {
            xra(&mut state.a, state.b, &mut state.flags);
        }
        0xa9 => {
            xra(&mut state.a, state.c, &mut state.flags);
        }
        0xaa => {
            xra(&mut state.a, state.d, &mut state.flags);
        }
        0xab => {
            xra(&mut state.a, state.e, &mut state.flags);
        }
        0xac => {
            xra(&mut state.a, state.h, &mut state.flags);
        }
        0xad => {
            xra(&mut state.a, state.l, &mut state.flags);
        }
        0xae => {
            let hl = state.get_hl();
            let register = state.get(hl);
            xra(&mut state.a, register, &mut state.flags);
        }
        0xaf => {
            let a = state.a;
            xra(&mut state.a, a, &mut state.flags);
        }
        0xb0 => or(&mut state.a, state.b, &mut state.flags),
        0xb1 => or(&mut state.a, state.c, &mut state.flags),
        0xb2 => or(&mut state.a, state.d, &mut state.flags),
        0xb3 => or(&mut state.a, state.e, &mut state.flags),
        0xb4 => or(&mut state.a, state.h, &mut state.flags),
        0xb5 => or(&mut state.a, state.l, &mut state.flags),
        0xb6 => {
            let data = state.get(state.get_hl());
            or(&mut state.a, data, &mut state.flags);
        }
        0xb7 => {
            let a = state.a;
            or(&mut state.a, a, &mut state.flags)
        }
        0xb8 => cmp(state.a, state.b, &mut state.flags),
        0xb9 => cmp(state.a, state.c, &mut state.flags),
        0xba => cmp(state.a, state.d, &mut state.flags),
        0xbb => cmp(state.a, state.e, &mut state.flags),
        0xbc => cmp(state.a, state.h, &mut state.flags),
        0xbd => cmp(state.a, state.l, &mut state.flags),
        0xbe => {
            let hl = state.get_hl();
            let register = state.get(hl);
            cmp(state.a, register, &mut state.flags);
        }
        0xbf => cmp(state.a, state.a, &mut state.flags),
        0xc0 => {
            if !state.flags.z {
                ret(&mut state.pc, &mut state.sp, &state.memory);
            }
        }
        0xc1 => {
            pop(&mut state.sp, &state.memory, &mut state.b, &mut state.c);
        }
        0xc2 => {
            if !state.flags.z {
                let lower_byte = state.get(state.pc + 1);
                let higher_byte = state.get(state.pc + 2);
                state.pc = get16bit(lower_byte, higher_byte);
                state.pc = state.pc.wrapping_sub(1);
            } else {
                state.pc = state.pc.wrapping_add(2);
            }
        }
        0xc3 => {
            let lower_byte = state.get(state.pc + 1);
            let higher_byte = state.get(state.pc + 2);
            state.pc = get16bit(lower_byte, higher_byte);
            state.pc = state.pc.wrapping_sub(1);
        }
        0xc4 => {
            if !state.flags.z {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                call(&mut state.pc, &mut state.sp, &mut state.memory, low, high);
                state.pc = state.pc.wrapping_sub(1);
            } else {
                state.pc = state.pc.wrapping_add(2)
            }
        }
        0xc5 => {
            push(&mut state.sp, &mut state.memory, state.b, state.c);
        }
        0xc6 => {
            let data = state.get(state.pc + 1);
            add(&mut state.a, data, &mut state.flags);
            state.pc = state.pc.wrapping_add(1);
        }
        0xc7 => unimplemented!(),
        0xc8 => {
            if state.flags.z {
                ret(&mut state.pc, &mut state.sp, &state.memory);
            }
        }
        0xc9 => {
            ret(&mut state.pc, &mut state.sp, &state.memory);
            state.pc = state.pc.wrapping_sub(1);
        }
        0xca => {
            if state.flags.z {
                let lower_byte = state.get(state.pc + 1);
                let higher_byte = state.get(state.pc + 2);
                state.pc = get16bit(lower_byte, higher_byte);
                state.pc = state.pc.wrapping_sub(1);
            }
        }
        0xcb => {
            state.pc = state.pc.wrapping_add(1);
            emulate_prefix(state);
        }
        0xcc => unimplemented!(),
        0xcd => {
            let lower_byte = state.get(state.pc + 1);
            let higher_byte = state.get(state.pc + 2);
            call(
                &mut state.pc,
                &mut state.sp,
                &mut state.memory,
                lower_byte,
                higher_byte,
            );
            state.pc = state.pc.wrapping_sub(1);
        }
        0xce => {
            let data = state.get(state.pc);
            adc(&mut state.a, data, &mut state.flags);
            state.pc = state.pc.wrapping_add(1);
        }
        0xcf => unimplemented!(),
        0xd0 => unimplemented!(),
        0xd1 => {
            pop(&mut state.sp, &state.memory, &mut state.d, &mut state.e);
        }
        0xd2 => unimplemented!(),
        0xd3 => state.pc = state.pc.wrapping_add(1),
        0xd4 => unimplemented!(),
        0xd5 => {
            push(&mut state.sp, &mut state.memory, state.d, state.e);
        }
        0xd6 => {
            let pc = state.get(state.pc);
            sub(&mut state.a, pc, &mut state.flags);
        }
        0xd7 => unimplemented!(),
        0xd8 => unimplemented!(),
        0xd9 => {}
        0xda => {
            if state.flags.cy {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                state.pc = get16bit(low, high);
                state.pc = state.pc.wrapping_sub(1);
            } else {
                state.pc = state.pc.wrapping_add(2);
            }
        }
        0xdb => {
            // ---
        },
        0xdc => unimplemented!(),
        0xdd => {
            // ---
        }
        0xde => {
            let data = state.get(state.pc + 1);
            sbc(&mut state.a, data, &mut state.flags);
            state.pc = state.pc.wrapping_add(1);
        },
        0xdf => unimplemented!(),
        0xe0 => {
            load(&mut state.get(state.pc), state.a);
        }
        0xe1 => {
            pop(&mut state.sp, &state.memory, &mut state.h, &mut state.l);
        }
        0xe2 => {
            load(&mut state.a, state.c);
        }
        0xe3 => {
            //PASS
            //XTHL
        }
        0xe4 => {
            if !state.flags.p {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                call(&mut state.pc, &mut state.sp, &mut state.memory, low, high);
                state.pc = state.pc.wrapping_sub(1);
            } else {
                state.pc = state.pc.wrapping_add(2);
            }
        }
        0xe5 => {
            push(&mut state.sp, &mut state.memory, state.h, state.l);
        }
        0xe6 => {
            let data = state.get(state.pc + 1);
            ani(&mut state.a, data, &mut state.flags);
            state.pc = state.pc.wrapping_add(1);
        }
        0xe7 => unimplemented!(),
        0xe8 => {
            let data = state.get(state.sp);
            add(&mut state.a, data, &mut state.flags);
        },
        0xe9 => unimplemented!(),
        0xea => {
            if state.flags.p {
                let lower_byte = state.get(state.pc + 1);
                let higher_byte = state.get(state.pc + 2);
                state.pc = get16bit(lower_byte, higher_byte);
                state.pc = state.pc.wrapping_sub(1);
            } else {
                state.pc = state.pc.wrapping_add(2);
            }
        }
        0xeb => {
            // ---
        }
        0xec => {
            // ---
        }
        0xed => {
            // ---
        }
        0xee => {
            let data = state.get(state.pc + 1);
            xor(&mut state.a, data, &mut state.flags);
            state.pc = state.pc.wrapping_add(1);
        }
        0xef => unimplemented!(),
        0xf0 => unimplemented!(),
        0xf1 => {
            let mut flags = 0;
            pop(&mut state.sp, &mut state.memory, &mut state.a, &mut flags);
            state.flags.load(flags);
        }
        0xf2 => unimplemented!(),
        0xf3 => {
            state.flags.ime = true;
        }
        0xf4 => {
            // ---
        },
        0xf5 => {
            let flags = state.flags.save();
            push(&mut state.sp, &mut state.memory, state.a, flags);
        }
        0xf6 => {
            let data = state.get(state.pc + 1);
            or(&mut state.a, data, &mut state.flags);
            state.pc = state.pc.wrapping_add(1);
        }
        0xf7 => unimplemented!(),
        0xf8 => {
            let to_add: i8 = state.get(state.pc + 1) as i8;
            let data;
            if to_add >= 0 {
                data = state.sp + (to_add as u16)
            } else {
                data = state.sp - (to_add as u16)
            }

            state.h = get_higher8(data);
            state.l = get_lower8(data);

            state.pc = state.pc.wrapping_add(1);
        }
        0xf9 => {
            state.h = get_higher8(state.sp);
            state.l = get_lower8(state.sp);

            state.pc = state.pc.wrapping_add(1);
        }
        0xfa => {
            if state.flags.s {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                state.pc = get16bit(low, high);
                state.pc = state.pc.wrapping_sub(1);
            } else {
                state.pc = state.pc.wrapping_add(2);
            }
        }
        0xfb => {
            state.int_enable = true;
        }
        0xfc => {
            if state.flags.s {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                call(&mut state.pc, &mut state.sp, &mut state.memory, low, high);
                state.pc = state.pc.wrapping_sub(1);
            } else {
                state.pc = state.pc.wrapping_add(2);
            }
        }
        0xfd => {}
        0xfe => {
            cpi(state.a, state.get(state.pc + 1), &mut state.flags);
            state.pc = state.pc.wrapping_add(1);
        }
        0xff => call(&mut state.pc, &mut state.sp, &mut state.memory, 8, 3),
    }

}


fn rlc(register: &mut u8, flags: &mut Flags) {
    *register = register.rotate_right(1);

    handle_condition_codes(*register, flags);
}

fn sla(register: &mut u8, flags: &mut Flags) {
    *register = *register << 1;

    handle_condition_codes(*register, flags);
}

fn sra(register: &mut u8, flags: &mut Flags) {
    let msb = *register & 0x80;
    *register = *register >> 1;

    if msb == 0 {
        *register = *register & 0xFE;
    } else {
        *register = *register | 0x01;
    }

    handle_condition_codes(*register, flags);
}

fn srl(register: &mut u8, flags: &mut Flags) {
    *register = *register >> 1;

    handle_condition_codes(*register, flags);
}

fn swap(register: &mut u8, flags: &mut Flags) {
    let high = *register & 0xF0;
    *register = *register << 4 | high;    

    handle_condition_codes(*register, flags);
}

fn get_bit_mask_or(bit_number: u8) -> u8 {
    let mut bit_mask: u8 = 0;

    match bit_number {
        0 => bit_mask = 0b00000001,
        1 => bit_mask = 0b00000010,
        2 => bit_mask = 0b00000100,
        3 => bit_mask = 0b00001000,
        4 => bit_mask = 0b00010000,
        5 => bit_mask = 0b00100000,
        6 => bit_mask = 0b01000000,
        7 => bit_mask = 0b10000000,
        _ => {}
    }

    bit_mask
}

fn get_bit_mask_and(bit_number: u8) -> u8 {
    let mut bit_mask: u8 = 0;

    match bit_number {
        0 => bit_mask = 0b11111110,
        1 => bit_mask = 0b11111101,
        2 => bit_mask = 0b11111011,
        3 => bit_mask = 0b11110111,
        4 => bit_mask = 0b11101111,
        5 => bit_mask = 0b11011111,
        6 => bit_mask = 0b10111111,
        7 => bit_mask = 0b01111111,
        _ => {}
    }

    bit_mask
}

fn bit(register: u8, bit_number: u8, flags: &mut Flags) {
    flags.z = register | get_bit_mask_or(bit_number) == 0;
}

fn res(register: &mut u8, bit_number: u8) {
    *register = *register & get_bit_mask_and(bit_number);
}

fn set(register: &mut u8, bit_number: u8) {
    *register = *register | get_bit_mask_or(bit_number);
}

fn emulate_prefix(state: &mut State8080) {
    let code: u8 = state.get(state.pc);

    match code {
        0x00 => rlc(&mut state.b, &mut state.flags),
        0x01 => rlc(&mut state.c, &mut state.flags),
        0x02 => rlc(&mut state.d, &mut state.flags),
        0x03 => rlc(&mut state.e, &mut state.flags),
        0x04 => rlc(&mut state.h, &mut state.flags),
        0x05 => rlc(&mut state.l, &mut state.flags),
        0x06 => rlc(&mut state.get(state.get_hl()), &mut state.flags),
        0x07 => rlc(&mut state.a, &mut state.flags),
        0x08 => rrc(&mut state.b, &mut state.flags),
        0x09 => rrc(&mut state.b, &mut state.flags),
        0x0a => rrc(&mut state.b, &mut state.flags),
        0x0b => rrc(&mut state.e, &mut state.flags),
        0x0c => rrc(&mut state.h, &mut state.flags),
        0x0d => rrc(&mut state.l, &mut state.flags),
        0x0e => rrc(&mut state.get(state.get_hl()), &mut state.flags),
        0x0f => rrc(&mut state.a, &mut state.flags),
        0x10 => rl(&mut state.b, &mut state.flags),
        0x11 => rl(&mut state.c, &mut state.flags),
        0x12 => rl(&mut state.d, &mut state.flags),
        0x13 => rl(&mut state.e, &mut state.flags),
        0x14 => rl(&mut state.h, &mut state.flags),
        0x15 => rl(&mut state.l, &mut state.flags),
        0x16 => rl(&mut state.get(state.get_hl()), &mut state.flags),
        0x17 => rl(&mut state.a, &mut state.flags),
        0x18 => rr(&mut state.b, &mut state.flags),
        0x19 => rr(&mut state.c, &mut state.flags),
        0x1a => rr(&mut state.d, &mut state.flags),
        0x1b => rr(&mut state.e, &mut state.flags),
        0x1c => rr(&mut state.h, &mut state.flags),
        0x1d => rr(&mut state.l, &mut state.flags),
        0x1e => rr(&mut state.get(state.get_hl()), &mut state.flags),
        0x1f => rr(&mut state.a, &mut state.flags),
        0x20 => sla(&mut state.b, &mut state.flags),
        0x21 => sla(&mut state.c, &mut state.flags),
        0x22 => sla(&mut state.d, &mut state.flags),
        0x23 => sla(&mut state.e, &mut state.flags),
        0x24 => sla(&mut state.h, &mut state.flags),
        0x25 => sla(&mut state.l, &mut state.flags),
        0x26 => sla(&mut state.get(state.get_hl()), &mut state.flags),
        0x27 => sla(&mut state.a, &mut state.flags),
        0x28 => sra(&mut state.b, &mut state.flags),
        0x29 => sra(&mut state.c, &mut state.flags),
        0x2a => sra(&mut state.d, &mut state.flags),
        0x2b => sra(&mut state.e, &mut state.flags),
        0x2c => sra(&mut state.h, &mut state.flags),
        0x2d => sra(&mut state.l, &mut state.flags),
        0x2e => sra(&mut state.get(state.get_hl()), &mut state.flags),
        0x2f => sra(&mut state.a, &mut state.flags),
        0x30 => swap(&mut state.b, &mut state.flags),
        0x31 => swap(&mut state.c, &mut state.flags),
        0x32 => swap(&mut state.d, &mut state.flags),
        0x33 => swap(&mut state.e, &mut state.flags),
        0x34 => swap(&mut state.h, &mut state.flags),
        0x35 => swap(&mut state.l, &mut state.flags),
        0x36 => swap(&mut state.get(state.get_hl()), &mut state.flags),
        0x37 => swap(&mut state.a, &mut state.flags),
        0x38 => srl(&mut state.b, &mut state.flags),
        0x39 => srl(&mut state.c, &mut state.flags),
        0x3a => srl(&mut state.d, &mut state.flags),
        0x3b => srl(&mut state.e, &mut state.flags),
        0x3c => srl(&mut state.h, &mut state.flags),
        0x3d => srl(&mut state.l, &mut state.flags),
        0x3e => srl(&mut state.get(state.get_hl()), &mut state.flags),
        0x3f => srl(&mut state.a, &mut state.flags),
        0x40 => bit(state.b, 0, &mut state.flags),
        0x41 => bit(state.c, 0, &mut state.flags),
        0x42 => bit(state.d, 0, &mut state.flags),
        0x43 => bit(state.e, 0, &mut state.flags),
        0x44 => bit(state.h, 0, &mut state.flags),
        0x45 => bit(state.l, 0, &mut state.flags),
        0x46 => bit(state.get(state.get_hl()), 0, &mut state.flags),
        0x47 => bit(state.a, 0, &mut state.flags),
        0x48 => bit(state.b, 1, &mut state.flags),
        0x49 => bit(state.c, 1, &mut state.flags),
        0x4a => bit(state.d, 1, &mut state.flags),
        0x4b => bit(state.e, 1, &mut state.flags),
        0x4c => bit(state.h, 1, &mut state.flags),
        0x4d => bit(state.l, 1, &mut state.flags),
        0x4e => bit(state.get(state.get_hl()), 1, &mut state.flags),
        0x4f => bit(state.a, 1, &mut state.flags),
        0x50 => bit(state.b, 2, &mut state.flags),
        0x51 => bit(state.c, 2, &mut state.flags),
        0x52 => bit(state.d, 2, &mut state.flags),
        0x53 => bit(state.e, 2, &mut state.flags),
        0x54 => bit(state.h, 2, &mut state.flags),
        0x55 => bit(state.l, 2, &mut state.flags),
        0x56 => bit(state.get(state.get_hl()), 2, &mut state.flags),
        0x57 => bit(state.a, 2, &mut state.flags),
        0x58 => bit(state.b, 3, &mut state.flags),
        0x59 => bit(state.c, 3, &mut state.flags),
        0x5a => bit(state.d, 3, &mut state.flags),
        0x5b => bit(state.e, 3, &mut state.flags),
        0x5c => bit(state.h, 3, &mut state.flags),
        0x5d => bit(state.l, 3, &mut state.flags),
        0x5e => bit(state.get(state.get_hl()), 3, &mut state.flags),
        0x5f => bit(state.a, 3, &mut state.flags),
        0x60 => bit(state.b, 4, &mut state.flags),
        0x61 => bit(state.c, 4, &mut state.flags),
        0x62 => bit(state.d, 4, &mut state.flags),
        0x63 => bit(state.e, 4, &mut state.flags),
        0x64 => bit(state.h, 4, &mut state.flags),
        0x65 => bit(state.l, 4, &mut state.flags),
        0x66 => bit(state.get(state.get_hl()), 4, &mut state.flags),
        0x67 => bit(state.a, 4, &mut state.flags),
        0x68 => bit(state.b, 5, &mut state.flags),
        0x69 => bit(state.c, 5, &mut state.flags),
        0x6a => bit(state.d, 5, &mut state.flags),
        0x6b => bit(state.e, 5, &mut state.flags),
        0x6c => bit(state.h, 5, &mut state.flags),
        0x6d => bit(state.l, 5, &mut state.flags),
        0x6e => bit(state.get(state.get_hl()), 5, &mut state.flags),
        0x6f => bit(state.a, 5, &mut state.flags),
        0x70 => bit(state.b, 6, &mut state.flags),
        0x71 => bit(state.c, 6, &mut state.flags),
        0x72 => bit(state.d, 6, &mut state.flags),
        0x73 => bit(state.e, 6, &mut state.flags),
        0x74 => bit(state.h, 6, &mut state.flags),
        0x75 => bit(state.l, 6, &mut state.flags),
        0x76 => bit(state.get(state.get_hl()), 6, &mut state.flags),
        0x77 => bit(state.a, 6, &mut state.flags),
        0x78 => bit(state.b, 7, &mut state.flags),
        0x79 => bit(state.c, 7, &mut state.flags),
        0x7a => bit(state.d, 7, &mut state.flags),
        0x7b => bit(state.e, 7, &mut state.flags),
        0x7c => bit(state.h, 7, &mut state.flags),
        0x7d => bit(state.l, 7, &mut state.flags),
        0x7e => bit(state.get(state.get_hl()), 7, &mut state.flags),
        0x7f => bit(state.a, 7, &mut state.flags),
        0x80 => res(&mut state.b, 0),
        0x81 => res(&mut state.c, 0),
        0x82 => res(&mut state.d, 0),
        0x83 => res(&mut state.e, 0),
        0x84 => res(&mut state.h, 0),
        0x85 => res(&mut state.l, 0),
        0x86 => res(&mut state.get(state.get_hl()), 0),
        0x87 => res(&mut state.a, 0),
        0x88 => res(&mut state.b, 1),
        0x89 => res(&mut state.c, 1),
        0x8a => res(&mut state.d, 1),
        0x8b => res(&mut state.e, 1),
        0x8c => res(&mut state.h, 1),
        0x8d => res(&mut state.l, 1),
        0x8e => res(&mut state.get(state.get_hl()), 1),
        0x8f => res(&mut state.a, 1),
        0x90 => res(&mut state.b, 2),
        0x91 => res(&mut state.c, 2),
        0x92 => res(&mut state.d, 2),
        0x93 => res(&mut state.e, 2),
        0x94 => res(&mut state.h, 2),
        0x95 => res(&mut state.l, 2),
        0x96 => res(&mut state.get(state.get_hl()), 2),
        0x97 => res(&mut state.a, 2),
        0x98 => res(&mut state.b, 3),
        0x99 => res(&mut state.c, 3),
        0x9a => res(&mut state.d, 3),
        0x9b => res(&mut state.e, 3),
        0x9c => res(&mut state.h, 3),
        0x9d => res(&mut state.l, 3),
        0x9e => res(&mut state.get(state.get_hl()), 3),
        0x9f => res(&mut state.a, 3),
        0xa0 => res(&mut state.b, 4),
        0xa1 => res(&mut state.c, 4),
        0xa2 => res(&mut state.d, 4),
        0xa3 => res(&mut state.e, 4),
        0xa4 => res(&mut state.h, 4),
        0xa5 => res(&mut state.l, 4),
        0xa6 => res(&mut state.get(state.get_hl()), 4),
        0xa7 => res(&mut state.a, 4),
        0xa8 => res(&mut state.b, 5),
        0xa9 => res(&mut state.c, 5),
        0xaa => res(&mut state.d, 5),
        0xab => res(&mut state.e, 5),
        0xac => res(&mut state.h, 5),
        0xad => res(&mut state.l, 5),
        0xae => res(&mut state.get(state.get_hl()), 5),
        0xaf => res(&mut state.a, 5),
        0xb0 => res(&mut state.b, 6),
        0xb1 => res(&mut state.c, 6),
        0xb2 => res(&mut state.d, 6),
        0xb3 => res(&mut state.e, 6),
        0xb4 => res(&mut state.h, 6),
        0xb5 => res(&mut state.l, 6),
        0xb6 => res(&mut state.get(state.get_hl()), 6),
        0xb7 => res(&mut state.a, 6),
        0xb8 => res(&mut state.b, 7),
        0xb9 => res(&mut state.c, 7),
        0xba => res(&mut state.d, 7),
        0xbb => res(&mut state.e, 7),
        0xbc => res(&mut state.h, 7),
        0xbd => res(&mut state.l, 7),
        0xbe => res(&mut state.get(state.get_hl()), 7),
        0xbf => res(&mut state.a, 7),
        0xc0 => set(&mut state.b, 0),
        0xc1 => set(&mut state.c, 0),
        0xc2 => set(&mut state.d, 0),
        0xc3 => set(&mut state.e, 0),
        0xc4 => set(&mut state.h, 0),
        0xc5 => set(&mut state.l, 0),
        0xc6 => set(&mut state.get(state.get_hl()), 0),
        0xc7 => set(&mut state.a, 0),
        0xc8 => set(&mut state.b, 1),
        0xc9 => set(&mut state.c, 1),
        0xca => set(&mut state.d, 1),
        0xcb => set(&mut state.e, 1),
        0xcc => set(&mut state.h, 1),
        0xcd => set(&mut state.l, 1),
        0xce => set(&mut state.get(state.get_hl()), 1),
        0xcf => set(&mut state.a, 1),
        0xd0 => set(&mut state.b, 2),
        0xd1 => set(&mut state.c, 2),
        0xd2 => set(&mut state.d, 2),
        0xd3 => set(&mut state.e, 2),
        0xd4 => set(&mut state.h, 2),
        0xd5 => set(&mut state.l, 2),
        0xd6 => set(&mut state.get(state.get_hl()), 2),
        0xd7 => set(&mut state.a, 2),
        0xd8 => set(&mut state.b, 3),
        0xd9 => set(&mut state.c, 3),
        0xda => set(&mut state.d, 3),
        0xdb => set(&mut state.e, 3),
        0xdc => set(&mut state.h, 3),
        0xdd => set(&mut state.l, 3),
        0xde => set(&mut state.get(state.get_hl()), 3),
        0xdf => set(&mut state.a, 3),
        0xe0 => set(&mut state.b, 4),
        0xe1 => set(&mut state.c, 4),
        0xe2 => set(&mut state.d, 4),
        0xe3 => set(&mut state.e, 4),
        0xe4 => set(&mut state.h, 4),
        0xe5 => set(&mut state.l, 4),
        0xe6 => set(&mut state.get(state.get_hl()), 4),
        0xe7 => set(&mut state.a, 4),
        0xe8 => set(&mut state.b, 5),
        0xe9 => set(&mut state.c, 5),
        0xea => set(&mut state.d, 5),
        0xeb => set(&mut state.e, 5),
        0xec => set(&mut state.h, 5),
        0xed => set(&mut state.l, 5),
        0xee => set(&mut state.get(state.get_hl()), 5),
        0xef => set(&mut state.a, 5),
        0xf0 => set(&mut state.b, 6),
        0xf1 => set(&mut state.c, 6),
        0xf2 => set(&mut state.d, 6),
        0xf3 => set(&mut state.e, 6),
        0xf4 => set(&mut state.h, 6),
        0xf5 => set(&mut state.l, 6),
        0xf6 => set(&mut state.get(state.get_hl()), 6),
        0xf7 => set(&mut state.a, 6),
        0xf8 => set(&mut state.b, 7),
        0xf9 => set(&mut state.c, 7),
        0xfa => set(&mut state.d, 7),
        0xfb => set(&mut state.e, 7),
        0xfc => set(&mut state.h, 7),
        0xfd => set(&mut state.l, 7),
        0xfe => set(&mut state.get(state.get_hl()), 7),
        0xff => set(&mut state.a, 7),
    }
}
