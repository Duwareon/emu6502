mod lib;
use lib::CPU;
use lib::MEM;

fn main() {
    let mut memory = MEM::new();
    
    memory.setrom(0xFFFC, 0x00);
    memory.setrom(0xFFFD, 0xFF);
    memory.set(0xff40, 27); // Error test

    let mut cpu = CPU::new(&mut memory);

    println!("{:x}", memory.get(0xA3));
    let program = vec![
        0xA9, 0xFF, //LDA #$FF
        0x69, 0x04, //ADC #$04
        0x85, 0xA3, //STA $A3
        0xEA, //NOP
        0xAA //ERROR
    ]; 
    memory.setromrange(0xFF00, &program);
    
    cpu.lexec(&mut memory, 5);

    println!("{:x}", memory.get(0xA3));
}
