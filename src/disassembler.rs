use std::fs;

pub fn disassemble8080op(codebuffer: &Vec<u8>, pc: u16) -> u16 {
    let code: &u8 = codebuffer.get(pc as usize).expect("oh	oh");
    let mut opbytes: u16 = 1;

    match code {
        0x00 => println!("{:02x}: NOP", code),
        0x01 => {
            println!(
                "{:02x}: LD\tBC,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x02 => println!("{:02x}: LD\t[BC],A", code),
        0x03 => println!("{:02x}: INC\tBC", code),
        0x04 => println!("{:02x}: INC\tB", code),
        0x05 => println!("{:02x}: DEC\tB", code),
        0x06 => {
            println!(
                "{:02x}: LD\tB,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x07 => println!("{:02x}: RLCA", code),
        0x08 => {
            println!(
                "{:02x}: LD\t[{:02x}],A",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x09 => println!("{:02x}: ADD\tHL?BC", code),
        0x0a => println!("{:02x}: LD\tA,[BC]", code),
        0x0b => println!("{:02x}: DEC\tBC", code),
        0x0c => println!("{:02x}: INC\tC", code),
        0x0d => println!("{:02x}: DEC\tC", code),
        0x0e => {
            println!(
                "{:02x}: LD\tC,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x0f => println!("{:02x}: RRCA", code),
        0x10 => {
            println!(
                "{:02x}: STOP\t{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x11 => {
            println!(
                "{:02x}: LD\tDE,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x12 => println!("{:02x}: LD\t[DE],A", code),
        0x13 => println!("{:02x}: INC\tDE", code),
        0x14 => println!("{:02x}: INC\tD", code),
        0x15 => println!("{:02x}: DEC\tD", code),
        0x16 => {
            println!(
                "{:02x}: LD\tD,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x17 => println!("{:02x}: RLA", code),
        0x18 => {
            println!(
                "{:02x}: JR\t{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x19 => println!("{:02x}: ADD\tHL,DE", code),
        0x1a => println!("{:02x}: LD\tA,[DE]", code),
        0x1b => println!("{:02x}: DEC\tDE", code),
        0x1c => println!("{:02x}: INC\tE", code),
        0x1d => println!("{:02x}: DEC\tE", code),
        0x1e => {
            println!(
                "{:02x}: LD\tE,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x1f => println!("{:02x}: RRA", code),
        0x20 => {
            println!(
                "{:02x}: JR\tNZ,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x21 => {
            println!(
                "{:02x}: LD\tHL,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x22 => println!("{:02x}: LD\t[HL+],A", code),
        0x23 => println!("{:02x}: INC\tHL", code),
        0x24 => println!("{:02x}: INC\tH", code),
        0x25 => println!("{:02x}: DEC\tH", code),
        0x26 => {
            println!(
                "{:02x}: LD\tH,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x27 => println!("{:02x}: DAA", code),
        0x28 => {
            println!(
                "{:02x}: JR\tZ,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x29 => println!("{:02x}: ADD\tHL,HL", code),
        0x2a => println!("{:02x}: LD\tA,[HL+]", code),
        0x2b => println!("{:02x}: DEC\tHL", code),
        0x2c => println!("{:02x}: INC\tL", code),
        0x2d => println!("{:02x}: DEC\tL", code),
        0x2e => {
            println!(
                "{:02x}: LD\tL,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x2f => println!("{:02x}: CPL", code),
        0x30 => {
            println!(
                "{:02x}: JR\tNC,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x31 => {
            println!(
                "{:02x}: LD\tSP,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x32 => println!("{:02x}: LD\t[HL-],A", code),
        0x33 => println!("{:02x}: INC\tSP", code),
        0x34 => println!("{:02x}: INC\t[HL]", code),
        0x35 => println!("{:02x}: DEC\t[HL]", code),
        0x36 => {
            println!(
                "{:02x}: LD\t[HL],{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x37 => println!("{:02x}: SCF", code),
        0x38 => {
            println!(
                "{:02x}: JR\tC,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x39 => println!("{:02x}: ADD\tHL,SP", code),
        0x3a => println!("{:02x}: LD\tA,[HL-]", code),
        0x3b => println!("{:02x}: DEC\tSP", code),
        0x3c => println!("{:02x}: INC\tA", code),
        0x3d => println!("{:02x}: DEC\tA", code),
        0x3e => {
            println!(
                "{:02x}: LD\tA,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0x3f => println!("{:02x}: CCF", code),
        0x40 => println!("{:02x}: LD\tB,B", code),
        0x41 => println!("{:02x}: LD\tB,C", code),
        0x42 => println!("{:02x}: LD\tB,D", code),
        0x43 => println!("{:02x}: LD\tB,E", code),
        0x44 => println!("{:02x}: LD\tB,H", code),
        0x45 => println!("{:02x}: LD\tB,L", code),
        0x46 => println!("{:02x}: LD\tB,[HL]", code),
        0x47 => println!("{:02x}: LD\tB,A", code),
        0x48 => println!("{:02x}: LD\tC,B", code),
        0x49 => println!("{:02x}: LD\tC,C", code),
        0x4a => println!("{:02x}: LD\tC,D", code),
        0x4b => println!("{:02x}: LD\tC,E", code),
        0x4c => println!("{:02x}: LD\tC,H", code),
        0x4d => println!("{:02x}: LD\tC,L", code),
        0x4e => println!("{:02x}: LD\tC,[HL]", code),
        0x4f => println!("{:02x}: LD\tC,A", code),
        0x50 => println!("{:02x}: LD\tD,B", code),
        0x51 => println!("{:02x}: LD\tD,C", code),
        0x52 => println!("{:02x}: LD\tD,D", code),
        0x53 => println!("{:02x}: LD\tD,E", code),
        0x54 => println!("{:02x}: LD\tD,H", code),
        0x55 => println!("{:02x}: LD\tD,L", code),
        0x56 => println!("{:02x}: LD\tD,[HL]", code),
        0x57 => println!("{:02x}: LD\tD,A", code),
        0x58 => println!("{:02x}: LD\tE,B", code),
        0x59 => println!("{:02x}: LD\tE,C", code),
        0x5a => println!("{:02x}: LD\tE,D", code),
        0x5b => println!("{:02x}: LD\tE,E", code),
        0x5c => println!("{:02x}: LD\tE,H", code),
        0x5d => println!("{:02x}: LD\tE,L", code),
        0x5e => println!("{:02x}: LD\tE,[HL]", code),
        0x5f => println!("{:02x}: LD\tE,A", code),
        0x60 => println!("{:02x}: LD\tH,B", code),
        0x61 => println!("{:02x}: LD\tH,C", code),
        0x62 => println!("{:02x}: LD\tH,D", code),
        0x63 => println!("{:02x}: LD\tH,E", code),
        0x64 => println!("{:02x}: LD\tH,H", code),
        0x65 => println!("{:02x}: LD\tH,L", code),
        0x66 => println!("{:02x}: LD\tH,[HL]", code),
        0x67 => println!("{:02x}: LD\tH,A", code),
        0x68 => println!("{:02x}: LD\tL,B", code),
        0x69 => println!("{:02x}: LD\tL,C", code),
        0x6a => println!("{:02x}: LD\tL,D", code),
        0x6b => println!("{:02x}: LD\tL,E", code),
        0x6c => println!("{:02x}: LD\tL,H", code),
        0x6d => println!("{:02x}: LD\tL,L", code),
        0x6e => println!("{:02x}: LD\tL,[HL]", code),
        0x6f => println!("{:02x}: LD\tL,A", code),
        0x70 => println!("{:02x}: LD\t[HL],B", code),
        0x71 => println!("{:02x}: LD\t[HL],C", code),
        0x72 => println!("{:02x}: LD\t[HL],D", code),
        0x73 => println!("{:02x}: LD\t[HL],E", code),
        0x74 => println!("{:02x}: LD\t[HL],H", code),
        0x75 => println!("{:02x}: LD\t[HL],L", code),
        0x76 => println!("{:02x}: HALT", code),
        0x77 => println!("{:02x}: LD\t[HL],A", code),
        0x78 => println!("{:02x}: LD\tA,B", code),
        0x79 => println!("{:02x}: LD\tA,C", code),
        0x7a => println!("{:02x}: LD\tA,D", code),
        0x7b => println!("{:02x}: LD\tA,E", code),
        0x7c => println!("{:02x}: LD\tA,H", code),
        0x7d => println!("{:02x}: LD\tA,L", code),
        0x7e => println!("{:02x}: LD\tA,[HL]", code),
        0x7f => println!("{:02x}: LD\tA,A", code),
        0x80 => println!("{:02x}: ADD\tA,B", code),
        0x81 => println!("{:02x}: ADD\tA,C", code),
        0x82 => println!("{:02x}: ADD\tA,D", code),
        0x83 => println!("{:02x}: ADD\tA,E", code),
        0x84 => println!("{:02x}: ADD\tA,H", code),
        0x85 => println!("{:02x}: ADD\tA,L", code),
        0x86 => println!("{:02x}: ADD\tA,[HL]", code),
        0x87 => println!("{:02x}: ADD\tA,A", code),
        0x88 => println!("{:02x}: ADC\tA,B", code),
        0x89 => println!("{:02x}: ADC\tA,C", code),
        0x8a => println!("{:02x}: ADC\tA,D", code),
        0x8b => println!("{:02x}: ADC\tA,E", code),
        0x8c => println!("{:02x}: ADC\tA,H", code),
        0x8d => println!("{:02x}: ADC\tA,L", code),
        0x8e => println!("{:02x}: ADC\tA,[HL]", code),
        0x8f => println!("{:02x}: ADC\tA,A", code),
        0x90 => println!("{:02x}: SUB\tA,B", code),
        0x91 => println!("{:02x}: SUB\tA,C", code),
        0x92 => println!("{:02x}: SUB\tA,D", code),
        0x93 => println!("{:02x}: SUB\tA,E", code),
        0x94 => println!("{:02x}: SUB\tA,H", code),
        0x95 => println!("{:02x}: SUB\tA,L", code),
        0x96 => println!("{:02x}: SUB\tA,[HL]", code),
        0x97 => println!("{:02x}: SUB\tA,A", code),
        0x98 => println!("{:02x}: SBC\tA,B", code),
        0x99 => println!("{:02x}: SBC\tA,C", code),
        0x9a => println!("{:02x}: SBC\tA,D", code),
        0x9b => println!("{:02x}: SBC\tA,E", code),
        0x9c => println!("{:02x}: SBC\tA,H", code),
        0x9d => println!("{:02x}: SBC\tA,L", code),
        0x9e => println!("{:02x}: SBC\tA,[HL]", code),
        0x9f => println!("{:02x}: SBC\tA,A", code),
        0xa0 => println!("{:02x}: AND\tA,B", code),
        0xa1 => println!("{:02x}: AND\tA,C", code),
        0xa2 => println!("{:02x}: AND\tA,D", code),
        0xa3 => println!("{:02x}: AND\tA,E", code),
        0xa4 => println!("{:02x}: AND\tA,H", code),
        0xa5 => println!("{:02x}: AND\tA,L", code),
        0xa6 => println!("{:02x}: AND\tA,[HL]", code),
        0xa7 => println!("{:02x}: AND\tA,A", code),
        0xa8 => println!("{:02x}: XOR\tA,B", code),
        0xa9 => println!("{:02x}: XOR\tA,C", code),
        0xaa => println!("{:02x}: XOR\tA,D", code),
        0xab => println!("{:02x}: XOR\tA,E", code),
        0xac => println!("{:02x}: XOR\tA,H", code),
        0xad => println!("{:02x}: XOR\tA,L", code),
        0xae => println!("{:02x}: XOR\tA,[HL]", code),
        0xaf => println!("{:02x}: XOR\tA,A", code),
        0xb0 => println!("{:02x}: OR\tA,B", code),
        0xb1 => println!("{:02x}: OR\tA,C", code),
        0xb2 => println!("{:02x}: OR\tA,D", code),
        0xb3 => println!("{:02x}: OR\tA,E", code),
        0xb4 => println!("{:02x}: OR\tA,H", code),
        0xb5 => println!("{:02x}: OR\tA,L", code),
        0xb6 => println!("{:02x}: OR\tA,[HL]", code),
        0xb7 => println!("{:02x}: OR\tA,A", code),
        0xb8 => println!("{:02x}: CP\tA,B", code),
        0xb9 => println!("{:02x}: CP\tA,C", code),
        0xba => println!("{:02x}: CP\tA,D", code),
        0xbb => println!("{:02x}: CP\tA,E", code),
        0xbc => println!("{:02x}: CP\tA,H", code),
        0xbd => println!("{:02x}: CP\tA,L", code),
        0xbe => println!("{:02x}: CP\tA,[HL]", code),
        0xbf => println!("{:02x}: CP\tA,A", code),
        0xc0 => println!("{:02x}: RET\tNZ", code),
        0xc1 => println!("{:02x}: POP\tBC", code),
        0xc2 => {
            println!(
                "{:02x}: JP\tNZ,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xc3 => {
            println!(
                "{:02x}: JP\t{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xc4 => {
            println!(
                "{:02x}: CALL\tNZ,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xc5 => println!("{:02x}: PUSH\tBC", code),
        0xc6 => {
            println!("{:02x}: ADD\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xc7 => println!("{:02x}: RST\t$00", code),
        0xc8 => println!("{:02x}: RET\tZ", code),
        0xc9 => println!("{:02x}: RET", code),
        0xca => {
            println!(
                "{:02x}: JP\tZ,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xcb => {
            println!("{:02x}: PREFIX\t{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xcc => {
            println!(
                "{:02x}: CALL\tZ,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xcd => {
            println!(
                "{:02x}: CALL\t{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xce => {
            println!(
                "{:02x}: ADC\tA,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0xcf => println!("{:02x}: RST\t$08", code),
        0xd0 => println!("{:02x}: RET\tNC", code),
        0xd1 => println!("{:02x}: POP\tDE", code),
        0xd2 => {
            println!(
                "{:02x}: JP\tNC,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xd3 => println!("{:02x}: ---", code),
        0xd4 => {
            println!("{:02x}: CALL\tNC,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xd5 => println!("{:02x}: PUSH\tDE", code),
        0xd6 => {
            println!(
                "{:02x}: SUB\tA,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0xd7 => println!("{:02x}: RST\t$10", code),
        0xd8 => println!("{:02x}: RET\tC", code),
        0xd9 => println!("{:02x}: RETI", code),
        0xda => {
            println!(
                "{:02x}: JP\tC,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xdb => println!("{:02x}: ---", code),
        0xdc => {
            println!(
                "{:02x}: CALL\tC,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xdd => println!("{:02x}: ---", code),
        0xde => {
            println!("{:02x}: SBC\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xdf => println!("{:02x}: RST\t$18", code),
        0xe0 => println!("{:02x}: LDH\t[{:02x}],A", code, codebuffer[(pc as usize) + 2]),
        0xe1 => println!("{:02x}: POP\tHL", code),
        0xe2 => println!("{:02x}: LDH\t[C],A", code),
        0xe3 => println!("{:02x}: ---", code),
        0xe4 => println!("{:02x}: ---", code),
        0xe5 => println!("{:02x}: PUSH\tHL", code),
        0xe6 => {
            println!("{:02x}: AND\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xe7 => println!("{:02x}: RST\t$20", code),
        0xe8 => {
            println!("{:02x}: ADD\tSP,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xe9 => println!("{:02x}: JP\tHL", code),
        0xea => {
            println!(
                "{:02x}: LD\t[{:02x}{:02x}],A",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xeb => println!("{:02x}: ---", code),
        0xec => println!("{:02x}: ---", code),
        0xed => println!("{:02x}: ---", code),
        0xee => {
            println!("{:02x}: XOR\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xef => println!("{:02x}: RST\t$28", code),
        0xf0 => {
            println!("{:02x}: LDH\tA,[{:02x}]", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xf1 => println!("{:02x}: POP\tAF", code),
        0xf2 => println!("{:02x}: LDH\tA,[C]",code),
        0xf3 => println!("{:02x}: DI", code),
        0xf4 => println!("{:02x}: ---", code),
        0xf5 => println!("{:02x}: PUSH\tAF", code),
        0xf6 => {
            println!("{:02x}: OR\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xf7 => println!("{:02x}: RST\t$30", code),
        0xf8 => {
            println!("{:02x}: LD\tHL, SP + {:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xf9 => println!("{:02x}: LD\tSP,HL", code),
        0xfa => {
            println!(
                "{:02x}: LD\tA,[{:02x}{:02x}]",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xfb => println!("{:02x}: EI", code),
        0xfc => println!("{:02x}: ---", code),
        0xfd => println!("{:02x}: ---", code),
        0xfe => {
            println!("{:02x}: CP\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xff => println!("{:02x}: RST\t$38", code),
        &_ => {}
    }

    opbytes
}

fn main() {
    let filename = "invaders.rom";
    let rom: Vec<u8> = fs::read(filename).expect("Something	wrong");
    let mut pc: u16 = 0;

    let mut i = 0;

    while (pc as usize) < rom.len() {
        print!("{:04x}:\t", i);
        i += 1;
        pc += disassemble8080op(&rom, pc);
    }
}
