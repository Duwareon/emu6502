mod lib; use lib::*;
use std::env;

use std::io::Error;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);
    if args.len() < 2 {
        panic!("Please specify what file to run.");
    }
    let path = &args[1];
    let romfile = load_rom(path);
    let mut rom = [0u8; 0x10000];

    for i in romfile {
        for j in 0..(rom.len()){
            rom[j]=i[j];
        }
    }

    let mut memory = MEM::new(rom);
    let mut cpu = CPU::new(&mut memory);

    cpu.fexec(&mut memory);
}

pub fn load_rom(path: &String) -> Result<[u8; 0x10000], Error> {
    let f = File::open(path);
    if f.is_err(){
        panic!("ERROR INTAKING FILE")
    }
    let f = f?;
    let mut reader = BufReader::new(f);
    let mut buffer1 = Vec::new();
    let mut buffer = [0u8; 0x10000];

    if reader.read_to_end(&mut buffer1).is_err(){
        println!("Couldn't read file!")
    }
    for i in 0..buffer.len() {
        buffer[i] = buffer1[i];
    }

    return Ok(buffer);
}