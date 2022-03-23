
use stm32f4::stm32f401::I2C3;
use stm32f4xx_hal::i2c::*;
use stm32f4xx_hal::gpio::*;


/// Color type for the RGB LED
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Color{  
    /// Red
    /// 0-255
    pub r : u8,
    /// Green
    /// 0-255
    pub g : u8,
    /// Blue
    /// 0-255
    pub b : u8,
}

impl Color
{
    
    //=================================================
    // Constructors
    //=================================================
    pub fn new(r : u8, g : u8, b : u8) -> Color
    {
        Color
        {
            r : r,
            g : g,
            b : b,
        }
    }
    pub fn from_hex(hex : u32) -> Color
    {
        let r = (hex >> 16) as u8;
        let g = (hex >> 8) as u8;
        let b = (hex >> 0) as u8;
        Color
        {
            r : r,
            g : g,
            b : b,
        }
    }
    //=================================================
    // Modifiers
    //=================================================
    pub fn invert(&mut self)
    {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }
    pub fn modfie_from_hex(&mut self, hex : u32)
    {
        let r = (hex >> 16) as u8;
        let g = (hex >> 8) as u8;
        let b = (hex >> 0) as u8;
        self.r = r;
        self.g = g;
        self.b = b;
    }
    //=================================================
    // Representers
    //=================================================
    /// Returns a hex representation of the color
    pub fn to_hex(&self) -> u32
    {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        (r << 16) | (g << 8) | b
    }
}



//This should really be improved
// Not using erased pins is dumb
/// SCL pin for the I2C
type SCL = Pin<Alternate<OpenDrain, 4_u8>, 'A', 8_u8>;
/// SDA pin for the I2C
type SDA = Pin<Alternate<OpenDrain, 4_u8>, 'C', 9_u8>;
/// I2C object
type I2C = I2c<I2C3, (SCL, SDA)>;

// Defines a register map for the rgb controller
/// Register map for the rgb controller
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
// Defines a interface for one rgb row
/// Interface for one rgb row
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct interface{
    /// Register that represents the red channel
    r : register_map,
    /// Register that represents the green channel
    g : register_map,
    /// Register that represents the blue channel
    b : register_map,
    color : Color
}
impl interface{
    /// Creates a new rgb interface
    pub fn new(r:register_map,g:register_map,b:register_map) -> interface{
        interface{
            r : r,
            g : g,
            b : b,
            color : Color::new(0,0,0)
        }
    }
    //================================================
    // Setters
    //================================================
    /// Sets the color of the interface
    /// 0 = off, 255 = full brightness
    pub fn set_all_channels(&mut self,color : Color){
        self.color = color;
    }
    /// Sets the color of the interface
    /// 0 = off, 255 = full brightness
    pub fn set_all_channels_from_value(&mut self,r:u8,g:u8,b:u8){
        self.color = Color::new(r,g,b);
        self.set_r(r);
        self.set_g(g);
        self.set_b(b);
    }
    /// Sets the red channel
    /// 0 = off, 255 = full brightness
    pub fn set_r(&mut self,val:u8){
        let offset : u32 = 8 ;
        let mask   : u32 = (u8::MAX as u32) << offset;
        let hex    : u32 = self.color.to_hex() & !mask;
    }
    /// Sets the green channel
    /// 0 = off, 255 = full brightness
    pub fn set_g(&mut self,val:u8){
        let offset : u32 = 8 ;
        let mask   : u32 = (u8::MAX as u32) << offset;
        let hex    : u32 = self.color.to_hex() & !mask;
        self.color.modfie_from_hex(hex | ((val as u32)<<offset));
        
    }
    /// Sets the brightness of the blue channel
    /// 0 = off, 255 = full brightness
    pub fn set_b(&mut self,val:u8){
        let offset : u32 = 0;
        let mask   : u32 = (u8::MAX as u32) << offset;
        let hex    : u32 = self.color.to_hex() & !mask;
        self.color.modfie_from_hex(hex | ((val as u32)<<offset));
    }
    //================================================
    // Getters
    //================================================
    /// Returns the values of all the channels in the interface
    /// 0 = off, 255 = full brightness
    /// [r,g,b]
    pub fn get_all_channels(&self) -> [u8;3]{
        [self.color.r,self.color.g,self.color.b]
    }
    
    /// Returns an array of the interface registers
    /// [r,g,b]
    pub fn get_registers(&self) -> [register_map;3]{
        [self.r,self.g,self.b]
    }

}
pub fn standard_interfaces() -> [interface;2]{
    [interface::new(register_map::PWM0,register_map::PWM1,register_map::PWM2),
    interface::new(register_map::PWM3,register_map::PWM4,register_map::PWM5)]
}

// Define the data field
pub struct PCA9624PW{
    /// I2C bus that will be used, defining it like this makes it agnostic to the 
    /// target used.
    i2c : I2C,
    /// The interface for the rgb rows,
    /// Since we have 2 rows we need to define 2 interfaces
    interfaces : [interface; 2],
    /// The address of the rgb controller
    address : u8,
}
// Define a driver
impl PCA9624PW {
    pub fn new(i2c : I2C, interfaces : [interface;2],address : u8) -> PCA9624PW {
        PCA9624PW{
            i2c : i2c,
            interfaces : interfaces,
            address : address
        }
    }
    // reads who am i reg, not sure if this is correct
    pub fn whoami(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.i2c.write(self.address, &[register_map::SUBADR1 as u8]).unwrap();
        self.i2c.read(self.address, &mut buf).unwrap();
        buf[0]
    }
    /// Writes the values of the interface to the rgb controller
    pub fn write_register(&mut self, reg : register_map, data : u8) {
        let mut buf = [0; 2];
        buf[0] = reg as u8;
        buf[1] = data;
        self.i2c.write(self.address, &buf).unwrap();
    }
    /// Reads the values of the interface from the rgb controller
    pub fn read_register(&mut self, reg : register_map) -> u8 {
        let mut buf = [0; 1];
        buf[0] = reg as u8;
        let mut buf2 = [0; 1];
        self.i2c.write_read(self.address, &buf, &mut buf2).unwrap();
        buf2[0]
    }

    pub fn write_colours(&mut self, interface_id : u8) {
        let mut regs = self.interfaces[interface_id as usize].get_registers();
        let mut color_vals = self.interfaces[interface_id as usize].get_all_channels();
        for i in 0..3 {
            self.write_register(regs[i], color_vals[i]);
        }
    }
    // Writes to a set of 3 registers
    pub fn set_colour_from_values(&mut self,interface_id : u8, r : u8, g : u8, b : u8) {
        // Since we only use the 2 paths
        if interface_id > 1 {
            panic!("Interface must be between 0 and 1");
        }
        self.interfaces[interface_id as usize].set_all_channels_from_value(r,g,b);
    }
    // Writes to a set of 3 registers
    pub fn set_colour(&mut self,interface_id : u8,color : Color) {
        // Since we only use the 2 paths
        if interface_id > 1 {
            panic!("Interface must be between 0 and 1");
        }
        self.interfaces[interface_id as usize].set_all_channels(color);
    }

}