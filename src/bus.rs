// Mirrors of $0000-$07FF
const RAM_ADDRESS_RANGE: (usize, usize) = (0, 0x1FFF);
// Mirrors of $2000-$2007 (repeats every 8 bytes)
const NES_PPU_REGISTERS: (usize, usize) = (0x2000, 0x3FFF);
//NES APU and I/O registers
const APU_AND_IO: (usize, usize) = (0x4000, 0x4017);
// Unmapped. Available for cartridge use.
// $6000-$7FFF usually for cartridge RAM when present.
// $8000-$FFFF usually for cartridge ROM and mapper registers
const CARTRIDGE: (usize, usize) = (0x4020, 0xFFFF);

struct Bus {
    prg_rom: Vec<u8>,
    ram: Vec<u8>,
    mapper: u16,
}

impl Bus {
    pub fn new(mapper: u16, prg_rom: Vec<u8>) -> Bus {
        if mapper != 0 {
            panic!("Mapper index {} is not supported", mapper);
        }

        Bus {
            prg_rom,
            ram: vec![0; 0x0800],
            mapper,
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        if address >= 0x8000 {
            panic!("Trying to write to PRG ROM area with address {}", address);
        }

        match address {
            0..=0x1FFF => {
                let actual_address = address % 0x0800;
                self.ram[actual_address] = value;
            }
            _ => panic!("Unsupported write address {}", address),
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        match address {
            0..=0x1FFF => {
                let ram_address = address % 0x0800;
                self.ram[ram_address]
            }
            0x8000..=0xFFFF => {
                let mut rom_address = address - 0x8000;
                if self.prg_rom.len() <= 16 * 1024 {
                    rom_address = rom_address % (16 * 1024);
                }
                self.prg_rom[rom_address]
            }
            _ => panic!("Unsupported read address {}", address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_that_system_panics_if_write_address_is_too_large() {
        let mut bus = Bus::new(0, vec![0; 16 * 1024]);
        bus.write(0x8000, 5);
    }

    #[test]
    fn test_that_writing_to_internal_ram_works_correctly() {
        let mut bus = Bus::new(0, vec![0; 16 * 1024]);
        bus.write(0x0000, 1);
        assert_eq!(1, bus.ram[0]);

        bus.write(0x0801, 2);
        assert_eq!(2, bus.ram[1]);

        bus.write(0x1002, 3);
        assert_eq!(3, bus.ram[2]);

        bus.write(0x1803, 4);
        assert_eq!(4, bus.ram[3]);
    }

    #[test]
    fn test_that_values_from_internal_ram_are_read_correctly() {
        let mut bus = Bus::new(0, vec![0; 16 * 1024]);
        bus.ram[0] = 1;
        bus.ram[1] = 2;
        bus.ram[2] = 3;
        bus.ram[3] = 4;
        assert_eq!(1, bus.read(0));
        assert_eq!(2, bus.read(0x0801));
        assert_eq!(3, bus.read(0x1002));
        assert_eq!(4, bus.read(0x1803));
    }

    #[test]
    fn test_that_values_from_prg_rom_are_read_correctly() {
        let mut prg_rom = vec![0; 16 * 1024];
        prg_rom[0] = 1;
        prg_rom[1] = 2;
        prg_rom[2] = 3;
        prg_rom[3] = 4;
        let bus = Bus::new(0, prg_rom);
        assert_eq!(1, bus.read(0x8000));
        assert_eq!(2, bus.read(0xC001));
    }
}
