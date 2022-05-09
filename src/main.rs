fn set_bit(byte: u8, bit: u8) -> u8 {
    return byte | 0b00000001<<bit+1
}
fn unset_bit(byte: u8, bit: u8) -> u8 {
    return byte & 0b11111110<<bit+1
}
fn get_bit(byte: u8, bit: u8) -> bool {
    return ((byte>>bit+1)&1) != 0
}
struct CPU {
    pc: u16, //Program Counter, should look at $FFFC and $FFFD for start location
    sp: u8, //Stack Pointer, inits to $FD and gets 100 added to its value when used
    sr: u8, //Status Flags, each bit is a flag, CZIDB-VN

    a: u8, //Registers
    x: u8,
    y: u8, 
}

impl CPU {
    fn reset(&mut self, mut memory: MEM) {
	self.pc = 0xFFFC;
	self.sp = 0xFD;
	self.sr = 0b00010000;
	self.a = 0;
	self.x = 0;
	self.y = 0;

	memory.init();
    }
    
    fn get_sp(&mut self, mem: MEM) -> u8 {
	return mem.ram[0x1FD]
    }

    fn get_pc(&mut self, cycles: u32, mem: MEM) -> u8 { 
	let data = mem.ram[self.pc as usize];
	return data;
    }

    fn exec(&mut self, mut cycles: u32, mut mem: MEM) -> Vec<u8> {
	let mut instlist = Vec::new();
	while cycles > 0 {
	    instlist.push(self.get_pc(cycles, mem));
	    self.pc += 1;
	    cycles -= 1;
	}
	return instlist;
    }
}

#[derive(Copy, Clone)]
struct MEM {
    ram: [u8;65535], //$0000 to $00FF is ZP, $0100 to $01FF is stack, $0200 to $FFFF is general purpose.
}

impl MEM {
    fn init(&mut self) {
	self.ram = [0; 0xFFFF];
    }
}

fn main() {
    let mut cpu = CPU {pc: 0, sp: 0, sr: 0, a: 0, x: 0, y: 0};
    let mut memory = MEM {ram: [0; 0xFFFF]};

    cpu.reset(memory);
    memory.ram[0xFFFC] = 0xA9;
    memory.ram[0xFFFD] = 0x02;
    print!("{:?}", cpu.exec(2, memory));
}
