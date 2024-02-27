mod p6502;
use crate::p6502::Instruction;
use crate::p6502::Memory;
use crate::p6502::P6502;

fn main() {
    let mut memory = Memory { memory: [0; 0xFFF] };
    let mut p6502 = P6502::new();

    let immediate_and = Instruction {
        op_code: p6502::OpCode::AndImmediate,
        short_value: Some(0x08),
        long_value: None,
        bytes: 2,
        cycles: 2,
    };

    p6502.execute_instruction(immediate_and, &mut memory);
}
