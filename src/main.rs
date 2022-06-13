mod lib; use lib::*;
use std::env;

use std::io;
use std::io::Error;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;


fn main() {
    let args: Vec<String> = env::args().collect();

    let romfile = load_rom(&args[0]);
    let mut ROM = [0u8; 0xFF];

    for i in romfile {
        for j in 0..i.len(){
            ROM[j]=i[j];
            print!("{:?}, ", i[j]);
        }
       println!("")
    }

    let mut memory = MEM::new(ROM);
    //memory.set(0xFFFF, 0xFF, true);
    println!("{:?}", memory.get(0xFFFF));

    let mut cpu = CPU::new(&mut memory);


    println!("0x{:02x}", memory.get(0xA3) as i8);
    cpu.lexec(&mut memory, 3);
    println!("0x{:02x}", memory.get(0xA3) as i8);
}

pub fn load_rom(path: &String) -> Result<[u8; 0xFF], Error> {
    let mut f = File::open("C:\\Users\\trenton\\Documents\\code\\aaa\\emu6502\\testprogram.sfot")?;
    let mut reader = BufReader::new(f);
    let mut buffer1 = Vec::new();
    reader.read_to_end(&mut buffer1);

    let mut buffer = [0u8; 0xFF];

    for i in 0..buffer.len() {
        buffer[i] = buffer1[i];
    }

    return Ok(buffer);
}