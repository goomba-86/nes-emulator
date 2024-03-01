use super::instruction::Instruction;
use core::panic;
use std::{fmt, usize};

pub struct InesContent {
    pub headers: InesHeaders,
    pub instructions: Vec<Instruction>,
}

pub struct InesHeaders {
    pub nes_string: Vec<u8>,
    pub prg_rom_size_lsb: u8,
    pub chr_rom_size_lsb: u8,
    pub flags6: u8,
    pub flags7: u8,
    pub mapper_msb_submapper: u8,
    pub prg_rom_chr_rom_size_msb: u8,
    pub prg_ram_eeprom_size: u8,
    pub chr_ram_size: u8,
    pub cpu_ppu_timing: u8,
    pub hardware_type: u8,
    pub misc_roms: u8,
    pub default_expansion_device: u8,
}

impl InesHeaders {
    pub fn get_prg_rom_size_in_16_kb(&self) -> u16 {
        let mut prg_rom_size_16_kb: u16 = self.prg_rom_chr_rom_size_msb as u16 & 0x000F;

        if prg_rom_size_16_kb > 0x0E {
            panic!("Exponent notation for PRG ROM not supported");
        }

        prg_rom_size_16_kb = prg_rom_size_16_kb << 8;
        prg_rom_size_16_kb + self.prg_rom_size_lsb as u16
    }

    pub fn get_chr_rom_size_in_8_kb(&self) -> u16 {
        let mut chr_rom_size_8_kb: u16 = self.prg_rom_chr_rom_size_msb as u16 >> 4;

        if chr_rom_size_8_kb > 0x0E {
            panic!("Exponent notation for CHR ROM not supported");
        }
        chr_rom_size_8_kb = chr_rom_size_8_kb << 8;
        chr_rom_size_8_kb + self.chr_rom_size_lsb as u16
    }

    pub fn trainer_area_present(&self) -> bool {
        self.flags6 & 0b00000100 > 1
    }
}

impl fmt::Display for InesHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PRG ROM size(16 KB): {}\nCHR ROM size(8KB): {}\nTrainer area: {}",
            self.get_prg_rom_size_in_16_kb(),
            self.get_chr_rom_size_in_8_kb(),
            self.trainer_area_present()
        )
    }
}

pub fn parse(filepath: &str) -> InesContent {
    let data = std::fs::read(filepath).unwrap();
    parse_file_content(data)
}

pub fn parse_file_content(file_content: Vec<u8>) -> InesContent {
    let mut nes_string = Vec::new();
    for i in file_content[0..=3].iter() {
        nes_string.push(*i);
    }

    let ines_headers = InesHeaders {
        nes_string,
        prg_rom_size_lsb: file_content[4],
        chr_rom_size_lsb: file_content[5],
        flags6: file_content[6],
        flags7: file_content[7],
        mapper_msb_submapper: file_content[8],
        prg_rom_chr_rom_size_msb: file_content[9],
        prg_ram_eeprom_size: file_content[10],
        chr_ram_size: file_content[11],
        cpu_ppu_timing: file_content[12],
        hardware_type: file_content[13],
        misc_roms: file_content[14],
        default_expansion_device: file_content[15],
    };

    println!("NES headers:\n{}", ines_headers);

    if ines_headers.trainer_area_present() {
        panic!("Trainer are in iNES file not supported.")
    }

    let mut prg_rom: Vec<u8> = Vec::new();
    for i in 16..=ines_headers.get_prg_rom_size_in_16_kb() * 16 {
        prg_rom.push(file_content[i as usize]);
    }

    let instructions = parse_instructions(prg_rom);

    InesContent {
        headers: ines_headers,
        instructions,
    }
}

pub fn parse_instructions(prg_rom: Vec<u8>) -> Vec<Instruction> {
    panic!("Instruction parsing not supported");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_prg_rom_size_is_calculated_correctly() {
        let ines_headers = InesHeaders {
            nes_string: vec!['N' as u8, 'E' as u8, 'S' as u8, 0x1A],
            prg_rom_size_lsb: 0x12,
            chr_rom_size_lsb: 0x34,
            flags6: 0,
            flags7: 0,
            prg_rom_chr_rom_size_msb: 0x12,
            mapper_msb_submapper: 0,
            prg_ram_eeprom_size: 0,
            chr_ram_size: 0,
            cpu_ppu_timing: 0,
            hardware_type: 0,
            misc_roms: 0,
            default_expansion_device: 0,
        };

        assert_eq!(0x0212, ines_headers.get_prg_rom_size_in_16_kb());
    }

    #[test]
    fn test_that_chr_rom_size_is_calculated_correctly() {
        let ines_headers = InesHeaders {
            nes_string: vec!['N' as u8, 'E' as u8, 'S' as u8, 0x1A],
            prg_rom_size_lsb: 0x12,
            chr_rom_size_lsb: 0x34,
            flags6: 0,
            flags7: 0,
            prg_rom_chr_rom_size_msb: 0x12,
            mapper_msb_submapper: 0,
            prg_ram_eeprom_size: 0,
            chr_ram_size: 0,
            cpu_ppu_timing: 0,
            hardware_type: 0,
            misc_roms: 0,
            default_expansion_device: 0,
        };

        assert_eq!(0x0134, ines_headers.get_chr_rom_size_in_8_kb());
    }

    #[test]
    fn test_that_trainer_area_bit_is_read_correctly() {
        let ines_headers = InesHeaders {
            nes_string: vec!['N' as u8, 'E' as u8, 'S' as u8, 0x1A],
            prg_rom_size_lsb: 0x12,
            chr_rom_size_lsb: 0x34,
            flags6: 0,
            flags7: 0,
            prg_rom_chr_rom_size_msb: 0x12,
            mapper_msb_submapper: 0,
            prg_ram_eeprom_size: 0,
            chr_ram_size: 0,
            cpu_ppu_timing: 0,
            hardware_type: 0,
            misc_roms: 0,
            default_expansion_device: 0,
        };
        assert!(!ines_headers.trainer_area_present());

        let ines_headers_with_trainer_area = InesHeaders {
            flags6: 0b00000100,
            ..ines_headers
        };
        assert!(ines_headers_with_trainer_area.trainer_area_present());
    }
}
