mod lib;
use lib::CPU;
use lib::MEM;

fn main() {
    let mut memory = MEM::new();
    
    memory.setrange(0xFFFC, &vec![0x00, 0xFF], true); // Tell the 6502 where to find the code
    memory.set(0xff40, 27, true); // Error test

    let mut cpu = CPU::new(&mut memory);
 
    memory.setrange(0xFF00, &vec![
        0xA9, 0x03, //LDA #$FF
        0x69, 0x07, //ADC #$04
        0x85, 0xA3, //STA $A3
        0xEA, //NOP
        0xff //ERROR
    ], true);

    println!("{:x}", memory.get(0xA3));
    cpu.lexec(&mut memory, 5);
    println!("{:x}", memory.get(0xA3) as i8);
}
