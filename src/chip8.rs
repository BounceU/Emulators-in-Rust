use std::fs;

use rand::Rng;
use winit::keyboard::KeyCode;

use crate::emulator::Emulator;

const C8_VBUF_WIDTH: usize = 64;
const C8_VBUF_HEIGHT: usize = 32;

pub struct Chip8 {
    v: [u8; 16],
    sp: u8,
    stack: [u16; 16],
    i: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    ram: [u8; 0xFFF],
    keypad: [u8; 16],
    looking_for_key: bool,
    most_recent_key: u8,
    vbuf: [u8; C8_VBUF_WIDTH * C8_VBUF_HEIGHT],
    off_color: [u8; 3],
    on_color: [u8; 3], // rng: ThreadRng,
}

impl Chip8 {
    pub fn new(filename: &str) -> Self {
        let mut chip8 = Chip8::default();

        let bytes: Vec<u8> = fs::read(filename).expect("Could not get file");
        chip8.ram[0x200..(bytes.len() + 0x200)].copy_from_slice(&bytes[..]);

        chip8
    }

    pub fn set_colors(&mut self, off_color: [u8; 3], on_color: [u8; 3]) {
        self.off_color = off_color;
        self.on_color = on_color;
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut ram = [0u8; 0xFFF];
        // 0
        ram[0] = 0xF0;
        ram[1] = 0x90;
        ram[2] = 0x90;
        ram[3] = 0x90;
        ram[4] = 0xF0;
        // 1
        ram[5] = 0x20;
        ram[6] = 0x60;
        ram[7] = 0x20;
        ram[8] = 0x20;
        ram[9] = 0x70;
        // 2
        ram[10] = 0xF0;
        ram[11] = 0x10;
        ram[12] = 0xF0;
        ram[13] = 0x80;
        ram[14] = 0xF0;
        // 3
        ram[15] = 0xF0;
        ram[16] = 0x10;
        ram[17] = 0xF0;
        ram[18] = 0x10;
        ram[19] = 0xF0;
        // 4
        ram[20] = 0x90;
        ram[21] = 0x90;
        ram[22] = 0xF0;
        ram[23] = 0x10;
        ram[24] = 0x10;
        // 5
        ram[25] = 0xF0;
        ram[26] = 0x80;
        ram[27] = 0xF0;
        ram[28] = 0x10;
        ram[29] = 0xF0;
        // 6
        ram[30] = 0xF0;
        ram[31] = 0x80;
        ram[32] = 0xF0;
        ram[33] = 0x90;
        ram[34] = 0xF0;
        // 7
        ram[35] = 0xF0;
        ram[36] = 0x10;
        ram[37] = 0x20;
        ram[38] = 0x40;
        ram[39] = 0x40;
        // 8
        ram[40] = 0xF0;
        ram[41] = 0x90;
        ram[42] = 0xF0;
        ram[43] = 0x90;
        ram[44] = 0xF0;
        // 9
        ram[45] = 0xF0;
        ram[46] = 0x90;
        ram[47] = 0xF0;
        ram[48] = 0x10;
        ram[49] = 0xF0;
        // A
        ram[50] = 0xF0;
        ram[51] = 0x90;
        ram[52] = 0xF0;
        ram[53] = 0x90;
        ram[54] = 0x90;
        // B
        ram[55] = 0xE0;
        ram[56] = 0x90;
        ram[57] = 0xE0;
        ram[58] = 0x90;
        ram[59] = 0xE0;
        // C
        ram[60] = 0xF0;
        ram[61] = 0x80;
        ram[62] = 0x80;
        ram[63] = 0x80;
        ram[64] = 0xF0;
        // D
        ram[65] = 0xE0;
        ram[66] = 0x90;
        ram[67] = 0x90;
        ram[68] = 0x90;
        ram[69] = 0xE0;
        // E
        ram[70] = 0xF0;
        ram[71] = 0x80;
        ram[72] = 0xF0;
        ram[73] = 0x80;
        ram[74] = 0xF0;
        // F
        ram[75] = 0xF0;
        ram[76] = 0x80;
        ram[77] = 0xF0;
        ram[78] = 0x80;
        ram[79] = 0x80;

        Self {
            v: Default::default(),
            sp: Default::default(),
            stack: [0u16; 16],
            i: Default::default(),
            pc: 0x200,
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            ram,
            keypad: [0u8; 16],
            looking_for_key: false,
            most_recent_key: 16u8,
            vbuf: [0u8; C8_VBUF_WIDTH * C8_VBUF_HEIGHT],
            off_color: [0u8, 0u8, 0u8],
            on_color: [255u8, 255u8, 255u8],
        }
    }
}

impl Emulator for Chip8 {
    fn handle_key(&mut self, code: KeyCode, is_pressed: bool) {
        match code {
            KeyCode::KeyX => {
                if is_pressed {
                    self.keypad[0] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 0;
                    }
                } else {
                    self.keypad[0] = 0;
                }
            }
            KeyCode::Digit1 => {
                if is_pressed {
                    self.keypad[1] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 1;
                    }
                } else {
                    self.keypad[1] = 0;
                }
            }
            KeyCode::Digit2 => {
                if is_pressed {
                    self.keypad[2] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 2;
                    }
                } else {
                    self.keypad[2] = 0;
                }
            }
            KeyCode::Digit3 => {
                if is_pressed {
                    self.keypad[3] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 3;
                    }
                } else {
                    self.keypad[3] = 0;
                }
            }
            KeyCode::KeyQ => {
                if is_pressed {
                    self.keypad[4] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 4;
                    }
                } else {
                    self.keypad[4] = 0;
                }
            }
            KeyCode::KeyW => {
                if is_pressed {
                    self.keypad[5] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 5;
                    }
                } else {
                    self.keypad[5] = 0;
                }
            }
            KeyCode::KeyE => {
                if is_pressed {
                    self.keypad[6] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 6;
                    }
                } else {
                    self.keypad[6] = 0;
                }
            }
            KeyCode::KeyA => {
                if is_pressed {
                    self.keypad[7] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 7;
                    }
                } else {
                    self.keypad[7] = 0;
                }
            }
            KeyCode::KeyS => {
                if is_pressed {
                    self.keypad[8] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 8;
                    }
                } else {
                    self.keypad[8] = 0;
                }
            }
            KeyCode::KeyD => {
                if is_pressed {
                    self.keypad[9] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 9;
                    }
                } else {
                    self.keypad[9] = 0;
                }
            }
            KeyCode::KeyZ => {
                if is_pressed {
                    self.keypad[10] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 10;
                    }
                } else {
                    self.keypad[10] = 0;
                }
            }
            KeyCode::KeyC => {
                if is_pressed {
                    self.keypad[11] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 11;
                    }
                } else {
                    self.keypad[11] = 0;
                }
            }
            KeyCode::Digit4 => {
                if is_pressed {
                    self.keypad[12] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 12;
                    }
                } else {
                    self.keypad[12] = 0;
                }
            }
            KeyCode::KeyR => {
                if is_pressed {
                    self.keypad[13] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 13;
                    }
                } else {
                    self.keypad[13] = 0;
                }
            }
            KeyCode::KeyF => {
                if is_pressed {
                    self.keypad[14] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 14;
                    }
                } else {
                    self.keypad[14] = 0;
                }
            }
            KeyCode::KeyV => {
                if is_pressed {
                    self.keypad[15] = 1;
                    if self.looking_for_key && self.most_recent_key >= 16 {
                        self.most_recent_key = 15;
                    }
                } else {
                    self.keypad[15] = 0;
                }
            }
            _ => {}
        }
    }

    fn timer_tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
    fn tick(&mut self) {
        let instruction: u16 =
            ((self.ram[self.pc as usize] as u16) << 8) | self.ram[self.pc as usize + 1] as u16;
        println!("Instruction: {:X}", instruction);
        let nnn = instruction & 0xFFF;
        let n = instruction & 0xF;
        let x = (instruction >> 8) & 0xF;
        let y = (instruction >> 4) & 0xF;
        let kk = instruction & 0xFF;

        match (instruction >> 12) & 0xF {
            0 => match instruction {
                0x00E0 => {
                    // CLS
                    self.vbuf = [0u8; C8_VBUF_WIDTH * C8_VBUF_HEIGHT];
                    self.pc += 2; // Move ahead
                }
                0x00EE => {
                    // RET
                    self.pc = self.stack[self.sp as usize];
                    self.sp -= 1;
                }
                _ => {
                    // SYS addr (ignore for now)
                    // self.pc = nnn;
                }
            },
            1 => {
                // JP addr
                self.pc = nnn;
            }
            2 => {
                // CALL addr
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc + 2;
                self.pc = nnn;
            }
            3 => {
                // SE Vx, byte
                if self.v[x as usize] == kk as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            4 => {
                // SNE Vx, byte
                if self.v[x as usize] != kk as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            5 => {
                // SE Vx, Vy
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            6 => {
                // LD Vx, byte
                self.v[x as usize] = kk as u8;
                self.pc += 2;
            }
            7 => {
                // ADD Vx, byte
                self.v[x as usize] = self.v[x as usize].wrapping_add(kk as u8);
                self.pc += 2;
            }
            8 => match n {
                0 => {
                    // LD Vx, Vy
                    self.v[x as usize] = self.v[y as usize];
                    self.pc += 2;
                }
                1 => {
                    // OR Vx, Vy
                    self.v[x as usize] |= self.v[y as usize];
                    self.pc += 2;

                    self.v[0xf] = 0; // Quirk
                }
                2 => {
                    // AND Vx, Vy
                    self.v[x as usize] &= self.v[y as usize];
                    self.pc += 2;

                    self.v[0xf] = 0; // Quirk
                }
                3 => {
                    // XOR Vx, Vy
                    self.v[x as usize] ^= self.v[y as usize];
                    self.pc += 2;

                    self.v[0xf] = 0; // Quirk
                }
                4 => {
                    // ADD Vx, Vy
                    let carry: bool;
                    (self.v[x as usize], carry) =
                        self.v[x as usize].carrying_add(self.v[y as usize], false);
                    self.v[0xF] = if carry { 1 } else { 0 };

                    self.pc += 2;
                }
                5 => {
                    // SUB Vx, Vy
                    let carry: bool;
                    (self.v[x as usize], carry) =
                        self.v[x as usize].borrowing_sub(self.v[y as usize], false);
                    self.v[0xF] = if !carry { 1 } else { 0 };

                    self.pc += 2;
                }
                6 => {
                    // SHR Vx {, Vy}
                    self.v[0xf] = self.v[x as usize] & 0x1;
                    self.v[x as usize] >>= 1;

                    self.pc += 2;
                }
                7 => {
                    // SUBN Vx, Vy
                    let carry: bool;
                    (self.v[x as usize], carry) =
                        self.v[y as usize].borrowing_sub(self.v[x as usize], false);
                    self.v[0xF] = if !carry { 1 } else { 0 };

                    self.pc += 2;
                }
                0xE => {
                    // SHL Vx {, Vy}
                    self.v[0xf] = (self.v[x as usize].reverse_bits()) & 0x1;
                    self.v[x as usize] <<= 1;

                    self.pc += 2;
                }
                _ => {
                    // Unknown Instruction
                    println!("Unknown instruction: {:X}", instruction);
                    self.pc += 2;
                }
            },
            9 => {
                if n == 0 {
                    // SNE Vx, Vy
                    if self.v[x as usize] != self.v[y as usize] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                } else {
                    // Unknown Instruction
                    println!("Unknown instruction: {:X}", instruction);
                    self.pc += 2;
                }
            }
            0xA => {
                // LD I, addr
                self.i = nnn;
                self.pc += 2;
            }
            0xB => {
                // JP V0, addr
                self.pc = nnn + self.v[0] as u16;
            }
            0xC => {
                // RND Vx, byte
                let rand_byte: u8 = rand::rng().random();
                self.v[x as usize] = rand_byte & kk as u8;
                self.pc += 2;
            }
            0xD => {
                // DRW Vx, Vy, nibble

                let mut collision = false;
                for dy in 0u8..n as u8 {
                    let sprite_byte =
                        self.ram[self.i.wrapping_add(dy as u16) as usize].reverse_bits();
                    for dx in 0u8..8u8 {
                        let screen_x = self.v[x as usize].wrapping_add(dx) % C8_VBUF_WIDTH as u8;
                        let screen_y = self.v[y as usize].wrapping_add(dy) % C8_VBUF_HEIGHT as u8;
                        let prev_vbuf_xy =
                            self.vbuf[screen_x as usize + screen_y as usize * C8_VBUF_WIDTH];
                        self.vbuf[screen_x as usize + screen_y as usize * C8_VBUF_WIDTH] ^=
                            (sprite_byte >> dx) & 0x1;

                        if self.vbuf[screen_x as usize + screen_y as usize * C8_VBUF_WIDTH] == 0
                            && prev_vbuf_xy == 1
                        {
                            collision = true;
                        }
                    }
                }

                self.v[0xF] = if collision { 1 } else { 0 };

                self.pc += 2;
            }
            0xE => match kk {
                0x9E => {
                    // SKP Vx
                    if self.keypad[self.v[x as usize] as usize] == 1 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    // SKNP Vx
                    if self.keypad[self.v[x as usize] as usize] != 1 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => {
                    // Unkown Instruction
                    println!("Unknown instruction: {:X}", instruction);
                    self.pc += 2;
                }
            },
            0xF => match kk {
                0x07 => {
                    // LD Vx, DT
                    self.v[x as usize] = self.delay_timer;
                    self.pc += 2;
                }
                0x0A => {
                    // LD Vx, K
                    self.looking_for_key = true;

                    if self.most_recent_key < 16 && self.keypad[self.most_recent_key as usize] == 0
                    {
                        self.v[x as usize] = self.most_recent_key;
                        self.looking_for_key = false;
                        self.most_recent_key = 16;
                        self.pc += 2;
                    }
                }
                0x15 => {
                    // LD DT, Vx
                    self.delay_timer = self.v[x as usize];
                    self.pc += 2;
                }
                0x18 => {
                    // LD ST, Vx
                    self.sound_timer = self.v[x as usize];
                    self.pc += 2;
                }
                0x1E => {
                    // ADD I, Vx
                    self.i += self.v[x as usize] as u16;
                    self.pc += 2;
                }
                0x29 => {
                    // LD F, Vx
                    self.i = (self.v[x as usize] & 0xf) as u16 * 5;
                    self.pc += 2;
                }
                0x33 => {
                    // LD B, Vx
                    let val = self.v[x as usize];
                    let hundreds = val / 100;
                    let tens = (val % 100) / 10;
                    let ones = val % 10;
                    self.ram[self.i as usize] = hundreds;
                    self.ram[self.i as usize + 1] = tens;
                    self.ram[self.i as usize + 2] = ones;

                    self.pc += 2;
                }
                0x55 => {
                    // LD [I], Vx
                    for reg in 0usize..x as usize + 1 {
                        self.ram[self.i as usize + reg] = self.v[reg];
                    }
                    self.pc += 2;
                    self.i += x + 1; // Quirk
                }
                0x65 => {
                    // LD Vx, [I]
                    for reg in 0usize..x as usize + 1 {
                        self.v[reg] = self.ram[self.i as usize + reg];
                    }
                    self.pc += 2;
                    self.i += x + 1; // Quirk
                }
                _ => {
                    // Unkown instruction
                    println!("Unknown instruction: {:X}", instruction);
                    self.pc += 2;
                }
            },

            _ => {
                // Unknown Instruction
                println!("Unknown instruction: {:X}", instruction);
                self.pc += 2;
            }
        }
    }
    fn get_vbuf(&mut self) -> (Vec<u8>, usize, usize) {
        (
            self.vbuf
                .to_vec()
                .iter()
                .flat_map(|val| {
                    if *val == 0 {
                        self.off_color
                    } else {
                        self.on_color
                    }
                })
                .collect(),
            C8_VBUF_WIDTH,
            C8_VBUF_HEIGHT,
        )
    }
}
