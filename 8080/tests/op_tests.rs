use em8080;

#[test]
fn nop() {
    let mut state = init_state();
    state.set(0, 0x00);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
}

#[test]
fn lxi_b() {
    let mut state = init_state();
    // Instruction
    state.set(0, 0x01);
    // Data
    state.set(1, 0x02);
    state.set(2, 0x03);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.c, 0x02);
    assert_eq!(state.b, 0x03);
    assert_eq!(state.pc, 2);
    assert_eq!(state.sp, 0);
}

#[test]
fn inx_b_only_c() {
    let mut state = init_state();
    state.set(0, 0x03);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.c, 0x01);
    assert_eq!(state.b, 0x00);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
}

#[test]
fn inx_b_carry_to_b() {
    let mut state = init_state();
    state.set(0, 0x03);
    state.b = 0x00;
    state.c = 0xFF;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.c, 0x00);
    assert_eq!(state.b, 0x01);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
}

#[test]
fn inx_b_carry_to_0() {
    let mut state = init_state();
    state.set(0, 0x03);
    state.b = 0xFF;
    state.c = 0xFF;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.c, 0x00);
    assert_eq!(state.b, 0x00);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
}


#[test]
fn inr_b() {
let mut state = init_state();
    state.set(0, 0x04);
    state.b = 0x00;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.b, 0x01);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);

    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn inr_b_to_0() {
let mut state = init_state();
    state.set(0, 0x04);
    state.b = 0xFF;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.b, 0x00);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);

    assert_eq!(state.flags.z, true);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
}

#[test]
fn dcr_b() {
let mut state = init_state();
    state.set(0, 0x05);
    state.b = 0x01;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.b, 0x00);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);

    assert_eq!(state.flags.z, true);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
}

#[test]
fn dcr_b_to_ff() {
let mut state = init_state();
    state.set(0, 0x05);
    state.b = 0x00;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.b, 0xFF);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);

    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, true);
    assert_eq!(state.flags.p, true);
}

#[test]
fn mvi_b() {
let mut state = init_state();
    state.set(0, 0x06);
    state.set(1, 0xFA);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.b, 0xFA);
    assert_eq!(state.pc, 1);
    assert_eq!(state.sp, 0);
}

#[test]
fn rlc() {
let mut state = init_state();
    state.set(0, 0x07);
    state.a = 0b01001000;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b10010000);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
    assert_eq!(state.flags.cy, false);
}

#[test]
fn rlc_carry() {
let mut state = init_state();
    state.set(0, 0x07);
    state.a = 0b10000000;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b00000000);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
    assert_eq!(state.flags.cy, true);
}

fn init_state() -> em8080::State8080 {
    let flags = em8080::Flags {
        z: false,
        s: false,
        p: false,
        cy: false,
        ac: false,
        pad: 0,
    };

    em8080::State8080 {
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
        int_enable: 0,
    }
}