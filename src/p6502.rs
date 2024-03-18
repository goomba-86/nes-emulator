use super::instruction::{Instruction, OpCode};
use super::memory::Memory;

pub struct P6502 {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub processor_status: u8,
}

fn bit_7_set(value: u8) -> bool {
    value & 0b10000000 > 0
}

impl P6502 {
    pub fn new() -> P6502 {
        P6502 {
            program_counter: 0,
            stack_pointer: 0xFF,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            processor_status: 0,
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction, memory: &mut Memory) {
        match instruction.op_code {
            OpCode::AndImmediate => {
                let value = instruction.short_value.unwrap();
                self.accumulator = self.accumulator & value;

                if self.accumulator == 0 {
                    self.set_zero_flag();
                }

                if bit_7_set(self.accumulator) {
                    self.set_negative_flag();
                }
            }
            _ => panic!("Unsupported Opcode "),
        }
    }

    pub fn set_negative_flag(&mut self) {
        self.processor_status = 0b0000010 | self.processor_status;
    }

    pub fn set_zero_flag(&mut self) {
        self.processor_status = 0b01000000 | self.processor_status;
    }

    pub fn set_carry_flag(&mut self) {
        self.processor_status = 0b10000000 | self.processor_status;
    }

    pub fn carry_flag_set(&self) -> bool {
        self.processor_status & 0b10000000 > 0
    }

    pub fn clear_carry_flag(&mut self) {
        self.processor_status = 0b01111111 & self.processor_status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_carry_flag_is_set_correctly() {
        let mut p6502 = P6502::new();
        p6502.set_carry_flag();
        assert_eq!(p6502.processor_status, 0b10000000);
    }

    #[test]
    fn test_that_carry_flag_is_status_is_read_correctly() {
        let mut p6502 = P6502::new();
        assert!(!p6502.carry_flag_set());
        p6502.set_carry_flag();
        assert!(p6502.carry_flag_set());
    }

    #[test]
    fn test_that_carry_flag_is_cleared_correctly() {
        let mut p6502 = P6502::new();
        p6502.processor_status = 0xFF;
        p6502.clear_carry_flag();
        assert_eq!(p6502.processor_status, 0b01111111);
    }

    #[test]
    fn test_that_zero_flag_is_set_correctly() {
        let mut p6502 = P6502::new();
        p6502.set_zero_flag();
        assert_eq!(p6502.processor_status, 0b01000000);
    }

    #[test]
    fn test_that_negative_flag_is_set_correctly() {
        let mut p6502 = P6502::new();
        p6502.set_negative_flag();
        assert_eq!(p6502.processor_status, 0b0000010);
    }
}
