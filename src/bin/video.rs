use ferropin::{display::ssd1306::Ssd1306, i2c::hardware::HardwareI2c};
use std::io::Read;
use std::process::{Command, Stdio};

const WIDTH: usize = 128;
const HEIGHT: usize = 64;

fn main() -> ferropin::error::Result<()> {
    // Initialize display
    let i2c = HardwareI2c::new(1)?;
    let mut display = Ssd1306::new(i2c)?;

    // Spawn ffmpeg process to stream raw grayscale frames
    let mut ffmpeg = Command::new("ffmpeg")
        .args(&[
            "-i",
            "/home/outofrange/video.mp4",
            "-vf",
            &format!("scale={}:{}", WIDTH, HEIGHT),
            "-f",
            "rawvideo",
            "-pix_fmt",
            "gray",
            "-",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start ffmpeg");

    let stdout = ffmpeg.stdout.as_mut().unwrap();
    let frame_size = WIDTH * HEIGHT;
    let mut buffer = vec![0u8; frame_size];

    // Read frames in a loop
    while stdout.read_exact(&mut buffer).is_ok() {

        for y in 0..HEIGHT {
            let row = &buffer[y * WIDTH..(y + 1) * WIDTH];
            for (x, &pixel) in row.iter().enumerate() {
                display.set_pixel(x, y, pixel > 128);
            }
        }

        display.flush()?;
    }
    display.clear();

    Ok(())
}
