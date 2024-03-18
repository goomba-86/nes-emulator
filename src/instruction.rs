pub struct Instruction {
    pub op_code: OpCode,
    pub short_value: Option<u8>,
    pub long_value: Option<u16>,
    pub bytes: u8,
    pub cycles: u8,
}

pub enum OpCode {
    Brk = 0x00,
    AslZpx = 0x16,
    AndImmediate = 0x29,
}

pub fn parse_instruction(prg_rom: Vec<u8>, stack_pointer: usize) -> Instruction {
    let instruction_code = prg_rom[stack_pointer];
    match instruction_code {
        0x00 => {
            return parse_brk();
        }
        0x16 => {
            return parse_asl_zpx(prg_rom[stack_pointer + 1]);
        }
        _ => panic!("Unsupported Instruction code: {:02X}", instruction_code),
    };
}

fn parse_asl_zpx(value: u8) -> Instruction {
    Instruction {
        op_code: OpCode::AslZpx,
        short_value: Some(value),
        long_value: None,
        bytes: 2,
        cycles: 6,
    }
}

fn parse_brk() -> Instruction {
    Instruction {
        op_code: OpCode::Brk,
        short_value: None,
        long_value: None,
        bytes: 1,
        cycles: 7,
    }
}
