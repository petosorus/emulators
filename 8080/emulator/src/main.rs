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
    memory: Box<[u8]>,
    cc: ConditionCodes,
    int_enable: u8,
}

fn Emulate8080Op(state: State8080) {
    let code: &u8 = &state.memory[state.pc];

    match code {
        0x00 => println!("NOP"),
        0x01 => {
            println!(
                "LXI    B,#$"
            );
        }
        0x02 => println!("STAX   B"),
        0x03 => println!("INX    B"),
        0x04 => println!("INR    B"),
        0x05 => println!("DCR    B"),
        0x06 => {
            println!("MVI    B,#$");
        }
        0x07 => println!("RLC"),
        0x08 => println!("NOP"),
        &_ => {}
    }
}

fn main() {
    
}
