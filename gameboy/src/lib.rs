pub struct Flags {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool
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
    *register += 1;

    handle_condition_codes(*register, flags);
}

fn dcr(register: &mut u8, flags: &mut Flags) {
    *register -= 1;

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
    *sp -= 1;
    *(memory.get_mut(*sp as usize)) = get_higher8(next_pc);
    *sp -= 1;
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

fn mov(target: &mut u8, source: u8) {
    *target = source;
}

fn lxi_sp(sp: &mut u16, low: u8, high: u8) {
    *sp = get16bit(low, high);
}

fn load(register: &mut u8, memory: &Memory, adr_high: u8, adr_low: u8) {
    let adr = get16bit(adr_low, adr_high);
    load_adr(register, memory, adr);
}

fn load_adr(register: &mut u8, memory: &Memory, adr: u16) {
    *register = memory.get(adr as usize);
}

fn add(target: &mut u8, source: u8, flags: &mut Flags) {
    let old_value = *target;
    *target = *target + source;

    handle_condition_codes(*target, flags);
    if *target < old_value {
        flags.cy = true;
    }
}

fn ani(register: &mut u8, data: u8, flags: &mut Flags) {
    *register = *register & data;

    flags.cy = false;
    handle_condition_codes(*register, flags);
}

fn dad(target_left: &mut u8, target_right: &mut u8, source: u16, flags: &mut Flags) {
    let mut target = get16bit(*target_right, *target_left);
    let targetcheck = target.clone();

    target += source;
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

fn rrc(register: &mut u8, flags: &mut Flags) {
    let lsb = *register & 0x01;
    *register = *register >> 1;

    if lsb == 0 {
        flags.cy = false;
        *register = *register & 0x7F;
    } else {
        flags.cy = true;
        *register = *register | 0x80;
    }
}

fn rlc(register: &mut u8, flags: &mut Flags) {
    let msb = *register & 0x80;
    *register = *register << 1;

    if msb == 0 {
        flags.cy = false;
        *register = *register & 0xFE;
    } else {
        flags.cy = true;
        *register = *register | 0x01;
    }
}

fn ori(register: &mut u8, data: u8, flags: &mut Flags) {
    flags.cy = false;

    *register = *register | data;

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
    let result = register - data;
    if result > register {
        flags.cy = true;
    } else {
        flags.cy = false;
    }

    //println!("{}, {}, {}", register, data, result);

    handle_condition_codes(result, flags);
}

fn cmp(accumulator: u8, register: u8, flags: &mut Flags) {
    let result = accumulator - register;
    flags.cy = register > accumulator;
    handle_condition_codes(result, flags);
}

pub fn handle_interrupt() {

}

pub fn emulate8080_op(state: &mut State8080) {
    let code: u8 = state.get(state.pc);

    match code {
        0x00 => {}
        0x01 => {
            state.c = state.get(state.pc + 1);
            state.b = state.get(state.pc + 2);
            state.pc += 2;
        }
        0x02 => unimplemented!(),
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
            mov(&mut state.b, source);
            state.pc += 1;
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
            load(&mut state.a, &state.memory, state.b, state.c);
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
            mov(&mut state.c, source);
            state.pc += 1;
        }
        0x0f => rrc(&mut state.a, &mut state.flags),
        0x10 => {}
        0x11 => {
            state.e = state.get(state.pc + 1);
            state.d = state.get(state.pc + 2);
            state.pc += 2;
        }
        0x12 => unimplemented!(),
        0x13 => {
            inx(&mut state.d, &mut state.e)
        }
        0x14 => {
            inr(&mut state.d, &mut state.flags);
        }
        0x15 => {
            dcr(&mut state.d, &mut state.flags);
        }
        0x16 => {
            let source = state.get(state.pc + 1);
            mov(&mut state.d, source);
            state.pc += 1;
        }
        0x17 => unimplemented!(),
        0x18 => {}
        0x19 => {
            let de = get16bit(state.d, state.e);
            dad(&mut state.h, &mut state.l, de, &mut state.flags);
        }
        0x1a => {
            load(&mut state.a, &state.memory, state.d, state.e);
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
            mov(&mut state.e, source);
            state.pc += 1;
        }
        0x1f => {
            rar(&mut state.a, &mut state.flags);
        }
        0x20 => {}
        0x21 => {
            state.l = state.get(state.pc + 1);
            state.h = state.get(state.pc + 2);
            state.pc += 2;
        }
        0x22 => unimplemented!(),
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
            mov(&mut state.h, source);
            state.pc += 1;
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
            mov(&mut state.l, source);
            state.pc += 1;
        }
        0x2f => {
            cma(&mut state.a);
        }
        0x30 => {}
        0x31 => {
            let low = state.get(state.pc + 1);
            let high = state.get(state.pc + 2);
            lxi_sp(&mut state.sp, low, high);
            state.pc += 2;
        }
        0x32 => {
            let lower_byte = state.get(state.pc + 1);
            let higher_byte = state.get(state.pc + 2);
            let adressed_memory = state
                .memory
                .get_mut(get16bit(lower_byte, higher_byte) as usize);
            mov(adressed_memory, state.a);
            state.pc += 2;
        }
        0x33 => unimplemented!(),
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
            mov(register, source);
            state.pc += 1;
        }
        0x37 => unimplemented!(),
        0x38 => {}
        0x39 => {
            dad(&mut state.h, &mut state.l, state.sp, &mut state.flags);
        }
        0x3a => {
            let low = state.get(state.pc + 1);
            let high = state.get(state.pc + 2);
            state.pc += 2;
            load(&mut state.a, &state.memory, high, low);
        },
        0x3b => {
            dcx_sp(&mut state.sp);
        }
        0x3c => {
            inr(&mut state.a, &mut state.flags);
        }
        0x3d => dcr(&mut state.a, &mut state.flags),
        0x3e => {
            state.a = state.get(state.pc + 1);
            state.pc += 1;
        }
        0x3f => unimplemented!(),
        0x40 => {
            let b = state.b;
            mov(&mut state.b, b);
        }
        0x41 => {
            mov(&mut state.b, state.c);
        }
        0x42 => {
            mov(&mut state.b, state.d);
        }
        0x43 => {
            mov(&mut state.b, state.e);
        }
        0x44 => {
            mov(&mut state.b, state.h);
        }
        0x45 => {
            mov(&mut state.b, state.l);
        }
        0x46 => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            mov(&mut state.b, value)
        }
        0x47 => {
            mov(&mut state.b, state.a);
        }
        0x48 => {
            mov(&mut state.c, state.b);
        }
        0x49 => {
            let c = state.c;
            mov(&mut state.c, c);
        }
        0x4a => {
            mov(&mut state.c, state.d);
        }
        0x4b => {
            mov(&mut state.c, state.e);
        }
        0x4c => {
            mov(&mut state.c, state.h);
        }
        0x4d => {
            mov(&mut state.c, state.l);
        }
        0x4e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            mov(&mut state.c, value)
        }
        0x4f => {
            mov(&mut state.c, state.a);
        }
        0x50 => {
            mov(&mut state.d, state.b);
        }
        0x51 => {
            mov(&mut state.d, state.c);
        }
        0x52 => {
            let d = state.d;
            mov(&mut state.d, d);
        }
        0x53 => {
            mov(&mut state.d, state.e);
        }
        0x54 => {
            mov(&mut state.d, state.h);
        }
        0x55 => {
            mov(&mut state.d, state.l);
        }
        0x56 => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            mov(&mut state.d, value)
        }
        0x57 => {
            mov(&mut state.d, state.a);
        }
        0x58 => {
            mov(&mut state.e, state.b);
        }
        0x59 => {
            mov(&mut state.e, state.c);
        }
        0x5a => {
            mov(&mut state.e, state.d);
        }
        0x5b => {
            let e = state.e;
            mov(&mut state.e, e);
        }
        0x5c => {
            mov(&mut state.e, state.h);
        }
        0x5d => {
            mov(&mut state.e, state.l);
        }
        0x5e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            mov(&mut state.e, value)
        }
        0x5f => {
            mov(&mut state.e, state.a);
        }
        0x60 => {
            mov(&mut state.h, state.b);
        }
        0x61 => mov(&mut state.h, state.c),
        0x62 => {
            mov(&mut state.h, state.d);
        }
        0x63 => {
            mov(&mut state.h, state.e);
        }
        0x64 => {
            let h = state.h;
            mov(&mut state.h, h);
        }
        0x65 => {
            mov(&mut state.h, state.l);
        }
        0x66 => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            mov(&mut state.h, value);
        }
        0x67 => {
            mov(&mut state.h, state.a);
        }
        0x68 => {
            mov(&mut state.l, state.b);
        }
        0x69 => {
            mov(&mut state.l, state.c);
        }
        0x6a => {
            mov(&mut state.l, state.d);
        }
        0x6b => {
            mov(&mut state.l, state.e);
        }
        0x6c => {
            mov(&mut state.l, state.h);
        }
        0x6d => {
            let l = state.l;
            mov(&mut state.l, l);
        }
        0x6e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            mov(&mut state.d, value);
        }
        0x6f => mov(&mut state.l, state.a),
        0x70 => {
            let b = state.b;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            mov(register, b)
        }
        0x71 => {
            let c = state.c;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            mov(register, c)
        }
        0x72 => {
            let d = state.d;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            mov(register, d)
        }
        0x73 => {
            let e = state.e;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            mov(register, e)
        }
        0x74 => {
            let h = state.h;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            mov(register, h)
        }
        0x75 => {
            let l = state.l;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            mov(register, l)
        }
        0x76 => unimplemented!(),
        0x77 => {
            let a = state.a;
            let hl = get16bit(state.l, state.h);
            let register = state.get_mut(hl);
            mov(register, a)
        }
        0x78 => {
            mov(&mut state.a, state.b);
        }
        0x79 => {
            mov(&mut state.a, state.c);
        }
        0x7a => {
            mov(&mut state.a, state.d);
        }
        0x7b => {
            mov(&mut state.a, state.e);
        }
        0x7c => {
            mov(&mut state.a, state.h);
        }
        0x7d => {
            mov(&mut state.a, state.l);
        }
        0x7e => {
            let hl = get16bit(state.l, state.h);
            let value = state.memory.get(hl as usize);
            mov(&mut state.a, value);
        }
        0x7f => {
            let a = state.a;
            mov(&mut state.a, a);
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
        0xa0 => {
            ana(&mut state.a, state.b, &mut state.flags);
        }
        0xa1 => {
            ana(&mut state.a, state.c, &mut state.flags);
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
        },
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
        0xb0 => unimplemented!(),
        0xb1 => unimplemented!(),
        0xb2 => unimplemented!(),
        0xb3 => unimplemented!(),
        0xb4 => unimplemented!(),
        0xb5 => unimplemented!(),
        0xb6 => unimplemented!(),
        0xb7 => unimplemented!(),
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
                state.pc -= 1;
            } else {
                state.pc += 2;
            }
        }
        0xc3 => {
            let lower_byte = state.get(state.pc + 1);
            let higher_byte = state.get(state.pc + 2);
            state.pc = get16bit(lower_byte, higher_byte);
            state.pc -= 1;
        }
        0xc4 => {
            if !state.flags.z {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                call(&mut state.pc, &mut state.sp, &mut state.memory, low, high);
                state.pc -= 1;
            } else {
                state.pc += 2
            }
        }
        0xc5 => {
            push(&mut state.sp, &mut state.memory, state.b, state.c);
        }
        0xc6 => {
            let data = state.get(state.pc + 1);
            add(&mut state.a, data, &mut state.flags);
            state.pc += 1;
        }
        0xc7 => unimplemented!(),
        0xc8 => unimplemented!(),
        0xc9 => {
            ret(&mut state.pc, &mut state.sp, &state.memory);
            state.pc -= 1;
        }
        0xca => {
            if state.flags.z {
                let lower_byte = state.get(state.pc + 1);
                let higher_byte = state.get(state.pc + 2);
                state.pc = get16bit(lower_byte, higher_byte);
                state.pc -= 1;
            }
        }
        0xcb => {}
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
            state.pc -= 1;
        }
        0xce => unimplemented!(),
        0xcf => unimplemented!(),
        0xd0 => unimplemented!(),
        0xd1 => {
            pop(&mut state.sp, &state.memory, &mut state.d, &mut state.e);
        }
        0xd2 => unimplemented!(),
        0xd3 => state.pc += 1,
        0xd4 => unimplemented!(),
        0xd5 => {
            push(&mut state.sp, &mut state.memory, state.d, state.e);
        }
        0xd6 => unimplemented!(),
        0xd7 => unimplemented!(),
        0xd8 => unimplemented!(),
        0xd9 => {}
        0xda => {
            if state.flags.cy {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                state.pc = get16bit(low, high);
                state.pc -= 1;
            } else {
                state.pc += 2;
            }
        },
        0xdb => unimplemented!(),
        0xdc => unimplemented!(),
        0xdd => {}
        0xde => unimplemented!(),
        0xdf => unimplemented!(),
        0xe0 => unimplemented!(),
        0xe1 => {
            pop(&mut state.sp, &state.memory, &mut state.h, &mut state.l);
        }
        0xe2 => unimplemented!(),
        0xe3 => unimplemented!(),
        0xe4 => {
            if !state.flags.p {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                call(&mut state.pc, &mut state.sp, &mut state.memory, low, high);
                state.pc -= 1;
            } else {
                state.pc += 2;
            }
        }
        0xe5 => {
            push(&mut state.sp, &mut state.memory, state.h, state.l);
        }
        0xe6 => {
            let data = state.get(state.pc + 1);
            ani(&mut state.a, data, &mut state.flags);
            state.pc += 1;
        }
        0xe7 => unimplemented!(),
        0xe8 => unimplemented!(),
        0xe9 => unimplemented!(),
        0xea => {
            if state.flags.p {
                let lower_byte = state.get(state.pc + 1);
                let higher_byte = state.get(state.pc + 2);
                state.pc = get16bit(lower_byte, higher_byte);
                state.pc -= 1;
            } else {
                state.pc += 2;
            }
        }
        0xeb => {
            xchg(state);
        }
        0xec => {
            let low = state.get(state.pc + 1);
            let high = state.get(state.pc + 2);
            state.pc += 2;
            if state.flags.p {
                call(&mut state.pc, &mut state.sp, &mut state.memory, low, high);
                state.pc -= 1;
            }
        }
        0xed => {}
        0xee => unimplemented!(),
        0xef => unimplemented!(),
        0xf0 => unimplemented!(),
        0xf1 => {
            let mut flags = 0;
            pop(&mut state.sp, &mut state.memory, &mut state.a, &mut flags);
            state.flags.load(flags);
        },
        0xf2 => unimplemented!(),
        0xf3 => unimplemented!(),
        0xf4 => unimplemented!(),
        0xf5 => {
            let flags = state.flags.save();
            push(&mut state.sp, &mut state.memory, state.a, flags);
        }
        0xf6 => {
            let data = state.get(state.pc + 1);
            ori(&mut state.a, data, &mut state.flags);
            state.pc += 1;
        }
        0xf7 => unimplemented!(),
        0xf8 => unimplemented!(),
        0xf9 => unimplemented!(),
        0xfa => {
            if state.flags.s {
                let low = state.get(state.pc + 1);
                let high = state.get(state.pc + 2);
                state.pc = get16bit(low, high);
                state.pc -= 1;
            } else {
                state.pc += 2;
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
                state.pc -= 1;
            } else {
                state.pc += 2;
            }
        }
        0xfd => {}
        0xfe => {
            cpi(state.a, state.get(state.pc + 1), &mut state.flags);
            state.pc += 1;
        }
        0xff => call(&mut state.pc, &mut state.sp, &mut state.memory, 8, 3),
    }
}


