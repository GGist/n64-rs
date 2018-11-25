#![allow(unused)]

#[macro_use]
extern crate nom;

mod cpu;

use cpu::instruction::Instruction;
use cpu::rom::Rom;
use nom::{IResult, Endianness};

const BYTES: &[u8] = include_bytes!("../Cube.z64");

fn main() {
    let initial = &BYTES[0..5000];
    let rom = Rom::from_rom(initial);

    println!("{:?}", rom);
}