pub trait Emulator {
    fn tick(&mut self);
    fn get_vbuf(&mut self) -> (Vec<u8>, usize, usize);
    fn sixty_hz_tick(&mut self);
}

pub struct EmulatorDevice<E: Emulator> {
    pub device: E,
}
