use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define GPIO pin (replace with your actual pin number)
    let gpio_pin = "/sys/class/gpio/gpio391/value"; // Example GPIO pin path

    // Function to write value to GPIO pin
    fn write_gpio(pin_path: &str, value: u8) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).open(pin_path)?;
        write!(file, "{}", value)?;
        Ok(())
    }

    // PWM parameters
    let frequency = 50; // PWM frequency in Hz
    let period = 1.0 / frequency as f64; // Period in seconds
    let pulse_width = 0.0015; // Pulse width in seconds (1.5 ms for middle position)

    // Main loop
    let start_time = Instant::now();
    loop {
        let elapsed = start_time.elapsed().as_secs_f64();
        let current_time = elapsed % period;

        if current_time < pulse_width {
            // Set GPIO high
            write_gpio(gpio_pin, 1)?;
        } else {
            // Set GPIO low
            write_gpio(gpio_pin, 0)?;
        }

        // Sleep to control frequency
        thread::sleep(Duration::from_micros((period * 1_000_000.0) as u64) - Duration::from_micros((pulse_width * 1_000_000.0) as u64));
    }
}
