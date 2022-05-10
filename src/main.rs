fn set_bit(byte: u8, bit: u8) -> u8 {
    return byte | 0b00000001 << bit + 1;
}
fn unset_bit(byte: u8, bit: u8) -> u8 {
    return byte & 0b11111110 << bit + 1;
}
fn get_bit(byte: u8, bit: u8) -> bool {
    return ((byte >> bit + 1) & 1) != 0;
}
struct CPU {
    pc: u16, //Program Counter, should look at $FFFC and $FFFD for start location
    sp: u8,  //Stack Pointer, inits to $FD and gets 100 added to its value when used
    sr: u8,  //Status Register, each bit is a flag, NV-BDIZC

    a: u8, //Registers
    x: u8,
    y: u8,
}

impl CPU {
    fn reset(&mut self, memory: &mut MEM) {
        self.pc = 0xFFFC;
        self.sp = 0xFD;
        self.sr = 0b00100000;
        self.a = 0;
        self.x = 0;
        self.y = 0;

        memory.init();
    }

    fn get_sp(&mut self, mem: &mut MEM) -> u8 {
        let addr: u16 = (self.sp as u16) + 0x100;
        return mem.get(addr);
    }

    fn get_inst(&mut self, mem: &mut MEM) -> u8 {
        let data = mem.get(self.pc as u16);
        self.pc += 1;
        return data;
    }

    fn exec(&mut self, mem: &mut MEM) {
        let inst = self.get_inst(mem);
        match inst {
            0xA9 => { //LDA a
                let val = self.get_inst(mem);
                self.a = val;
            }

            0x85 => { //STA zp
                let addr = self.get_inst(mem);
                println!("{:x}", addr);
                println!("{:x}", self.a);
                println!("{:X}", mem.get(0xA3));
                mem.set(addr as u16, self.a);
                println!("{:X}", mem.get(0xA3));
            }

            _ => {
                print!("BAD OPCODE!")
            }
        }
    }
}

#[derive(Copy, Clone)]
struct MEM {
    ram: [u8; 65535], //$0000 to $00FF is ZP, $0100 to $01FF is stack, $0200 to $FFFF is general purpose.
}

impl MEM {
    fn init(&mut self) {
        self.ram = [0; 0xFFFF];
    }

    fn get(&mut self, addr: u16) -> u8 {
        return self.ram[addr as usize];
    }

    fn set(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }
}

fn main() {
    let mut cpu = CPU {
        pc: 0,
        sp: 0,
        sr: 0,
        a: 0,
        x: 0,
        y: 0,
    };
    let mut memory = MEM { ram: [0; 0xFFFF] };

    cpu.reset(&mut memory);

    cpu.a = 0x23;

    memory.ram[0xFFFC] = 0x85;
    memory.ram[0xFFFD] = 0xA3;
    cpu.exec(&mut memory);
    println!("{:x}", memory.get(0xA3));
    assert!(memory.get(0xA3)==cpu.a);
}
