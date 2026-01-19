pub trait Emulator: Send {
    fn tick(&mut self);
    fn get_vbuf(&mut self) -> (Vec<u8>, usize, usize);
    fn timer_tick(&mut self);
}

pub struct EmulatorDevice {
    pub device: Box<dyn Emulator + Send>,
    pub cpu_hz: f64,
    pub timer_hz: f64,
}
