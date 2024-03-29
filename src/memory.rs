pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    fn new(prg_rom: Vec<u8>) -> Memory {
        let mut memory = Memory {
            memory: [0; 0xFFFF],
        };
        memory.memory[0x8000..(0x8000 + prg_rom.len())].copy_from_slice(&prg_rom[..]);

        if prg_rom.len() < 32 * 1024 {
            memory.memory[0xC000..(0xC000 + prg_rom.len())].copy_from_slice(&prg_rom[..]);
        }
        memory
    }

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
