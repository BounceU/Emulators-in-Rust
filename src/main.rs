use clap::Parser;
use wgpu_test::chip8::Chip8;
use wgpu_test::emulator::EmulatorDevice;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Filepath to the rom file you want to emulate
    #[arg(
        short,
        long,
        long_help = "Filepath to the rom file you want to emulate"
    )]
    filepath: String,

    // Name of the emulator you want to use (i.e. Chip8, NES, Gameboy)
    #[arg(short, long, default_value_t = String::from("Chip8"), long_help="Name of the emulator you want to use (i.e. Chip8, NES, Gameboy)")]
    emulator: String,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut chip8 = Chip8::new(&args.filepath);
    chip8.set_colors([136, 192, 112], [8, 24, 32]);

    let emulator_device = Some(EmulatorDevice {
        device: Box::new(chip8),
        timer_hz: 60.0,
        cpu_hz: 1000.0,
    });

    let _ = wgpu_test::run(emulator_device);
    println!("Exiting...");
}
