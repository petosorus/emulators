struct ConditionCodes {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
    pad: u8,
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
    pc: usize,
    memory: Vec<u8>,
    cc: ConditionCodes,
    int_enable: u8,
}

fn emulate8080_op(state: &mut State8080) {
    let code: u8 = state.memory[state.pc];

    match code {
        0x00 => {},
        0x01 => {
            state.pc += 1;
            state.c = state.memory[state.pc];
            state.pc += 1;
            state.b = state.memory[state.pc];
        },
        0x02 => println!("STAX   B"),
        0x03 => println!("INX    B"),
        0x04 => println!("INR    B"),
        0x05 => println!("DCR    B"),
        0x06 => {
            println!("MVI    B,#$");
        }
        0x07 => println!("RLC"),
        0x08 => println!("NOP"),
        _ => {}
    }
}

fn main() {
    let cc = ConditionCodes {
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
        memory: Vec::new(),
        cc: cc,
        int_enable: 0,
    };

    state.memory.push(1);
    state.memory.push(12);
    state.memory.push(123);

    emulate8080_op(&mut state);

    println!("b {}, c {}, pc {}", state.b, state.c, state.pc);
}
