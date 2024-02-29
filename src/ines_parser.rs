use super::instruction::Instruction;
use std::fmt;

pub struct InesContent {
    pub headers: InesHeaders,
    pub instructions: Vec<Instruction>,
}

pub struct InesHeaders {
    pub nes_string: Vec<u8>,
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub flags6: u8,
    pub flags7: u8,
    pub flags8: u8,
    pub flags9: u8,
    pub flags10: u8,
}

impl fmt::Display for InesHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NES string: {:?}\nPRG ROM size: {}\nCHR ROM size: {}\nFlags6: {}\nFlags7: {}\nFlags8: {}\nFlags9: {}\nFlags10: {}",
            self.nes_string,
            self.prg_rom_size,
            self.chr_rom_size,
            self.flags6,
            self.flags7,
            self.flags8,
            self.flags9,
            self.flags10
        )
    }
}

pub fn parse(filepath: &str) -> Result<InesContent, &'static str> {
    let data = std::fs::read(filepath).unwrap();
    parse_file_content(data)
}

pub fn parse_file_content(file_content: Vec<u8>) -> Result<InesContent, &'static str> {
    let mut nes_string = Vec::new();
    for i in file_content[0..=3].iter() {
        nes_string.push(*i);
    }

    let ines_headers = InesHeaders {
        nes_string,
        prg_rom_size: file_content[4],
        chr_rom_size: file_content[5],
        flags6: file_content[6],
        flags7: file_content[7],
        flags8: file_content[8],
        flags9: file_content[9],
        flags10: file_content[10],
    };

    println!("NES headers:\n{}", ines_headers);

    Err("Not implemented")
}
