//! Defines a pattern mechanic for the rgb controller
//! TODO : implement function based patterns
use crate::pca9624_pw::{self, *};
use crate::color::{self, *};
// Import all of the matrix math, can be usefull
use crate::matr_math::*;
const max_pattern_length: usize = 255;
const number_of_patterns: usize = 4;
/// Field reprensentation
/// ColorChange.0 = interface id
/// color_change.1 = color 
type ColorChange = (u8,Color);
#[derive(Copy, Clone)]
pub struct Pattern{
    /// The list of changes to go through
    /// To add changes instantly to both interaces
    /// we need to add them in turn to the list
    /// and set the delay in between them to 0
    color_changes : [ColorChange;max_pattern_length],
    /// The list of delays between changes
    delays : [u16;max_pattern_length],
    /// The length of the pattern
    /// This is the number of changes in the pattern
    /// and the number of delays in the pattern
    length : usize,
    /// The current index in the animation
    index : usize,
    /// The displacement matrix for the pattern, if this is not empty it will be used instead of the color_changes
    matrix : matr_3,
    /// Will be used if the matrix is used
    time_step : u16
}
#[allow(dead_code)]
impl Pattern{
    /// Constructs a new empty pattern
    pub fn new() -> Pattern{
        Pattern{
            color_changes : [(0,Color::new(0,0,0));max_pattern_length],
            delays : [0;max_pattern_length],
            length : 0,
            index : 0,
            matrix : matr_3::new([  
                                        [0.0,0.0,0.0],
                                        [0.0,0.0,0.0],
                                        [0.0,0.0,0.0]],false),
            time_step: 0
        }
    }
    /// Constructs a new pattern from a matrix, this allows 
    pub fn new_with_matrix(mut matrix :  matr_3,time_step : u16)-> Pattern{
        matrix.enabled = true;
        Pattern{
            color_changes : [(0,Color::new(0,0,0));max_pattern_length],
            delays : [0;max_pattern_length],
            length : 1,
            index : 0,
            matrix : matrix,
            time_step : time_step

        }
    }
    

    pub fn get_current_color_change(&mut self) -> (ColorChange,u16){
        if !self.matrix.enabled{
            if (self.index >= self.length) {
                let color : ColorChange = self.color_changes[self.index];
                let delay : u16 = self.delays[self.index];
                (color,delay)
            }
            else{
                let color : Color = Color::new(0,0,0);
                let delay : u16 = u16::MAX; // Wait for ever kinda
                self.index += 1;
                ((0,color),delay)
            }
        }
        else{
            // Handle matrix things i.e. rotations etc
                let color : Color = Color::new(0,0,0);
                let delay : u16 = u16::MAX; // Wait for ever kinda
                self.index += 1;
                ((0,color),delay)
        }
    }
    pub fn add_color_change(&mut self,color_change : ColorChange,delay:u16){
        if !self.matrix.enabled{
            if (self.length < max_pattern_length){
                self.color_changes[self.length] = color_change;
                self.delays[self.length] = delay;
                self.length += 1;
            }
        }
    }
    /// Clears out the current pattern
    /// 
    /// and sets the length to 0
    pub fn clear(&mut self){
        self.length = 0;
        self.index = 0;
    }
    /// Modifies a color at a specific index
    pub fn modify_color(&mut self,index:usize,color:Color){
        if (index < self.length){
            self.color_changes[index].1 = color;
        }
    }
    /// Modifies a delay at a specific index
    pub fn modify_delay(&mut self,index:usize,delay:u16){
        if (index < self.length){
            self.delays[index] = delay;
        }
    }
    /// Modifies delay and color at a specific index
    pub fn modify_color_and_delay(&mut self,index:usize,color:Color,delay:u16){
        if (index < self.length){
            self.color_changes[index].1 = color;
            self.delays[index] = delay;
        }
    }
    /// Goes to the next colour in the pattern
    pub fn next_color(&mut self) -> (ColorChange,u16){
        self.index = (self.index+1)%self.length;
        // Returns the next color and delay
        self.get_current_color_change()
    }
    /// Makes a simple fade pattern for both channels
    /// It only fades in one of the color lines
    /// thus simple
    /// 
    /// Colorspace = 0 => green, colorspace = 1 => blue, colorspace = 2 => red
    pub fn simple_fade( increment : u8,color_space : u8,delay:u16) -> Pattern{
        let mut pattern = Pattern::new();
        // Return if not a valid color space
        if color_space > 2{
            return pattern;
        }

        // Define the pattern
        for i in (0..255).step_by(increment as usize){
            let mut current_color = Color::new(0,0,0);
            let mut hex = current_color.to_hex();
            // Modifie the color space specified
            hex = hex&!((1<<(2*(color_space+1)))-1);
            hex |= i << 2*color_space;
            pattern.add_color_change((0,Color::from_hex(hex)), delay)
        }

        //  Return the pattern
        pattern
    }

}
// TODO : Predefined patterns





pub struct RgbController{
    pca : pca9624_pw::PCA9624PW,
    /// The list of patterns avaliable
    patterns : [Pattern;number_of_patterns],
    /// The number of patterns currently stored
    num_patterns : usize,
    /// The current pattern
    current_pattern : usize,
}
impl RgbController{

    pub fn new(pca : pca9624_pw::PCA9624PW) -> RgbController{
        RgbController{
            pca : pca,
            patterns : [Pattern::new();number_of_patterns],
            num_patterns : 1,
            current_pattern : 0,
        }
    }
    /// Handles api calls
    /// 
    /// Api is structerd after most common
    /// 
    /// 0 => adds color to current pattern
    /// 
    ///     next three bytes are color
    /// 
    ///     orderd by rgb
    /// 
    ///         next two bytes is delay associated
    /// 
    /// 1 => changes current patters
    /// 
    ///     Byte 0: specifies indecie
    pub fn handle_api(&mut self,args : [u8;8]){
        let task = args[1];
        match task{
            0=>{
                let mut color = Color::new(args[2],args[3],args[4]);
                let mut delay = (args[5] as u16)<<8|args[6] as u16; 
                self.patterns[self.current_pattern].add_color_change((0,color), delay);
            }
            1=>{
                if(args[2]<=self.num_patterns as u8){
                    self.current_pattern = args[2] as usize;
                }
            }
            _=>{

            }
        }
    }

    /// Appends a pattern to the list of patterns
    pub fn add_pattern(&mut self, pattern : Pattern){
        if self.num_patterns == number_of_patterns{
            return;
        }
        self.patterns[self.num_patterns] = pattern;
        self.num_patterns+=1;
    }
    /// Sets the current pattern
    pub fn set_pattern(&mut self,index:usize){
        if (index < self.num_patterns){
            self.current_pattern = index;
        }
    }
    /// Increment current pattern
    pub fn increment_pattern(&mut self){
        self.current_pattern = (self.current_pattern+1)%self.num_patterns;
    }
    ////
    /// Get current color and delay
    /// and increment the index
    /// 
    /// Returns (color,delay)
    pub fn next_color(&mut self) -> ((u8,Color),u16){
        let next = self.patterns[self.current_pattern].next_color();
        
        self.pca.set_colour(next.0.0, next.0.1);
        self.pca.write_colours(next.0.0);
        next
    }
    
}