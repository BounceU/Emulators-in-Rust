use wgpu_test::chip8::Chip8;
use wgpu_test::emulator::EmulatorDevice;

#[tokio::main]
async fn main() {
    let mut chip8 = Chip8::new("src/1-chip8-logo.ch8");
    // let mut chip8 = Chip8::new("src/2-ibm-logo.ch8");
    // let mut chip8 = Chip8::new("src/3-corax+.ch8");
    // let mut chip8 = Chip8::new("src/4-flags.ch8");
    chip8.set_colors([136, 192, 112], [8, 24, 32]);

    let emulator_device = Some(EmulatorDevice {
        device: Box::new(chip8),
        timer_hz: 60.0,
        cpu_hz: 1000.0,
    });

    let _ = wgpu_test::run(emulator_device);
    println!("Exiting...");
}
