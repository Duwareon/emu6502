mod lib;
use lib::CPU;
use lib::MEM;

fn main() {
    let mut memory = MEM::new();
    
    memory.set(0xFFFC, 0x00);
    memory.set(0xFFFD, 0xFF);

    let mut cpu = CPU::new(&mut memory);

    println!("{:x}", memory.get(0xA3));

    let program = vec![0xA9, 0x1C, 0x69, 0x2B, 0x85, 0xA3, 0xEA, 0xAA];

    memory.setrange(0xFF00, &program);
    
    cpu.lexec(&mut memory, 5);

    println!("{:x}", memory.get(0xA3));
}
