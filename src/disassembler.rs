use std::fs;

pub fn disassemble_op(codebuffer: &Vec<u8>, pc: u16) -> u16 {
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
            println!("{:02x}: LD\tB,{:02x}", code, codebuffer[(pc as usize) + 1]);
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
        0x09 => println!("{:02x}: ADD\tHL,BC", code),
        0x0a => println!("{:02x}: LD\tA,[BC]", code),
        0x0b => println!("{:02x}: DEC\tBC", code),
        0x0c => println!("{:02x}: INC\tC", code),
        0x0d => println!("{:02x}: DEC\tC", code),
        0x0e => {
            println!("{:02x}: LD\tC,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x0f => println!("{:02x}: RRCA", code),
        0x10 => {
            println!("{:02x}: STOP\t{:02x}", code, codebuffer[(pc as usize) + 1]);
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
            println!("{:02x}: LD\tD,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x17 => println!("{:02x}: RLA", code),
        0x18 => {
            println!("{:02x}: JR\t{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x19 => println!("{:02x}: ADD\tHL,DE", code),
        0x1a => println!("{:02x}: LD\tA,[DE]", code),
        0x1b => println!("{:02x}: DEC\tDE", code),
        0x1c => println!("{:02x}: INC\tE", code),
        0x1d => println!("{:02x}: DEC\tE", code),
        0x1e => {
            println!("{:02x}: LD\tE,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x1f => println!("{:02x}: RRA", code),
        0x20 => {
            println!("{:02x}: JR\tNZ,{:02x}", code, codebuffer[(pc as usize) + 1]);
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
            println!("{:02x}: LD\tH,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x27 => println!("{:02x}: DAA", code),
        0x28 => {
            println!("{:02x}: JR\tZ,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x29 => println!("{:02x}: ADD\tHL,HL", code),
        0x2a => println!("{:02x}: LD\tA,[HL+]", code),
        0x2b => println!("{:02x}: DEC\tHL", code),
        0x2c => println!("{:02x}: INC\tL", code),
        0x2d => println!("{:02x}: DEC\tL", code),
        0x2e => {
            println!("{:02x}: LD\tL,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x2f => println!("{:02x}: CPL", code),
        0x30 => {
            println!("{:02x}: JR\tNC,{:02x}", code, codebuffer[(pc as usize) + 1]);
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
            println!("{:02x}: JR\tC,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x39 => println!("{:02x}: ADD\tHL,SP", code),
        0x3a => println!("{:02x}: LD\tA,[HL-]", code),
        0x3b => println!("{:02x}: DEC\tSP", code),
        0x3c => println!("{:02x}: INC\tA", code),
        0x3d => println!("{:02x}: DEC\tA", code),
        0x3e => {
            println!("{:02x}: LD\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
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
            print!(
                "{:02x}: PREFIX\t{:02x}:\t",
                code,
                codebuffer[(pc as usize) + 1]
            );
            disassemble_prefix(codebuffer[(pc as usize) + 1]);
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
            println!("{:02x}: ADC\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
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
            println!(
                "{:02x}: CALL\tNC,{:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xd5 => println!("{:02x}: PUSH\tDE", code),
        0xd6 => {
            println!("{:02x}: SUB\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
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
        0xe0 => println!(
            "{:02x}: LDH\t[{:02x}],A",
            code,
            codebuffer[(pc as usize) + 2]
        ),
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
            println!(
                "{:02x}: ADD\tSP,{:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
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
            println!(
                "{:02x}: LDH\tA,[{:02x}]",
                code,
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 2;
        }
        0xf1 => println!("{:02x}: POP\tAF", code),
        0xf2 => println!("{:02x}: LDH\tA,[C]", code),
        0xf3 => println!("{:02x}: DI", code),
        0xf4 => println!("{:02x}: ---", code),
        0xf5 => println!("{:02x}: PUSH\tAF", code),
        0xf6 => {
            println!("{:02x}: OR\tA,{:02x}", code, codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xf7 => println!("{:02x}: RST\t$30", code),
        0xf8 => {
            println!(
                "{:02x}: LD\tHL, SP + {:02x}",
                code,
                codebuffer[(pc as usize) + 1]
            );
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
    }

    opbytes
}

fn disassemble_prefix(code: u8) {
    match code {
        0x00 => println!("RLC B"),
        0x01 => println!("RLC C"),
        0x02 => println!("RLC D"),
        0x03 => println!("RLC E"),
        0x04 => println!("RLC H"),
        0x05 => println!("RLC L"),
        0x06 => println!("RLC [HL]"),
        0x07 => println!("RLC A"),
        0x08 => println!("RRC B"),
        0x09 => println!("RRC C"),
        0x0a => println!("RRC D"),
        0x0b => println!("RRC E"),
        0x0c => println!("RRC H"),
        0x0d => println!("RRC L"),
        0x0e => println!("RRC [HL]"),
        0x0f => println!("RRC A"),
        0x10 => println!("RL B"),
        0x11 => println!("RL C"),
        0x12 => println!("RL D"),
        0x13 => println!("RL E"),
        0x14 => println!("RL H"),
        0x15 => println!("RL L"),
        0x16 => println!("RL [HL]"),
        0x17 => println!("RL A"),
        0x18 => println!("RR B"),
        0x19 => println!("RR C"),
        0x1a => println!("RR D"),
        0x1b => println!("RR E"),
        0x1c => println!("RR H"),
        0x1d => println!("RR L"),
        0x1e => println!("RR [HL]"),
        0x1f => println!("RR A"),
        0x20 => println!("SLA B"),
        0x21 => println!("SLA C"),
        0x22 => println!("SLA D"),
        0x23 => println!("SLA E"),
        0x24 => println!("SLA H"),
        0x25 => println!("SLA L"),
        0x26 => println!("SLA [HL]"),
        0x27 => println!("SLA A"),
        0x28 => println!("SRA B"),
        0x29 => println!("SRA C"),
        0x2a => println!("SRA D"),
        0x2b => println!("SRA E"),
        0x2c => println!("SRA H"),
        0x2d => println!("SRA L"),
        0x2e => println!("SRA [HL]"),
        0x2f => println!("SRA A"),
        0x30 => println!("SWAP B"),
        0x31 => println!("SWAP C"),
        0x32 => println!("SWAP D"),
        0x33 => println!("SWAP E"),
        0x34 => println!("SWAP H"),
        0x35 => println!("SWAP L"),
        0x36 => println!("SWAP [HL]"),
        0x37 => println!("SWAP A"),
        0x38 => println!("SRL B"),
        0x39 => println!("SRL C"),
        0x3a => println!("SRL D"),
        0x3b => println!("SRL E"),
        0x3c => println!("SRL H"),
        0x3d => println!("SRL L"),
        0x3e => println!("SRL [HL]"),
        0x3f => println!("SRL A"),
        0x40 => println!("BIT 0,B"),
        0x41 => println!("BIT 0,C"),
        0x42 => println!("BIT 0,D"),
        0x43 => println!("BIT 0,E"),
        0x44 => println!("BIT 0,H"),
        0x45 => println!("BIT 0,L"),
        0x46 => println!("BIT 0,[HL]"),
        0x47 => println!("BIT 0,A"),
        0x48 => println!("BIT 1,B"),
        0x49 => println!("BIT 1,C"),
        0x4a => println!("BIT 1,D"),
        0x4b => println!("BIT 1,E"),
        0x4c => println!("BIT 1,H"),
        0x4d => println!("BIT 1,L"),
        0x4e => println!("BIT 1,[HL]"),
        0x4f => println!("BIT 1,A"),
        0x50 => println!("BIT 2,B"),
        0x51 => println!("BIT 2,C"),
        0x52 => println!("BIT 2,D"),
        0x53 => println!("BIT 2,E"),
        0x54 => println!("BIT 2,H"),
        0x55 => println!("BIT 2,L"),
        0x56 => println!("BIT 2,[HL]"),
        0x57 => println!("BIT 2,A"),
        0x58 => println!("BIT 3,B"),
        0x59 => println!("BIT 3,C"),
        0x5a => println!("BIT 3,D"),
        0x5b => println!("BIT 3,E"),
        0x5c => println!("BIT 3,H"),
        0x5d => println!("BIT 3,L"),
        0x5e => println!("BIT 3,[HL]"),
        0x5f => println!("BIT 3,A"),
        0x60 => println!("BIT 4,B"),
        0x61 => println!("BIT 4,C"),
        0x62 => println!("BIT 4,D"),
        0x63 => println!("BIT 4,E"),
        0x64 => println!("BIT 4,H"),
        0x65 => println!("BIT 4,L"),
        0x66 => println!("BIT 4,[HL]"),
        0x67 => println!("BIT 4,A"),
        0x68 => println!("BIT 5,B"),
        0x69 => println!("BIT 5,C"),
        0x6a => println!("BIT 5,D"),
        0x6b => println!("BIT 5,E"),
        0x6c => println!("BIT 5,H"),
        0x6d => println!("BIT 5,L"),
        0x6e => println!("BIT 5,[HL]"),
        0x6f => println!("BIT 5,A"),
        0x70 => println!("BIT 6,B"),
        0x71 => println!("BIT 6,C"),
        0x72 => println!("BIT 6,D"),
        0x73 => println!("BIT 6,E"),
        0x74 => println!("BIT 6,H"),
        0x75 => println!("BIT 6,L"),
        0x76 => println!("BIT 6,[HL]"),
        0x77 => println!("BIT 6,A"),
        0x78 => println!("BIT 7,B"),
        0x79 => println!("BIT 7,C"),
        0x7a => println!("BIT 7,D"),
        0x7b => println!("BIT 7,E"),
        0x7c => println!("BIT 7,H"),
        0x7d => println!("BIT 7,L"),
        0x7e => println!("BIT 7,[HL]"),
        0x7f => println!("BIT 7,A"),
        0x80 => println!("RES 0,B"),
        0x81 => println!("RES 0,C"),
        0x82 => println!("RES 0,D"),
        0x83 => println!("RES 0,E"),
        0x84 => println!("RES 0,H"),
        0x85 => println!("RES 0,L"),
        0x86 => println!("RES 0,[HL]"),
        0x87 => println!("RES 0,A"),
        0x88 => println!("RES 1,B"),
        0x89 => println!("RES 1,C"),
        0x8a => println!("RES 1,D"),
        0x8b => println!("RES 1,E"),
        0x8c => println!("RES 1,H"),
        0x8d => println!("RES 1,L"),
        0x8e => println!("RES 1,[HL]"),
        0x8f => println!("RES 1,A"),
        0x90 => println!("RES 2,B"),
        0x91 => println!("RES 2,C"),
        0x92 => println!("RES 2,D"),
        0x93 => println!("RES 2,E"),
        0x94 => println!("RES 2,H"),
        0x95 => println!("RES 2,L"),
        0x96 => println!("RES 2,[HL]"),
        0x97 => println!("RES 2,A"),
        0x98 => println!("RES 3,B"),
        0x99 => println!("RES 3,C"),
        0x9a => println!("RES 3,D"),
        0x9b => println!("RES 3,E"),
        0x9c => println!("RES 3,H"),
        0x9d => println!("RES 3,L"),
        0x9e => println!("RES 3,[HL]"),
        0x9f => println!("RES 3,A"),
        0xa0 => println!("RES 4,B"),
        0xa1 => println!("RES 4,C"),
        0xa2 => println!("RES 4,D"),
        0xa3 => println!("RES 4,E"),
        0xa4 => println!("RES 4,H"),
        0xa5 => println!("RES 4,L"),
        0xa6 => println!("RES 4,[HL]"),
        0xa7 => println!("RES 4,A"),
        0xa8 => println!("RES 5,B"),
        0xa9 => println!("RES 5,C"),
        0xaa => println!("RES 5,D"),
        0xab => println!("RES 5,E"),
        0xac => println!("RES 5,H"),
        0xad => println!("RES 5,L"),
        0xae => println!("RES 5,[HL]"),
        0xaf => println!("RES 5,A"),
        0xb0 => println!("RES 6,B"),
        0xb1 => println!("RES 6,C"),
        0xb2 => println!("RES 6,D"),
        0xb3 => println!("RES 6,E"),
        0xb4 => println!("RES 6,H"),
        0xb5 => println!("RES 6,L"),
        0xb6 => println!("RES 6,[HL]"),
        0xb7 => println!("RES 6,A"),
        0xb8 => println!("RES 7,B"),
        0xb9 => println!("RES 7,C"),
        0xba => println!("RES 7,D"),
        0xbb => println!("RES 7,E"),
        0xbc => println!("RES 7,H"),
        0xbd => println!("RES 7,L"),
        0xbe => println!("RES 7,[HL]"),
        0xbf => println!("RES 7,A"),
        0xc0 => println!("SET 0,B"),
        0xc1 => println!("SET 0,C"),
        0xc2 => println!("SET 0,D"),
        0xc3 => println!("SET 0,E"),
        0xc4 => println!("SET 0,H"),
        0xc5 => println!("SET 0,L"),
        0xc6 => println!("SET 0,[HL]"),
        0xc7 => println!("SET 0,A"),
        0xc8 => println!("SET 1,B"),
        0xc9 => println!("SET 1,C"),
        0xca => println!("SET 1,D"),
        0xcb => println!("SET 1,E"),
        0xcc => println!("SET 1,H"),
        0xcd => println!("SET 1,L"),
        0xce => println!("SET 1,[HL]"),
        0xcf => println!("SET 1,A"),
        0xd0 => println!("SET 2,B"),
        0xd1 => println!("SET 2,C"),
        0xd2 => println!("SET 2,D"),
        0xd3 => println!("SET 2,E"),
        0xd4 => println!("SET 2,H"),
        0xd5 => println!("SET 2,L"),
        0xd6 => println!("SET 2,[HL]"),
        0xd7 => println!("SET 2,A"),
        0xd8 => println!("SET 3,B"),
        0xd9 => println!("SET 3,C"),
        0xda => println!("SET 3,D"),
        0xdb => println!("SET 3,E"),
        0xdc => println!("SET 3,H"),
        0xdd => println!("SET 3,L"),
        0xde => println!("SET 3,[HL]"),
        0xdf => println!("SET 3,A"),
        0xe0 => println!("SET 4,B"),
        0xe1 => println!("SET 4,C"),
        0xe2 => println!("SET 4,D"),
        0xe3 => println!("SET 4,E"),
        0xe4 => println!("SET 4,H"),
        0xe5 => println!("SET 4,L"),
        0xe6 => println!("SET 4,[HL]"),
        0xe7 => println!("SET 4,A"),
        0xe8 => println!("SET 5,B"),
        0xe9 => println!("SET 5,C"),
        0xea => println!("SET 5,D"),
        0xeb => println!("SET 5,E"),
        0xec => println!("SET 5,H"),
        0xed => println!("SET 5,L"),
        0xee => println!("SET 5,[HL]"),
        0xef => println!("SET 5,A"),
        0xf0 => println!("SET 6,B"),
        0xf1 => println!("SET 6,C"),
        0xf2 => println!("SET 6,D"),
        0xf3 => println!("SET 6,E"),
        0xf4 => println!("SET 6,H"),
        0xf5 => println!("SET 6,L"),
        0xf6 => println!("SET 6,[HL]"),
        0xf7 => println!("SET 6,A"),
        0xf8 => println!("SET 7,B"),
        0xf9 => println!("SET 7,C"),
        0xfa => println!("SET 7,D"),
        0xfb => println!("SET 7,E"),
        0xfc => println!("SET 7,H"),
        0xfd => println!("SET 7,L"),
        0xfe => println!("SET 7,[HL]"),
        0xff => println!("SET 7,A"),
    }
}

fn main() {
    let filename = "invaders.rom";
    let rom: Vec<u8> = fs::read(filename).expect("Something	wrong");
    let mut pc: u16 = 0;

    let mut i = 0;

    while (pc as usize) < rom.len() {
        print!("{:04x}:\t", i);
        i += 1;
        pc += disassemble_op(&rom, pc);
    }
}
