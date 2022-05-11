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
    memory.set(0xFF02, 0x85); //STA $A3
    memory.set(0xFF03, 0xA3);

    cpu.exec(&mut memory);
    cpu.exec(&mut memory);

    println!("{:x}", memory.get(0xA3));
}
