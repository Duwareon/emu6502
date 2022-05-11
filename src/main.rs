mod lib;
use lib::CPU;
use lib::MEM;

fn main() {
    let mut memory = MEM::new();
    let mut cpu = CPU::new(&mut memory);

    memory.init();

    memory.set(0xFFFC, 0x00);
    memory.set(0xFFFD, 0xFF);

    cpu.reset(&mut memory);

    memory.set(0xFF00, 0xA9); //LDA #$10
    memory.set(0xFF01, 0x10);
    memory.set(0xFF02, 0x69); //ADC #$20
    memory.set(0xFF03, 0x20);
    memory.set(0xFF04, 0x85); //STA $A3
    memory.set(0xFF05, 0xA3);

    cpu.lexec(&mut memory, 3);

    println!("{:x}", memory.get(0xA3));
}
