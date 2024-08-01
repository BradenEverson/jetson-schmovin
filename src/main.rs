use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

const SERVO_PIN: u64 = 18; // Change this to your servo's GPIO pin number
const PWM_PERIOD: u64 = 20000; // Period in microseconds for a 50Hz frequency

fn set_servo_pulse(pin: &Pin, pulse_width: u64) -> sysfs_gpio::Result<()> {
    pin.set_value(1)?;
    sleep(Duration::from_micros(pulse_width));
    pin.set_value(0)?;
    sleep(Duration::from_micros(PWM_PERIOD - pulse_width));
    Ok(())
}

fn main() -> sysfs_gpio::Result<()> {
    let servo_pin = Pin::new(SERVO_PIN);
    servo_pin.export()?;
    servo_pin.set_direction(Direction::Out)?;

    let min_pulse = 500;  // Minimum pulse length in microseconds
    let max_pulse = 2500; // Maximum pulse length in microseconds

    // Move servo to min position
    for _ in 0..50 {
        set_servo_pulse(&servo_pin, min_pulse)?;
    }
    sleep(Duration::from_secs(1));

    // Move servo to max position
    for _ in 0..50 {
        set_servo_pulse(&servo_pin, max_pulse)?;
    }
    sleep(Duration::from_secs(1));

    // Move servo to middle position
    let middle_pulse = (min_pulse + max_pulse) / 2;
    for _ in 0..50 {
        set_servo_pulse(&servo_pin, middle_pulse)?;
    }
    sleep(Duration::from_secs(1));

    servo_pin.unexport()?;
    Ok(())
}
