use crate::matr_math::*;

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
fn remap(val : f32)->u8{
    
    let ret : u8 =  (val*255.0) as u8;
    ret
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
    /// Creats a color from a vector
    pub fn from_vec_3(data : vec_3)->Color{
        Color { r: remap(data.data[0]), g: remap(data.data[1]), b: remap(data.data[2]) }
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
    /// Converts a color to r3
    pub fn to_vec_3(&mut self) -> vec_3{
        vec_3::from_color_values(self.r, self.g, self.b)
    }

}