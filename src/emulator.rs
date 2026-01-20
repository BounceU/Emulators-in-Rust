pub trait Emulator: Send {
    fn tick(&mut self);
    fn get_vbuf(&mut self) -> (Vec<u8>, usize, usize);
    fn timer_tick(&mut self);
    fn handle_key(&mut self, code: winit::keyboard::KeyCode, is_pressed: bool);
}

pub struct EmulatorDevice {
    pub device: Box<dyn Emulator + Send>,
    pub cpu_hz: f64,
    pub timer_hz: f64,
}
