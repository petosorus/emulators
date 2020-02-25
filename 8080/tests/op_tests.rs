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

    assert_eq!(state.a, 0b00000001);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
    assert_eq!(state.flags.cy, true);
}

#[test]
fn dad_b() {
    let mut state = init_state();
    state.set(0, 0x09);
    state.c = 0x0A;
    state.b = 0x0F;
    state.h = 0;
    state.l = 0;

    em8080::emulate8080_op(&mut state);

    let hl = get16bit(state.l, state.h);
    assert_eq!(0x0A0F, hl);
}

#[test]
fn dad_b_carry() {
    let mut state = init_state();
    state.set(0, 0x09);
    state.c = 0x0A;
    state.b = 0x01;
    state.h = 0;
    state.l = 0xFF;

    em8080::emulate8080_op(&mut state);

    let hl = get16bit(state.l, state.h);
    assert_eq!(0x0B00, hl);
}

#[test]
fn dad_b_carry_to_0() {
    let mut state = init_state();
    state.set(0, 0x09);
    state.c = 0xFF;
    state.b = 0xFF;
    state.h = 0;
    state.l = 0x01;

    em8080::emulate8080_op(&mut state);

    let hl = get16bit(state.l, state.h);
    assert_eq!(0x0000, hl);
}

#[test]
fn ldax_b() {
    let mut state = init_state();
    state.set(0, 0x0a);
    state.set(0x01, 0xFF);
    state.c = 0x01;
    state.b = 0x00;

    em8080::emulate8080_op(&mut state);

    assert_eq!(0xFF, state.a);
}

#[test]
fn dcx_b() {
    let mut state = init_state();
    state.set(0, 0x0b);
    state.c = 0x01;
    state.b = 0x00;

    em8080::emulate8080_op(&mut state);

    let bc = get16bit(state.c, state.b);
    assert_eq!(0x0000, bc);
}

#[test]
fn dcx_b_carry() {
    let mut state = init_state();
    state.set(0, 0x0b);
    state.c = 0x00;
    state.b = 0xF0;

    em8080::emulate8080_op(&mut state);

    let bc = get16bit(state.c, state.b);
    assert_eq!(0xEFFF, bc);
}

#[test]
fn dcx_b_carry_to_ff() {
    let mut state = init_state();
    state.set(0, 0x0b);
    state.c = 0x00;
    state.b = 0x00;

    em8080::emulate8080_op(&mut state);

    let bc = get16bit(state.c, state.b);
    assert_eq!(0xFFFF, bc);
}

#[test]
fn rrc() {
let mut state = init_state();
    state.set(0, 0x0f);
    state.a = 0b01001000;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b00100100);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
    assert_eq!(state.flags.cy, false);
}

#[test]
fn rrc_carry() {
let mut state = init_state();
    state.set(0, 0x0f);
    state.a = 0b00000001;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b10000000);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
    assert_eq!(state.flags.cy, true);
}

#[test]
fn rar() {
let mut state = init_state();
    state.set(0, 0x1f);
    state.a = 0b01001000;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b00100100);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
    assert_eq!(state.flags.cy, false);
}

#[test]
fn rar_carry() {
let mut state = init_state();
    state.set(0, 0x1f);
    state.a = 0b00000001;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b00000000);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
    assert_eq!(state.flags.cy, true);
}

#[test]
fn rar_from_carry() {
let mut state = init_state();
    state.set(0, 0x1f);
    state.a = 0b00000000;
    state.flags.cy = true;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b10000000);
    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0);
    assert_eq!(state.flags.cy, false);
}

#[test]
fn lhld() {
let mut state = init_state();
    state.set(0, 0x2a);
    state.set(1, 0x00);
    state.set(2, 0x03);
    state.set(3, 0xDE);
    state.set(4, 0xAD);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.h, 0xAD);
    assert_eq!(state.l, 0xDE);
}

#[test]
fn cma() {
let mut state = init_state();
    state.set(0, 0x2f);
    state.a = 0x00;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0xFF);
}

#[test]
fn lda() {
let mut state = init_state();
    state.set(0, 0x3a);
    state.set(1, 0x03);
    state.set(2, 0x00);
    state.set(3, 0xBB);
    state.a = 0x00;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0xBB);
}

#[test]
fn mov() {
let mut state = init_state();
    state.set(0, 0x41); // mov c to b
    state.b = 0x00;
    state.c = 0x26;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.b, 0x26);
}

#[test]
fn add() {
let mut state = init_state();
    state.set(0, 0x80);
    state.a = 0x00;
    state.b = 0x02;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0x02);
    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, false);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn add_even_p() {
let mut state = init_state();
    state.set(0, 0x80);
    state.a = 0x00;
    state.b = 0x03;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0x03);
    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn add_zero() {
    let mut state = init_state();
    state.set(0, 0x80);
    state.a = 0x00;
    state.b = 0x00;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0x00);
    assert_eq!(state.flags.z, true);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn ana() {
    let mut state = init_state();
    state.set(0, 0xa0);
    state.a = 0b00000011;
    state.b = 0b00000110;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b00000010);
    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, false);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn ana_sign_true() {
    let mut state = init_state();
    state.set(0, 0xa0);
    state.a = 0xFF;
    state.b = 0b10000110;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b10000110);
    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, true);
    assert_eq!(state.flags.p, false);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn ana_even_p() {
    let mut state = init_state();
    state.set(0, 0xa0);
    state.a = 0b00010011;
    state.b = 0b00010110;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0b00010010);
    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn ana_zero() {
    let mut state = init_state();
    state.set(0, 0xa0);
    state.a = 0xF0;
    state.b = 0x0F;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0x00);
    assert_eq!(state.flags.z, true);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn xra_full() {
    let mut state = init_state();
    state.set(0, 0xa8);
    state.a = 0xF0;
    state.b = 0x0F;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0xFF);
    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, true);
    assert_eq!(state.flags.p, true);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}


#[test]
fn xra_even_p() {
    let mut state = init_state();
    state.set(0, 0xa8);
    state.a = 0xFF;
    state.b = 0x0E;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0xF1);
    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, true);
    assert_eq!(state.flags.p, false);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn xra_zero() {
    let mut state = init_state();
    state.set(0, 0xa8);
    state.a = 0xFF;
    state.b = 0xFF;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.a, 0x00);
    assert_eq!(state.flags.z, true);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn cmp() {
    let mut state = init_state();
    state.set(0, 0xb8);
    state.a = 0x0A;
    state.b = 0x05;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn cmp_zero() {
    let mut state = init_state();
    state.set(0, 0xb8);
    state.a = 0xFF;
    state.b = 0xFF;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.flags.z, true);
    assert_eq!(state.flags.s, false);
    assert_eq!(state.flags.p, true);
    assert_eq!(state.flags.cy, false);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn cmp_carry() {
    let mut state = init_state();
    state.set(0, 0xb8);
    state.a = 0x02;
    state.b = 0x05;

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.flags.z, false);
    assert_eq!(state.flags.s, true);
    assert_eq!(state.flags.p, false);
    assert_eq!(state.flags.cy, true);
    assert_eq!(state.flags.ac, false);
}

#[test]
fn rnz() {
    let mut state = init_state();
    state.set(0, 0xc0);
    state.flags.z = false;
    state.sp = 0x0001;
    state.set(1, 0xBF);
    state.set(2, 0xAA);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.pc, 0xAABF);
    assert_eq!(state.sp, 0x0003);
}

#[test]
fn rnz_zero() {
    let mut state = init_state();
    state.set(0, 0xc0);
    state.flags.z = true;
    state.sp = 0x0001;
    state.set(1, 0xBF);
    state.set(2, 0xAA);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.pc, 0);
    assert_eq!(state.sp, 0x0001);
}

#[test]
fn pop() {
    let mut state = init_state();
    state.set(0, 0xc1);
    state.b = 0;
    state.c = 0;
    state.sp = 0x0001;
    state.set(1, 0xBF);
    state.set(2, 0xAA);
    state.set(3, 0xFF);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.b, 0xAA);
    assert_eq!(state.c, 0xBF);
    assert_eq!(state.sp, 3);
}

#[test]
fn jnz() {
    let mut state = init_state();
    state.set(0, 0xc2);
    state.flags.z = false;
    state.set(1, 0xBF);
    state.set(2, 0xAA);

    em8080::emulate8080_op(&mut state);

    // en anticipation de pc += 1 de main, on soustrait 1
    assert_eq!(state.pc, 0xAABE);
}

#[test]
fn jnz_zero() {
    let mut state = init_state();
    state.set(0, 0xc2);
    state.flags.z = true;
    state.set(1, 0xBF);
    state.set(2, 0xAA);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.pc, 0x2);
}

#[test]
fn cnz() {
    let mut state = init_state();
    state.set(0, 0xc4);
    state.flags.z = false;
    state.set(1, 0xBF);
    state.set(2, 0xAA);

    em8080::emulate8080_op(&mut state);

    // en anticipation de pc += 1 de main, on soustrait 1
    assert_eq!(state.pc, 0xBFA9);
    assert_eq!(state.sp, 0xFFFE);
}

#[test]
fn cnz_zero() {
    let mut state = init_state();
    state.set(0, 0xc4);
    state.flags.z = true;
    state.set(1, 0xBF);
    state.set(2, 0xAA);

    em8080::emulate8080_op(&mut state);

    // en anticipation de pc += 1 de main, on soustrait 1
    assert_eq!(state.pc, 0x02);
    assert_eq!(state.sp, 0);
}

// unimplemented
/*
stax
ral
shld
daa
stc
cmc
hlt
sub
sbb
ora
adi
rst
rz
cz
aci
rnc
jnc
out
cnc
sui
rc
cc
sbi
rpo
jpo
xthl
rpe
pchl
xri
rp
jp
di
rm
sphl

*/

#[test]
fn call_ret() {
    let mut state = init_state();
    state.set(0, 0xcd);
    state.set(1, 0x10);
    state.set(2, 0x00);
    state.set(0x0010, 0xc9);

    em8080::emulate8080_op(&mut state);
    state.pc += 1;

    assert_eq!(state.pc, 0x0010);
    assert_eq!(state.sp, 0xFFFE);

    em8080::emulate8080_op(&mut state);

    assert_eq!(state.pc, 0x02);
    assert_eq!(state.sp, 0);
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

fn get16bit(lower_byte: u8, higher_byte: u8) -> u16 {
    let result: u16 = (higher_byte as u16) << 8 | (lower_byte as u16);
    result
}