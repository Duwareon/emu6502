use std::process::exit;
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

impl Bit for i8 {
    fn get_bit(&self, bit: u8) -> bool {
        return ((self >> bit) & 1) != 0
    }
    fn set_bit(&mut self, bit: u8) -> Self {
        return *self | 0b00000001 << bit
    }
    fn unset_bit(&mut self, bit: u8) -> Self {
        return *self & (0b11111110 << bit) as i8
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

impl Bit for i16 {
    fn get_bit(&self, bit: u8) -> bool {
        return ((self >> bit) & 1) != 0
    }
    fn set_bit(&mut self, bit: u8) -> Self {
        return *self | 0b0000000000000001 << bit
    }
    fn unset_bit(&mut self, bit: u8) -> Self {
        return *self & (0b1111111111111110  << bit) as i16
    }
}

pub struct CPU {
    pc: u16, //Program Counter, should look at $FFFC and $FFFD for start location
    sp: u8,  //Stack Pointer, inits to $FD and gets 100 added to its value when used
                                                  //76543210
    sr: u8,  //Status Register, each bit is a flag, NV-BDIZC

    a: i8, //Registers
    x: i8,
    y: i8,
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

    fn push(&mut self, mem: &mut MEM, val: u8) {
        let addr: u16 = (self.sp as u16) + 0x100;
        mem.set(addr, val, true);
        self.sp -= 1;
    }

    fn pull(&mut self, mem: &mut MEM) -> u8 {
        let addr: u16 = (self.sp as u16) + 0x100;
        self.sp += 1;
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
            0x08 => { //PHP
                self.push(mem, self.sr)
            }

            0x0A => { //ASL A
                if self.a.get_bit(7){
                    self.sr.set_bit(0);
                }
                else{
                    self.sr.unset_bit(0);
                }

                if self.a == 0 {
                    self.sr.set_bit(1);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                }
                self.a = (self.a << 1).unset_bit(0);
            }

            0x18 => { //CLC
                self.sr.unset_bit(0);
            }

            0x28 => { //PLP
                self.sr = self.pull(mem)
            }

            0x38 => { //SEC
                self.sr.set_bit(0);
            }

            0x48 => { //PHA
                self.push(mem, self.a as u8);

                if self.a == 0 {
                    self.sr.set_bit(1);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                }
            }

            0x4C => { //JMP a
                let addr1 = self.get_next(mem);
                let addr2 = self.get_next(mem);
                self.pc = addr1 as u16 + (addr2 as u16)*0x100;
            }

            0x58 => { //CLI
                self.sr.unset_bit(2);
            }

            0x65 => { //ADC ZP
                let addr = self.get_next(mem);
                let val = mem.get(addr as u16) as i8;
                let result = self.a as i16 + val as i16;

                if result.get_bit(8) {
                    self.sr = self.sr.set_bit(0);
                }

                self.a = result as i8;

                if self.a == 0 {
                    self.sr.set_bit(1);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                } 
            }

            0x68 => { //PLA
                self.a = self.pull(mem) as i8;

                if self.a == 0 {
                    self.sr.set_bit(0);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                }
            }

            0x69 => { //ADC #
                let val = 0b0000000011111111 & (self.get_next(mem) as u16);
                let acc = 0b0000000011111111 & (self.a as u16);
                let result = (acc+val) as i16;
            
                if result.get_bit(8) {
                    self.sr = self.sr.set_bit(0);
                }

                self.a = result as i8;

                if self.a == 0 {
                    self.sr.set_bit(1);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                }
            }

            0x6C => { //JMP (a)
                let addr1 = self.get_next(mem);
                let addr2 = self.get_next(mem);
                self.pc = addr1 as u16 + (addr2 as u16)*0x100;

                let addr1 = self.get_next(mem);
                let addr2 = self.get_next(mem);
                self.pc = addr1 as u16 + (addr2 as u16)*0x100;
            }

            0x6D => { //ADC a
                let addr1 = self.get_next(mem);
                let addr2 = self.get_next(mem);
                let result = self.a + (mem.get(addr1 as u16 + (addr2 as u16)*0x100)) as i8;

                if result.get_bit(8) {
                    self.sr = self.sr.set_bit(0);
                }

                self.a = result;

                if self.a == 0 {
                    self.sr.set_bit(1);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                }
            }

            0x78 => { //SEI
                self.sr.set_bit(2);
            }

            0x84 => { //STY zp
                let addr = self.get_next(mem) as u16;
                mem.set(addr, self.y as u8, false);
            }

            0x85 => { //STA zp
                let addr = self.get_next(mem) as u16;
                mem.set(addr, self.a as u8, false);
            }

            0x86 => { //STX zp
                let addr = self.get_next(mem) as u16;
                mem.set(addr, self.x as u8, false);
            }

            0x88 => { //DEY
                self.y -= 1;

                if self.a == 0 {
                    self.sr.set_bit(1);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                }
            }

            0x8A => { //TXA
                self.a = self.x;

                if self.a == 0 {
                    self.sr.set_bit(1);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                }
            }

            0x98 => { //TYA
                self.a = self.y;

                if self.a == 0 {
                    self.sr.set_bit(1);
                }
                else if self.a < 0 {
                    self.sr.set_bit(7);
                }
            }

            0x9A => { //TXS
                self.push(mem, self.x as u8);
            }

            0xA8 => { //TAY
                self.y = self.a;

                if self.y == 0 {
                    self.sr.set_bit(1);
                }
                else if self.y < 0 {
                    self.sr.set_bit(7);
                }
            }

            0xA9 => { //LDA #
                let val = self.get_next(mem) as i8;
                self.a = val;
            }

            0xAA => { //TAX
                self.x = self.a;

                if self.x == 0 {
                    self.sr.set_bit(1);
                }
                else if self.x < 0 {
                    self.sr.set_bit(7);
                }
            }

            0xB8 => { //CLV
                self.sr.unset_bit(6);
            }

            0xBA => { //TSX
                self.x = self.pull(mem) as i8;

                if self.x == 0 {
                    self.sr.set_bit(1);
                }
                else if self.x < 0 {
                    self.sr.set_bit(7);
                }
            }

            0xC8 => { //INY
                self.y += 1;

                if self.y == 0 {
                    self.sr.set_bit(1);
                }
                else if self.y < 0 {
                    self.sr.set_bit(7);
                }
            }

            0xCA => { //DEX
                self.x -= 1;

                if self.x == 0 {
                    self.sr.set_bit(1);
                }
                else if self.x < 0 {
                    self.sr.set_bit(7);
                }
            }

            0xD8 => { //CLD
                self.sr.unset_bit(4);
            }

            0xE8 => { //INX
                self.x += 1;

                if self.x == 0 {
                    self.sr.set_bit(1);
                }
                else if self.x < 0 {
                    self.sr.set_bit(7);
                }
            }

            0xEA => {} //NOP

            0xF8 => { //SED
                self.sr.set_bit(4);
            }

            _ => { //UNRECOGNIZED
                println!("BAD OPCODE: 0x{:02x}, ADDR: 0x{:04x}", inst, loc)
            }
        }
    }
}

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

    pub fn set(&mut self, addr: u16, val: u8, rom: bool) {
        if ((addr < 0xff00) & (addr < 0xff || addr > 0x1ff)) || rom {
            self.ram[addr as usize] = val
        }
        else {
            println!("ERROR: ATTEMPT TO SET ROM/STACK TO 0x{:02x} AT 0x{:04x}!", val, addr)
        }
    }

    pub fn setrange(&mut self, addr: u16, vals: &Vec<u8>, rom: bool) {
        for (i, v) in vals.iter().enumerate() {
            self.set(addr + (i as u16), *v, rom);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut memory = MEM::new();
        memory.setrange(0xFFFC, &vec![0x00, 0xFF], true);
        memory.setrange(0xFF00, &vec![
            0xA9, 0x03, //LDA #$03
            0x69, 0x07, //ADC #$04
            0x85, 0xA3, //STA $A3
        ], true);

        let mut cpu = CPU::new(&mut memory);
        cpu.lexec(&mut memory, 3);

        assert_eq!(memory.get(0xA3), 0x0A);
    }

    #[test]
    fn test_add_carry() {
        let mut memory = MEM::new();
        memory.setrange(0xFFFC, &vec![0x00, 0xFF], true);
        memory.setrange(0xFF00, &vec![
            0xA9, 0xFF, //LDA #$FF
            0x69, 0x02, //ADC #$02
            0x85, 0xA3, //STA $A3
        ], true);

        let mut cpu = CPU::new(&mut memory);
        println!("{:08b}", cpu.sr);
        cpu.lexec(&mut memory, 3);
        println!("{:08b}", cpu.sr);

        assert!(cpu.sr.get_bit(0));
        assert_eq!(cpu.a, 1);
    }

    #[test]
    fn test_indirect_jump() {
        let mut memory = MEM::new();
        memory.setrange(0xFFFC, &vec![0x00, 0xFF], true);
        memory.setrange(0xFF00, &vec![
            0x6C, 0xFF, 0x69, // jmp ($FF69)
        ], true);
        memory.setrange(0xFF69, &vec![
            0xFF, 0xAA, // jmp ($FF69)
        ], true);
        memory.setrange(0xFFAA, &vec![
            0xA9, 0x03, //LDA #$03
            0x69, 0x07, //ADC #$04
            0x85, 0xA3, //STA $A3
        ], true);

        let mut cpu = CPU::new(&mut memory);
        cpu.lexec(&mut memory, 4);

        assert_eq!(memory.get(0xA3), 0x0A);
    }
}