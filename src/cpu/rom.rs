use cpu::instruction::Instruction;
use nom::{IResult, Endianness};

#[derive(Debug)]
pub struct Rom {
    clock_rate: u32,
    pc: u32,
    release: u32,
    crc_1: u32,
    crc_2: u32,
    image_name: String,
    manufacturer_id: u32,
    cartridge_id: u16,
    country_code: u16,
    bootcode: Vec <Instruction>,
}

impl Rom {
    pub fn from_rom(rom: &[u8]) -> IResult<&[u8], Rom> {
        parse_rom(rom)
    }
}

fn parse_rom(rom: &[u8]) -> IResult<&[u8], Rom> {
    do_parse!(rom,
        take!(4)        >>
        clock_rate:      u32!(Endianness::Big) >>
        pc:              u32!(Endianness::Big) >>
        release:         u32!(Endianness::Big) >>
        crc_1:           u32!(Endianness::Big) >>
        crc_2:           u32!(Endianness::Big) >>
        unknown_1:       take!(4 * 2)          >>
        image_name:      take_str!(20)         >>
        unknown_2:       take!(4 * 1)          >>
        manufacturer_id: u32!(Endianness::Big) >>
        cartridge_id:    u16!(Endianness::Big) >>
        country_code:    u16!(Endianness::Big) >>
        bootcode:        call!(parse_bootcode) >>
        (Rom{ clock_rate: clock_rate, pc: pc, release: release,
            crc_1: crc_1, crc_2: crc_2, image_name: image_name.to_string(),
            manufacturer_id: manufacturer_id, cartridge_id: cartridge_id,
            country_code: country_code, bootcode: bootcode })
    )
}

fn parse_bootcode(bootcode: &[u8]) -> IResult<&[u8], Vec<Instruction>> {
    let mut base_offset = 0x40;

    count!(bootcode, map_res!(u32!(Endianness::Big), |word| {
        let instruction = Instruction::from_word(word)
            .map_err(|err| format!("Failed To Decode Word At 0x{:X}: {}", base_offset, err));

        if let Err(asd) = instruction.clone() {
            println!("{}", asd);
        }

        base_offset += 0x04;
        instruction
    }), 1008)
}