mod ines_parser;
mod instruction;
mod p6502;

use crate::instruction::{Instruction, OpCode};
use crate::p6502::Memory;
use crate::p6502::P6502;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }

    if args.len() != 2 {
        println!("Invalid number of arguments.");
        return;
    }

    let ines_content = ines_parser::parse(&args[1]).unwrap();

    let mut memory = Memory { memory: [0; 0xFFF] };
    let mut p6502 = P6502::new();

    let immediate_and = Instruction {
        op_code: OpCode::AndImmediate,
        short_value: Some(0x08),
        long_value: None,
        bytes: 2,
        cycles: 2,
    };

    p6502.execute_instruction(immediate_and, &mut memory);
}
