import Jetson.GPIO as GPIO
import time

# Pin Definitions
servo_pin = 13  # GPIO pin 13

# PWM settings
frequency = 50  # 50 Hz

# Setup GPIO pin
GPIO.setmode(GPIO.BOARD)  # or GPIO.BCM
GPIO.setup(servo_pin, GPIO.OUT)

# Initialize PWM on the servo pin
pwm = GPIO.PWM(servo_pin, frequency)
pwm.start(0)  # Start PWM with 0% duty cycle

def set_servo_angle(angle):
    duty_cycle = (0.05 * 50) + (0.19 * 50 * angle / 180)
    pwm.ChangeDutyCycle(duty_cycle)

try:
    while True:
        # Move the servo to 0 degrees
        set_servo_angle(0)
        time.sleep(1)
        # Move the servo to 90 degrees
        set_servo_angle(90)
        time.sleep(1)
        # Move the servo to 180 degrees
        set_servo_angle(180)
        time.sleep(1)
except KeyboardInterrupt:
    pass

pwm.stop()
GPIO.cleanup()
