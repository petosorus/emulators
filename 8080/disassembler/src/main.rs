use std::fs;

fn disassemble8080op(codebuffer: &Vec<u8>, pc: usize) -> usize {
    let code: &u8 = codebuffer.get(pc).expect("oh	oh");
    let mut opbytes: usize = 1;

    match code {
        0x00 => println!("NOP"),
        0x01 => {
            println!(
                "LXI\tB,#${:02x}{:02x}",
                codebuffer[pc + 2],
                codebuffer[pc + 1]
            );
            opbytes = 3;
        }
        0x02 => println!("STAX\tB"),
        0x03 => println!("INX\tB"),
        0x04 => println!("INR\tB"),
        0x05 => println!("DCR\tB"),
        0x06 => {
            println!("MVI\tB,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        }
        0x07 => println!("RLC"),
        // 0x08
        0x09 => println!("DAD\tB"),
        0x0a => println!("LDAX\tB"),
        0x0b => println!("DCX\tB"),
        0x0c => println!("INCR\tC"),
        0x0d => println!("DCR\tC"),
        0x0e => {
            println!("MVI\tC,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0x0f => println!("RRC"),
        // 0x10
        0x11 => {
            println!("LXI\tD,#${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0x12 => { println!("STAX\tD") },
        0x13 => { println!("INX\tD") },
        0x14 => { println!("INR\tD") },
        0x15 => { println!("DCR\tD") },
        0x16 => {
            println!("MVI\tD,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0x17 => { println!("RAL") },
        // 0x18
        0x19 => { println!("DAD\tD") },
        0x1a => { println!("LDAX\tD") },
        0x1b => { println!("DCX\tD") },
        0x1c => { println!("INR\tE") },
        0x1d => { println!("DCR\tE") },
        0x1e => {
            println!("MVI\tE,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0x1f => { println!("RAR") },
        0x20 => { println!("RIM") },
        0x21 => {
            println!("LXI\tH,#${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0x22 => {
            println!("SHLD\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0x23 => { println!("INX\tH") },
        0x24 => { println!("INR\tH") },
        0x25 => { println!("DCR\tH") },
        0x26 => {
            println!("MVI\tH,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0x27 => { println!("DAA") },
        // 0x28
        0x29 => { println!("DAD\tH") },
        0x2a => {
            println!("LHLD\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0x2b => { println!("DCX\tH") },
        0x2c => { println!("INR\tL") },
        0x2d => { println!("DCR\tL") },
        0x2e => {
            println!("MVI\tL,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0x2f => { println!("CMA") },
        0x30 => { println!("SIM") },
        0x31 => {
            println!("LXI\tSP\t#${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0x32 => {
            println!("STA\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0x33 => { println!("INX\tSP") },
        0x34 => { println!("INR\tM") },
        0x35 => { println!("DCR\tM") },
        0x36 => {
            println!("MVI\tM,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0x37 => { println!("STC") },
        // 0x38
        0x39 => { println!("DAD\tSP") },
        0x3a => {
            println!("LDA\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0x3b => { println!("DCX\tSP") },
        0x3c => { println!("INR\tA") },
        0x3d => { println!("DCR\tA") },
        0x3e => {
            println!("MVI\tA,#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0x3f => { println!("CMC") },
        0x40 => { println!("MOV\tB,B") },
        0x41 => { println!("MOV\tB,C") },
        0x42 => { println!("MOV\tB,D") },
        0x43 => { println!("MOV\tB,E") },
        0x44 => { println!("MOV\tB,H") },
        0x45 => { println!("MOV\tB,L") },
        0x46 => { println!("MOV\tB,M") },
        0x47 => { println!("MOV\tB,A") },
        0x48 => { println!("MOV\tC,B") },
        0x49 => { println!("MOV\tC,C") },
        0x4a => { println!("MOV\tC,D") },
        0x4b => { println!("MOV\tC,E") },
        0x4c => { println!("MOV\tC,H") },
        0x4d => { println!("MOV\tC,L") },
        0x4e => { println!("MOV\tC,M") },
        0x4f => { println!("MOV\tC,A") },
        0x50 => { println!("MOV\tD,B") },
        0x51 => { println!("MOV\tD,C") },
        0x52 => { println!("MOV\tD,D") },
        0x53 => { println!("MOV\tD,E") },
        0x54 => { println!("MOV\tD,H") },
        0x55 => { println!("MOV\tD,L") },
        0x56 => { println!("MOV\tD,M") },
        0x57 => { println!("MOV\tD,A") },
        0x58 => { println!("MOV\tE,B") },
        0x59 => { println!("MOV\tE,C") },
        0x5a => { println!("MOV\tE,D") },
        0x5b => { println!("MOV\tE,E") },
        0x5c => { println!("MOV\tE,H") },
        0x5d => { println!("MOV\tE,L") },
        0x5e => { println!("MOV\tE,M") },
        0x5f => { println!("MOV\tE,A") },
        0x60 => { println!("MOV\tH,B") },
        0x61 => { println!("MOV\tH,C") },
        0x62 => { println!("MOV\tH,D") },
        0x63 => { println!("MOV\tH,E") },
        0x64 => { println!("MOV\tH,H") },
        0x65 => { println!("MOV\tH,L") },
        0x66 => { println!("MOV\tH,M") },
        0x67 => { println!("MOV\tH,A") },
        0x68 => { println!("MOV\tL,B") },
        0x69 => { println!("MOV\tL,C") },
        0x6a => { println!("MOV\tL,D") },
        0x6b => { println!("MOV\tL,E") },
        0x6c => { println!("MOV\tL,H") },
        0x6d => { println!("MOV\tL,L") },
        0x6e => { println!("MOV\tL,M") },
        0x6f => { println!("MOV\tL,A") },
        0x70 => { println!("MOV\tM,B") },
        0x71 => { println!("MOV\tM,C") },
        0x72 => { println!("MOV\tM,D") },
        0x73 => { println!("MOV\tM,E") },
        0x74 => { println!("MOV\tM,H") },
        0x75 => { println!("MOV\tM,L") },
        0x76 => { println!("HLT") },
        0x77 => { println!("MOV\tM,A") },
        0x78 => { println!("MOV\tA,B") },
        0x79 => { println!("MOV\tA,C") },
        0x7a => { println!("MOV\tA,D") },
        0x7b => { println!("MOV\tA,E") },
        0x7c => { println!("MOV\tA,H") },
        0x7d => { println!("MOV\tA,L") },
        0x7e => { println!("MOV\tA,M") },
        0x7f => { println!("MOV\tA,A") },
        0x80 => { println!("ADD\tB") },
        0x81 => { println!("ADD\tC") },
        0x82 => { println!("ADD\tD") },
        0x83 => { println!("ADD\tE") },
        0x84 => { println!("ADD\tH") },
        0x85 => { println!("ADD\tL") },
        0x86 => { println!("ADD\tM") },
        0x87 => { println!("ADD\tA") },
        0x88 => { println!("ADC\tB") },
        0x89 => { println!("ADC\tC") },
        0x8a => { println!("ADC\tD") },
        0x8b => { println!("ADC\tE") },
        0x8c => { println!("ADC\tH") },
        0x8d => { println!("ADC\tL") },
        0x8e => { println!("ADC\tM") },
        0x8f => { println!("ADC\tA") },
        0x90 => { println!("SUB\tB") },
        0x91 => { println!("SUB\tC") },
        0x92 => { println!("SUB\tD") },
        0x93 => { println!("SUB\tE") },
        0x94 => { println!("SUB\tH") },
        0x95 => { println!("SUB\tL") },
        0x96 => { println!("SUB\tM") },
        0x97 => { println!("SUB\tA") },
        0x98 => { println!("SBB\tB") },
        0x99 => { println!("SBB\tC") },
        0x9a => { println!("SBB\tD") },
        0x9b => { println!("SBB\tE") },
        0x9c => { println!("SBB\tH") },
        0x9d => { println!("SBB\tL") },
        0x9e => { println!("SBB\tM") },
        0x9f => { println!("SBB\tA") },
        0xa0 => { println!("ANA\tB") },
        0xa1 => { println!("ANA\tC") },
        0xa2 => { println!("ANA\tD") },
        0xa3 => { println!("ANA\tE") },
        0xa4 => { println!("ANA\tH") },
        0xa5 => { println!("ANA\tL") },
        0xa6 => { println!("ANA\tM") },
        0xa7 => { println!("ANA\tA") },
        0xa8 => { println!("XRA\tB") },
        0xa9 => { println!("XRA\tC") },
        0xaa => { println!("XRA\tD") },
        0xab => { println!("XRA\tE") },
        0xac => { println!("XRA\tH") },
        0xad => { println!("XRA\tL") },
        0xae => { println!("XRA\tM") },
        0xaf => { println!("XRA\tA") },
        0xb0 => { println!("ORA\tB") },
        0xb1 => { println!("ORA\tC") },
        0xb2 => { println!("ORA\tD") },
        0xb3 => { println!("ORA\tE") },
        0xb4 => { println!("ORA\tH") },
        0xb5 => { println!("ORA\tL") },
        0xb6 => { println!("ORA\tM") },
        0xb7 => { println!("ORA\tA") },
        0xb8 => { println!("CMP\tB") },
        0xb9 => { println!("CMP\tC") },
        0xba => { println!("CMP\tD") },
        0xbb => { println!("CMP\tE") },
        0xbc => { println!("CMP\tH") },
        0xbd => { println!("CMP\tL") },
        0xbe => { println!("CMP\tM") },
        0xbf => { println!("CMP\tA") },
        0xc0 => { println!("RNZ") },
        0xc1 => { println!("POP\tB") },
        0xc2 => { 
            println!("JNZ\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xc3 => {
            println!("JMP\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xc4 => {
            println!("CNZ\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xc5 => { println!("PUSH\tB") },
        0xc6 => {
            println!("ADI\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xc7 => { println!("RST\t0") },
        0xc8 => { println!("RZ") },
        0xc9 => { println!("RET") },
        0xca => {
            println!("JZ\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        // 0xcb
        0xcc => {
            println!("CZ\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xcd => {
            println!("CALL\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xce => {
            println!("ACI\t#${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xcf => { println!("RST\t1") },
        0xd0 => { println!("RNC") },
        0xd1 => { println!("POP\tD") },
        0xd2 => {
            println!("JNC\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xd3 => {
            println!("OUT\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xd4 => {
            println!("ADI\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xd5 => { println!("PUSH\tD") },
        0xd6 => {
            println!("CNC\t#${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xd7 => { println!("RST\t2") },
        0xd8 => { println!("RC") },
        // 0xd9
        0xda => {
            println!("JC\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xdb => {
            println!("IN\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xdc => {
            println!("CC\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        // 0xdd
        0xde => {
            println!("SBI\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xdf => { println!("RST\t3") },
        0xe0 => { println!("RPO") },
        0xe1 => { println!("POP\tH") },
        0xe2 => {
            println!("JPO\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xe3 => { println!("XTHL") },
        0xe4 => {
            println!("CPO\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xe5 => { println!("PUSH\tH") },
        0xe6 => {
            println!("ANI\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xe7 => { println!("RST\t4") },
        0xe8 => { println!("RPE") },
        0xe9 => { println!("PCHL") },
        0xea => {
            println!("JPE\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xeb => { println!("XCHG") },
        0xec => {
            println!("CPE\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        // 0xed
        0xee => {
            println!("XPI\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xef => { println!("RST\t5") },
        0xf0 => { println!("RP") },
        0xf1 => { println!("POP\tPSW") },
        0xf2 => {
            println!("JP\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xf3 => { println!("DI") },
        0xf4 => {
            println!("CP\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xf5 => { println!("PUSH\tPSW") },
        0xf6 => {
            println!("ORI\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xf7 => { println!("RST\t6") },
        0xf8 => { println!("RM") },
        0xf9 => { println!("SPHL") },
        0xfa => {
            println!("JM\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        0xfb => { println!("EI") },
        0xfc => {
            println!("CM\t${:02x}{:02x}", codebuffer[pc + 2], codebuffer[pc + 1]);
            opbytes = 3;
        },
        // 0xfd
        0xfe => {
            println!("CPI\t#${:02x}", codebuffer[pc + 1]);
            opbytes = 2;
        },
        0xff => { println!("RST\t7") },
        &_ => {}
    }

    opbytes
}

fn main() {

    let filename = "invaders";
    let rom: Vec<u8> = fs::read(filename)
        .expect("Something	wrong");        
    let mut pc: usize = 0;

    while pc < rom.len() {
        pc += disassemble8080op(&rom, pc);
    }
}
