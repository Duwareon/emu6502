pub trait Bit {
    fn get_bit(&self, bit: u8) -> bool;
    fn set_bit(&mut self, bit: u8) -> Self;
    fn unset_bit(&mut self, bit: u8) -> Self;
}

impl Bit for u8 {
    fn get_bit(&self, bit: u8) -> bool {
        return ((self >> bit) & 1) != 0
    }
    fn set_bit(&mut self, bit: u8) -> Self {
        return *self | 0b00000001 << bit
    }
    fn unset_bit(&mut self, bit: u8) -> Self {
        return *self & 0b11111110 << bit
    }
}

impl Bit for u16 {
    fn get_bit(&self, bit: u8) -> bool {
        return ((self >> bit) & 1) != 0
    }
    fn set_bit(&mut self, bit: u8) -> Self {
        return *self | 0b0000000000000001 << bit
    }
    fn unset_bit(&mut self, bit: u8) -> Self {
        return *self & 0b1111111111111110 << bit
    }
}

pub struct CPU {
    pc: u16, //Program Counter, should look at $FFFC and $FFFD for start location
    sp: u8,  //Stack Pointer, inits to $FD and gets 100 added to its value when used
    sr: u8,  //Status Register, each bit is a flag, NV-BDIZC

    a: u8, //Registers
    x: u8,
    y: u8,
}

impl CPU {
    pub fn reset(&mut self, mem: &mut MEM) {
        self.pc = mem.get(0xFFFC) as u16 + (mem.get(0xFFFD) as u16)*0x100;
        self.sp = 0xFD;
        self.sr = 0b00100000;
        self.a = 0;
        self.x = 0;
        self.y = 0;
    }

    pub fn new(mem: &mut MEM) -> CPU {
        let mut newcpu = CPU {pc: 0, sp: 0, sr: 0, a: 0, x: 0, y: 0};
        newcpu.reset(mem);
        return newcpu;
    }

    fn get_sp(&mut self, mem: &mut MEM) -> u8 {
        let addr: u16 = (self.sp as u16) + 0x100;
        return mem.get(addr);
    }

    fn get_next(&mut self, mem: &mut MEM) -> u8 {
        let data = mem.get(self.pc as u16);
        self.pc += 1;
        return data;
    }

    pub fn lexec(&mut self, mem: &mut MEM, mut cycles: u32) {
        while cycles > 0 {
            self.exec(mem);
            cycles-=1;
        }
    }

    pub fn exec(&mut self, mem: &mut MEM) {
        let loc = self.pc;
        let inst = self.get_next(mem);
        
        match inst {
            0xA9 => { //LDA #
                let val = self.get_next(mem);
                self.a = val;
            }

            0x69 => { //ADC #
                let val = self.get_next(mem);
                self.a = self.a + val;
                if (self.a as u16 + val as u16).get_bit(8) {
                    self.sr.set_bit(0);
                }
            }

            0x85 => { //STA zp
                let addr = self.get_next(mem);
                mem.set(addr as u16, self.a);
            }

            0xEA => {} //NOP

            _ => {
                println!("BAD OPCODE: 0x{:02x}, ADDR: 0x{:04x}", inst, loc)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct MEM {
    ram: [u8; 65535], //$0000 to $00FF is ZP, $0100 to $01FF is stack, $0200 to $FFFF is general purpose.
}

impl MEM {
    pub fn new() -> MEM {
        let mut newmem = MEM {ram: [0; 0xFFFF]};
        newmem.init();
        return newmem;
    }

    pub fn init(&mut self) {
        self.ram = [0; 0xFFFF]
    }

    pub fn get(&mut self, addr: u16) -> u8 {
        return self.ram[addr as usize]
    }

    pub fn set(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val
    }

    pub fn setrange(&mut self, addr: u16, vals: &Vec<u8>) {
        for (i, v) in vals.iter().enumerate() {
            self.set(addr + (i as u16), *v);
        }
    }
}

