use gpiod::{Chip, Options};
use std::{thread, time::{Duration, Instant}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the GPIO chip (usually gpiochip0 on Jetson devices)
    let chip = Chip::new("/dev/gpiochip0")?;

    // Get the GPIO line (replace with the actual line number for your GPIO pin)
    let line = chip.request_lines(Options::output(&[13]))?;

    // Request the line as output
    let handle = line;

    // Define PWM parameters
    let frequency = 50; // 50 Hz for servo
    let period = Duration::from_micros(1_000_000 / frequency); // Period in microseconds
    let min_pulse = Duration::from_micros(1000); // Minimum pulse width in microseconds
    let max_pulse = Duration::from_micros(2000); // Maximum pulse width in microseconds

    // Function to convert angle to pulse width
    fn angle_to_pulse(angle: f64, min_pulse: Duration, max_pulse: Duration) -> Duration {
        let pulse_range = max_pulse - min_pulse;
        min_pulse + pulse_range.mul_f64(angle / 180.0)
    }

    // Move the servo to specific angles
    let angles = [0.0, 90.0, 180.0];
    for &angle in &angles {
        let pulse = angle_to_pulse(angle, min_pulse, max_pulse);
        println!("Setting angle to {} degrees, pulse width to {:?}", angle, pulse);

        let start_time = Instant::now();
        while start_time.elapsed() < Duration::from_secs(1) {
            handle.set_values(1u8)?;
            thread::sleep(pulse);

            handle.set_values(0u8)?;
            thread::sleep(period - pulse);
        }
    }

    Ok(())
}
