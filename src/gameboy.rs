use crate::emulator::Emulator;

pub struct GameBoy {
    // Register File
    pc: u16,
    sp: u16,
    a: u8,
    f: u8,
    regs: [u8; 8],
    ir: u8,
    ie: u8,

    // Memory
    ram: [u8; 4096],
}

impl GameBoy {
    pub fn new() -> Self {
        let gameboy = GameBoy::default();

        gameboy
    }

    // Registers
    fn B(&self) -> u8 {
        self.regs[0]
    }
    fn C(&self) -> u8 {
        self.regs[1]
    }
    fn BC(&self) -> u16 {
        (self.regs[0] as u16) << 8 | (self.regs[1] as u16)
    }
    fn D(&self) -> u8 {
        self.regs[2]
    }
    fn E(&self) -> u8 {
        self.regs[3]
    }
    fn DE(&self) -> u16 {
        (self.regs[2] as u16) << 8 | (self.regs[3] as u16)
    }
    fn H(&self) -> u8 {
        self.regs[4]
    }
    fn L(&self) -> u8 {
        self.regs[5]
    }
    fn HL(&self) -> u16 {
        (self.regs[4] as u16) << 8 | (self.regs[5] as u16)
    }
    fn setB(&mut self, b: u8) {
        self.regs[0] = b;
    }
    fn setC(&mut self, c: u8) {
        self.regs[1] = c;
    }
    fn setBC(&mut self, bc: u16) {
        self.regs[0] = ((bc >> 8) & 0xf) as u8;
        self.regs[1] = (bc & 0xf) as u8;
    }
    fn setD(&mut self, d: u8) {
        self.regs[2] = d;
    }
    fn setE(&mut self, e: u8) {
        self.regs[3] = e;
    }
    fn setDE(&mut self, de: u16) {
        self.regs[2] = ((de >> 8) & 0xf) as u8;
        self.regs[3] = (de & 0xf) as u8;
    }
    fn setH(&mut self, h: u8) {
        self.regs[4] = h;
    }
    fn setL(&mut self, l: u8) {
        self.regs[5] = l;
    }
    fn setHL(&mut self, hl: u16) {
        self.regs[4] = ((hl >> 8) & 0xf) as u8;
        self.regs[5] = (hl & 0xf) as u8;
    }
}

impl Default for GameBoy {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Emulator for GameBoy {
    fn tick(&mut self) {
        let opcode = self.ram[self.pc as usize];
        self.pc += 1;
        let top_two = opcode >> 6;
        match top_two {
            0b00 => {
                let xxx = (opcode >> 3) & 0b111;
                let yyy = opcode & 0b111;

                if yyy == 0b110 && xxx != 0b110 {
                    // LD r, n
                    self.regs[xxx as usize] = self.ram[self.pc as usize];
                    self.pc += 1;
                } else if xxx == 0b110 && yyy == 0b110 {
                    // LD (HL), n
                    self.ram[self.HL() as usize] = self.ram[self.pc as usize];
                    self.pc += 1;
                } else if xxx == 0b001 && yyy == 0b010 {
                    // LD A, (BC)
                    self.a = self.ram[self.BC() as usize];
                } else if xxx == 0b011 && yyy == 0b010 {
                    // LD A, (DE)
                    self.a = self.ram[self.DE() as usize];
                } else if xxx == 0b000 && yyy == 0b010 {
                    // LD (BC), A
                    self.ram[self.BC() as usize] = self.a;
                } else if xxx == 0b010 && yyy == 0b010 {
                    // LD (DE), A
                    self.ram[self.DE() as usize] = self.a;
                }
            }
            0b01 => {
                let xxx = (opcode >> 3) & 0b111;
                let yyy = opcode & 0b111;
                if xxx == 0b110 {
                    // LD (HL), r
                    self.ram[self.HL() as usize] = self.regs[xxx as usize];
                } else if yyy == 0b110 {
                    // LD r, (HL)
                    self.regs[xxx as usize] = self.ram[self.HL() as usize];
                } else {
                    // LD r, r'
                    self.regs[xxx as usize] = self.regs[yyy as usize];
                }
            }
            _ => {}
        }
    }

    fn get_vbuf(&mut self) -> (Vec<u8>, usize, usize) {
        // todo!()
        (vec![0u8, 0u8, 0u8], 1, 1)
    }

    fn timer_tick(&mut self) {
        // todo!()
    }

    fn handle_key(&mut self, code: winit::keyboard::KeyCode, is_pressed: bool) {
        // todo!()
    }
}
