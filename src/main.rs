mod lib; use lib::*;
use std::env;

use std::io::Error;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let romfile = load_rom(&args[0]);
    let mut ROM = [0u8; 0x100];

    for i in romfile {
        for j in 0..(ROM.len()){
            ROM[j]=i[j];
        }
    }

    let mut memory = MEM::new(ROM);
    //memory.set(0xFFFF, 0xFF, true);
    let mut cpu = CPU::new(&mut memory);


    println!("0x{:02x}", memory.get(0xA3) as i8);
    cpu.lexec(&mut memory, 3);
    println!("0x{:02x}", memory.get(0xA3) as i8);
}

pub fn load_rom(path: &String) -> Result<[u8; 0x100], Error> {
    let mut f = File::open("C:\\Users\\trenton\\Documents\\code\\aaa\\emu6502\\testprogram.sfot")?; //File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer1 = Vec::new();
    let mut buffer = [0u8; 0x100];

    reader.read_to_end(&mut buffer1);
    for i in 0..buffer.len() {
        buffer[i] = buffer1[i];
    }

    return Ok(buffer);
}