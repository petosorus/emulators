use std::fs;

pub fn disassemble8080op(codebuffer: &Vec<u8>, pc: u16) -> u16 {
    let code: &u8 = codebuffer.get(pc as usize).expect("oh	oh");
    let mut opbytes: u16 = 1;

    match code {
        0x00 => println!("{:02x}: NOP", code),
        0x01 => {
            println!(
                "{:02x}: LXI\tB,#${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x02 => println!("{:02x}: STAX\tB", code),
        0x03 => println!("{:02x}: INX\tB", code),
        0x04 => println!("{:02x}: INR\tB", code),
        0x05 => println!("{:02x}: DCR\tB", code),
        0x06 => {
            println!("{:02x}: MVI\tB,#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x07 => println!("{:02x}: RLC", code),
        // 0x08
        0x09 => println!("{:02x}: DAD\tB", code),
        0x0a => println!("{:02x}: LDAX\tB", code),
        0x0b => println!("{:02x}: DCX\tB", code),
        0x0c => println!("{:02x}: INCR\tC", code),
        0x0d => println!("{:02x}: DCR\tC", code),
        0x0e => {
            println!("{:02x}: MVI\tC,#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x0f => println!("{:02x}: RRC", code),
        // 0x10
        0x11 => {
            println!(
                "{:02x}: LXI\tD,#${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x12 => println!("{:02x}: STAX\tD", code),
        0x13 => println!("{:02x}: INX\tD", code),
        0x14 => println!("{:02x}: INR\tD", code),
        0x15 => println!("{:02x}: DCR\tD", code),
        0x16 => {
            println!("{:02x}: MVI\tD,#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x17 => println!("{:02x}: RAL", code),
        // 0x18
        0x19 => println!("{:02x}: DAD\tD", code),
        0x1a => println!("{:02x}: LDAX\tD", code),
        0x1b => println!("{:02x}: DCX\tD", code),
        0x1c => println!("{:02x}: INR\tE", code),
        0x1d => println!("{:02x}: DCR\tE", code),
        0x1e => {
            println!("{:02x}: MVI\tE,#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x1f => println!("{:02x}: RAR", code),
        0x20 => println!("{:02x}: RIM", code),
        0x21 => {
            println!(
                "{:02x}: LXI\tH,#${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x22 => {
            println!(
                "{:02x}: SHLD\t${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x23 => println!("{:02x}: INX\tH", code),
        0x24 => println!("{:02x}: INR\tH", code),
        0x25 => println!("{:02x}: DCR\tH", code),
        0x26 => {
            println!("{:02x}: MVI\tH,#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x27 => println!("{:02x}: DAA", code),
        // 0x28
        0x29 => println!("{:02x}: DAD\tH", code),
        0x2a => {
            println!(
                "{:02x}: LHLD\t${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x2b => println!("{:02x}: DCX\tH", code),
        0x2c => println!("{:02x}: INR\tL", code),
        0x2d => println!("{:02x}: DCR\tL", code),
        0x2e => {
            println!("{:02x}: MVI\tL,#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x2f => println!("{:02x}: CMA", code),
        0x30 => println!("{:02x}: SIM", code),
        0x31 => {
            println!(
                "{:02x}: LXI\tSP\t#${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0x32 => {
            println!("{:02x}: STA\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0x33 => println!("{:02x}: INX\tSP", code),
        0x34 => println!("{:02x}: INR\tM", code),
        0x35 => println!("{:02x}: DCR\tM", code),
        0x36 => {
            println!("{:02x}: MVI\tM,#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x37 => println!("{:02x}: STC", code),
        // 0x38
        0x39 => println!("{:02x}: DAD\tSP", code),
        0x3a => {
            println!("{:02x}: LDA\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0x3b => println!("{:02x}: DCX\tSP", code),
        0x3c => println!("{:02x}: INR\tA", code),
        0x3d => println!("{:02x}: DCR\tA", code),
        0x3e => {
            println!("{:02x}: MVI\tA,#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0x3f => println!("{:02x}: CMC", code),
        0x40 => println!("{:02x}: MOV\tB,B", code),
        0x41 => println!("{:02x}: MOV\tB,C", code),
        0x42 => println!("{:02x}: MOV\tB,D", code),
        0x43 => println!("{:02x}: MOV\tB,E", code),
        0x44 => println!("{:02x}: MOV\tB,H", code),
        0x45 => println!("{:02x}: MOV\tB,L", code),
        0x46 => println!("{:02x}: MOV\tB,M", code),
        0x47 => println!("{:02x}: MOV\tB,A", code),
        0x48 => println!("{:02x}: MOV\tC,B", code),
        0x49 => println!("{:02x}: MOV\tC,C", code),
        0x4a => println!("{:02x}: MOV\tC,D", code),
        0x4b => println!("{:02x}: MOV\tC,E", code),
        0x4c => println!("{:02x}: MOV\tC,H", code),
        0x4d => println!("{:02x}: MOV\tC,L", code),
        0x4e => println!("{:02x}: MOV\tC,M", code),
        0x4f => println!("{:02x}: MOV\tC,A", code),
        0x50 => println!("{:02x}: MOV\tD,B", code),
        0x51 => println!("{:02x}: MOV\tD,C", code),
        0x52 => println!("{:02x}: MOV\tD,D", code),
        0x53 => println!("{:02x}: MOV\tD,E", code),
        0x54 => println!("{:02x}: MOV\tD,H", code),
        0x55 => println!("{:02x}: MOV\tD,L", code),
        0x56 => println!("{:02x}: MOV\tD,M", code),
        0x57 => println!("{:02x}: MOV\tD,A", code),
        0x58 => println!("{:02x}: MOV\tE,B", code),
        0x59 => println!("{:02x}: MOV\tE,C", code),
        0x5a => println!("{:02x}: MOV\tE,D", code),
        0x5b => println!("{:02x}: MOV\tE,E", code),
        0x5c => println!("{:02x}: MOV\tE,H", code),
        0x5d => println!("{:02x}: MOV\tE,L", code),
        0x5e => println!("{:02x}: MOV\tE,M", code),
        0x5f => println!("{:02x}: MOV\tE,A", code),
        0x60 => println!("{:02x}: MOV\tH,B", code),
        0x61 => println!("{:02x}: MOV\tH,C", code),
        0x62 => println!("{:02x}: MOV\tH,D", code),
        0x63 => println!("{:02x}: MOV\tH,E", code),
        0x64 => println!("{:02x}: MOV\tH,H", code),
        0x65 => println!("{:02x}: MOV\tH,L", code),
        0x66 => println!("{:02x}: MOV\tH,M", code),
        0x67 => println!("{:02x}: MOV\tH,A", code),
        0x68 => println!("{:02x}: MOV\tL,B", code),
        0x69 => println!("{:02x}: MOV\tL,C", code),
        0x6a => println!("{:02x}: MOV\tL,D", code),
        0x6b => println!("{:02x}: MOV\tL,E", code),
        0x6c => println!("{:02x}: MOV\tL,H", code),
        0x6d => println!("{:02x}: MOV\tL,L", code),
        0x6e => println!("{:02x}: MOV\tL,M", code),
        0x6f => println!("{:02x}: MOV\tL,A", code),
        0x70 => println!("{:02x}: MOV\tM,B", code),
        0x71 => println!("{:02x}: MOV\tM,C", code),
        0x72 => println!("{:02x}: MOV\tM,D", code),
        0x73 => println!("{:02x}: MOV\tM,E", code),
        0x74 => println!("{:02x}: MOV\tM,H", code),
        0x75 => println!("{:02x}: MOV\tM,L", code),
        0x76 => println!("{:02x}: HLT", code),
        0x77 => println!("{:02x}: MOV\tM,A", code),
        0x78 => println!("{:02x}: MOV\tA,B", code),
        0x79 => println!("{:02x}: MOV\tA,C", code),
        0x7a => println!("{:02x}: MOV\tA,D", code),
        0x7b => println!("{:02x}: MOV\tA,E", code),
        0x7c => println!("{:02x}: MOV\tA,H", code),
        0x7d => println!("{:02x}: MOV\tA,L", code),
        0x7e => println!("{:02x}: MOV\tA,M", code),
        0x7f => println!("{:02x}: MOV\tA,A", code),
        0x80 => println!("{:02x}: ADD\tB", code),
        0x81 => println!("{:02x}: ADD\tC", code),
        0x82 => println!("{:02x}: ADD\tD", code),
        0x83 => println!("{:02x}: ADD\tE", code),
        0x84 => println!("{:02x}: ADD\tH", code),
        0x85 => println!("{:02x}: ADD\tL", code),
        0x86 => println!("{:02x}: ADD\tM", code),
        0x87 => println!("{:02x}: ADD\tA", code),
        0x88 => println!("{:02x}: ADC\tB", code),
        0x89 => println!("{:02x}: ADC\tC", code),
        0x8a => println!("{:02x}: ADC\tD", code),
        0x8b => println!("{:02x}: ADC\tE", code),
        0x8c => println!("{:02x}: ADC\tH", code),
        0x8d => println!("{:02x}: ADC\tL", code),
        0x8e => println!("{:02x}: ADC\tM", code),
        0x8f => println!("{:02x}: ADC\tA", code),
        0x90 => println!("{:02x}: SUB\tB", code),
        0x91 => println!("{:02x}: SUB\tC", code),
        0x92 => println!("{:02x}: SUB\tD", code),
        0x93 => println!("{:02x}: SUB\tE", code),
        0x94 => println!("{:02x}: SUB\tH", code),
        0x95 => println!("{:02x}: SUB\tL", code),
        0x96 => println!("{:02x}: SUB\tM", code),
        0x97 => println!("{:02x}: SUB\tA", code),
        0x98 => println!("{:02x}: SBB\tB", code),
        0x99 => println!("{:02x}: SBB\tC", code),
        0x9a => println!("{:02x}: SBB\tD", code),
        0x9b => println!("{:02x}: SBB\tE", code),
        0x9c => println!("{:02x}: SBB\tH", code),
        0x9d => println!("{:02x}: SBB\tL", code),
        0x9e => println!("{:02x}: SBB\tM", code),
        0x9f => println!("{:02x}: SBB\tA", code),
        0xa0 => println!("{:02x}: ANA\tB", code),
        0xa1 => println!("{:02x}: ANA\tC", code),
        0xa2 => println!("{:02x}: ANA\tD", code),
        0xa3 => println!("{:02x}: ANA\tE", code),
        0xa4 => println!("{:02x}: ANA\tH", code),
        0xa5 => println!("{:02x}: ANA\tL", code),
        0xa6 => println!("{:02x}: ANA\tM", code),
        0xa7 => println!("{:02x}: ANA\tA", code),
        0xa8 => println!("{:02x}: XRA\tB", code),
        0xa9 => println!("{:02x}: XRA\tC", code),
        0xaa => println!("{:02x}: XRA\tD", code),
        0xab => println!("{:02x}: XRA\tE", code),
        0xac => println!("{:02x}: XRA\tH", code),
        0xad => println!("{:02x}: XRA\tL", code),
        0xae => println!("{:02x}: XRA\tM", code),
        0xaf => println!("{:02x}: XRA\tA", code),
        0xb0 => println!("{:02x}: ORA\tB", code),
        0xb1 => println!("{:02x}: ORA\tC", code),
        0xb2 => println!("{:02x}: ORA\tD", code),
        0xb3 => println!("{:02x}: ORA\tE", code),
        0xb4 => println!("{:02x}: ORA\tH", code),
        0xb5 => println!("{:02x}: ORA\tL", code),
        0xb6 => println!("{:02x}: ORA\tM", code),
        0xb7 => println!("{:02x}: ORA\tA", code),
        0xb8 => println!("{:02x}: CMP\tB", code),
        0xb9 => println!("{:02x}: CMP\tC", code),
        0xba => println!("{:02x}: CMP\tD", code),
        0xbb => println!("{:02x}: CMP\tE", code),
        0xbc => println!("{:02x}: CMP\tH", code),
        0xbd => println!("{:02x}: CMP\tL", code),
        0xbe => println!("{:02x}: CMP\tM", code),
        0xbf => println!("{:02x}: CMP\tA", code),
        0xc0 => println!("{:02x}: RNZ", code),
        0xc1 => println!("{:02x}: POP\tB", code),
        0xc2 => {
            println!("{:02x}: JNZ\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xc3 => {
            println!("{:02x}: JMP\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xc4 => {
            println!("{:02x}: CNZ\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xc5 => println!("{:02x}: PUSH\tB", code),
        0xc6 => {
            println!("{:02x}: ADI\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xc7 => println!("{:02x}: RST\t0", code),
        0xc8 => println!("{:02x}: RZ", code),
        0xc9 => println!("{:02x}: RET", code),
        0xca => {
            println!("{:02x}: JZ\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        // 0xcb
        0xcc => {
            println!("{:02x}: CZ\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xcd => {
            println!(
                "{:02x}: CALL\t${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xce => {
            println!(
                "{:02x}: ACI\t#${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xcf => println!("{:02x}: RST\t1", code),
        0xd0 => println!("{:02x}: RNC", code),
        0xd1 => println!("{:02x}: POP\tD", code),
        0xd2 => {
            println!("{:02x}: JNC\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xd3 => {
            println!("{:02x}: OUT\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xd4 => {
            println!("{:02x}: ADI\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xd5 => println!("{:02x}: PUSH\tD", code),
        0xd6 => {
            println!(
                "{:02x}: CNC\t#${:02x}{:02x}",
                code,
                codebuffer[(pc as usize) + 2],
                codebuffer[(pc as usize) + 1]
            );
            opbytes = 3;
        }
        0xd7 => println!("{:02x}: RST\t2", code),
        0xd8 => println!("{:02x}: RC", code),
        // 0xd9
        0xda => {
            println!("{:02x}: JC\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xdb => {
            println!("{:02x}: IN\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xdc => {
            println!("{:02x}: CC\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        // 0xdd
        0xde => {
            println!("{:02x}: SBI\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xdf => println!("{:02x}: RST\t3", code),
        0xe0 => println!("{:02x}: RPO", code),
        0xe1 => println!("{:02x}: POP\tH", code),
        0xe2 => {
            println!("{:02x}: JPO\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xe3 => println!("{:02x}: XTHL", code),
        0xe4 => {
            println!("{:02x}: CPO\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xe5 => println!("{:02x}: PUSH\tH", code),
        0xe6 => {
            println!("{:02x}: ANI\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xe7 => println!("{:02x}: RST\t4", code),
        0xe8 => println!("{:02x}: RPE", code),
        0xe9 => println!("{:02x}: PCHL", code),
        0xea => {
            println!("{:02x}: JPE\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xeb => println!("{:02x}: XCHG", code),
        0xec => {
            println!("{:02x}: CPE\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        // 0xed
        0xee => {
            println!("{:02x}: XPI\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xef => println!("{:02x}: RST\t5", code),
        0xf0 => println!("{:02x}: RP", code),
        0xf1 => println!("{:02x}: POP\tPSW", code),
        0xf2 => {
            println!("{:02x}: JP\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xf3 => println!("{:02x}: DI", code),
        0xf4 => {
            println!("{:02x}: CP\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xf5 => println!("{:02x}: PUSH\tPSW", code),
        0xf6 => {
            println!("{:02x}: ORI\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xf7 => println!("{:02x}: RST\t6", code),
        0xf8 => println!("{:02x}: RM", code),
        0xf9 => println!("{:02x}: SPHL", code),
        0xfa => {
            println!("{:02x}: JM\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        0xfb => println!("{:02x}: EI", code),
        0xfc => {
            println!("{:02x}: CM\t${:02x}{:02x}", code,codebuffer[(pc as usize) + 2], codebuffer[(pc as usize) + 1]);
            opbytes = 3;
        }
        // 0xfd
        0xfe => {
            println!("{:02x}: CPI\t#${:02x}", code,codebuffer[(pc as usize) + 1]);
            opbytes = 2;
        }
        0xff => println!("{:02x}: RST\t7", code),
        &_ => {}
    }

    opbytes
}

fn main() {
    let filename = "invaders.rom";
    let rom: Vec<u8> = fs::read(filename).expect("Something	wrong");
    let mut pc: u16 = 0;

    while (pc as usize) < rom.len() {
        pc += disassemble8080op(&rom, pc);
    }
}
