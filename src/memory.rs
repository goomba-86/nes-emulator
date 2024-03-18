pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }

    fn write_byte(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_byte_is_written_to_memory() {
        let mut memory = Memory {
            memory: [0; 0xFFFF],
        };

        memory.write_byte(5, 4);
        assert_eq!(4, memory.read_byte(5));
    }
}
