use embedded_hal::{
    i2c::{I2c},

}
use rtt_target::rprintln;


// Defines a register map for the rgb controller
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum register_map {
    MODE1        = 0x00,
    MODE2        = 0x01,
    PWM0         = 0x02,
    PWM1         = 0x03,
    PWM2         = 0x04,
    PWM3         = 0x05,
    PWM4         = 0x06,
    PWM5         = 0x07,
    PWM6         = 0x08,
    PWM7         = 0x09,
    GRPPWM       = 0x0A,
    GRPFREQ      = 0x0B,
    LEDOUT0      = 0x0C,
    LEDOUT1      = 0x0D,
    SUBADR1      = 0x0E,
    SUBADR2      = 0x0F,
    SUBADR3      = 0x10,
    ALLCALLADR   = 0x11,
}

// Define the data field
struct PCA9624PW<I2C>{
    i2c : I2c,
}
// Define a driver
impl<I2C> PCA9624PW<I2C>
where
    I2C : I2c<I2C: Instance, PINS>
{
    // Define self
    let pca9624PW = PCA9624PW{
        I2C
    }


}