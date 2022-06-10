mod lib; use lib::*;

fn main() {
    let mut memory = MEM::new();
    
    memory.setrange(0xFFFC, &vec![0x00, 0xFF], true); // Tell the 6502 where to find the code
    //memory.set(0xff40, 27, false); // Error test

    let mut cpu = CPU::new(&mut memory);
 
    memory.setrange(0xFF00, &vec![
        0xA9, 0x07, //LDA #$07
        0x69, 0xB4, //ADC #$B4
        0x85, 0xA3, //STA $A3
    ], true);

    println!("0x{:02x}", memory.get(0xA3) as i8);
    cpu.lexec(&mut memory, 3);
    println!("0x{:02x}", memory.get(0xA3) as i8);
}