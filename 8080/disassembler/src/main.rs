use std::fs;

fn disassemble8080op(codebuffer: &Vec<u8>, pc: usize) -> usize {
    let code: &u8 = codebuffer.get(pc).expect("oh oh");
    let mut opbytes: usize = 1;

    match code {
        0x00 => println!("NOP"),
        0x01 => {
            println!(
                "LXI    B,#${:02x}{:02x}",
                codebuffer[pc + 2],
                codebuffer[pc + 1]
            );
            opbytes = 3;
        }
        0x02 => println!("STAX   B"),
        0x03 => println!("INX    B"),
        0x04 => println!("INR    B"),
        0x05 => println!("DCR    B"),
        0x06 => {
            println!("MVI    B,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        }
        0x07 => println!("RLC"),
        0x08 => println!("NOP"),
        0x09 => println!("DAD   B"),
        0x0a => println!("LDAX  B"),
        0x0b => println!("DCX   B"),
        0x0c => println!("INCR  C"),
        0x0d => println!("DCR   C"),
        0x0e => {
            println!("MVI   C,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0x0f => println!("RRC"),
        0x11 => {
            println!("LXI   D,{:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0x3e => {
            println!("MVI    A,#{:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xc3 => {
            println!("JMP    ${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        &_ => {}
    }

    opbytes
}

fn main() {

    let filename = "invaders.h";
    let rom: Vec<u8> = fs::read(filename)
        .expect("Something wrong");        
    let mut pc: usize = 0;

    while pc < rom.len() {
        pc += disassemble8080op(&rom, pc);
    }
}
