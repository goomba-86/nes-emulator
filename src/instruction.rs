pub struct Instruction {
    pub op_code: OpCode,
    pub short_value: Option<u8>,
    pub long_value: Option<u16>,
    pub bytes: u8,
    pub cycles: u8,
}

pub enum OpCode {
    AndImmediate = 0x29,
}
