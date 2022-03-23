use crate::hidDescriptors::MouseKeyboard;
use rtt_target::{rprintln, rtt_init_print, rprint};

// Include sensor related packages
use crate::pmw3389::Pmw3389;
use stm32f4xx_hal::{
    gpio::{Alternate, Output, Pin, PushPull, Speed},
    prelude::*,
    spi::{Spi, TransferModeNormal},
    timer::Delay,
};
use stm32f4::stm32f401::{SPI1, TIM5};


// Define needed types

type SCK = Pin<Alternate<PushPull, 5_u8>, 'B', 3_u8>;
type MOSI = Pin<Alternate<PushPull, 5_u8>, 'B', 5_u8>;
type MISO = Pin<Alternate<PushPull, 5_u8>, 'B', 4_u8>;
type CS = Pin<Output<PushPull>, 'A', 4_u8>;
type SPI = Spi<SPI1, (SCK, MISO, MOSI), TransferModeNormal>;
type DELAY = Delay<TIM5, 1000000_u32>;
pub struct MouseKeyboardState {
    // mouse part
    x: i8,
    y: i8,
    wheel: i8,
    left_button: bool,
    right_button: bool,
    middle_button: bool,
    // keybord part
    keycode: [u8; 6],
    /// Sensor variable, holds sensor API
    sensor:Pmw3389<SPI,CS,DELAY>,
    last_phase:char,
    scroll_direction:i8
}

impl MouseKeyboardState {
    pub fn new(sensor:Pmw3389<SPI,CS,DELAY>) -> MouseKeyboardState {
        MouseKeyboardState {
            x: 0,
            y: 0,
            wheel: 0,
            left_button: false,
            right_button: false,
            middle_button: false,
            keycode: [0, 0, 0, 0, 0, 0],
            sensor : sensor,
            last_phase:' ',
            scroll_direction : 0,
        }
    }

    pub fn add_x_movement(&mut self, to_add: i8) {
        let result = self.x + to_add;
        if result > i8::MAX {
            self.x = i8::MAX;
        } else if result < i8::MIN {
            self.x = i8::MIN;
        } else {
            self.x = result
        }
    }

    pub fn add_y_movement(&mut self, to_add: i8) {
        let result = self.y + to_add;
        if result > i8::MAX {
            self.y = i8::MAX;
        } else if result < i8::MIN {
            self.y = i8::MIN;
        } else {
            self.y = result
        }
    }
    pub fn handle_scroll(&mut self, phase: char) {
        if self.last_phase == ' ' {
            self.last_phase = phase;
        } else if self.last_phase == phase {
            self.scroll_direction *= -1;
        } 
        if(self.scroll_direction == 1) {
            self.wheel_up();
        } else if(self.scroll_direction == -1) {
            self.wheel_down();
        }
    }
    pub fn wheel_up(&mut self) {
        match self.wheel {
            i8::MAX => (),
            i8::MIN => (),
            _ => self.wheel += 1
        }
    }

    pub fn wheel_down(&mut self) {
        match self.wheel {
            i8::MAX => (),
            i8::MIN => (),
            _ => self.wheel -= 1
        }
    }

    pub fn push_left(&mut self) {
        self.left_button = true;
    }

    pub fn push_right(&mut self) {
        self.right_button = true;
    }

    pub fn push_middle(&mut self) {
        self.middle_button = true;
    }

    pub fn release_left(&mut self) {
        self.left_button = false;
    }

    pub fn release_right(&mut self) {
        self.right_button = false;
    }

    pub fn release_middle(&mut self) {
        self.middle_button = false;
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.wheel = 0;
    }

    pub fn push_keybord_key(&mut self, keycode: u8) {
        for n in 0..self.keycode.len() {
            if self.keycode[n] == 0 {
                self.keycode[n] = keycode;
                return;
            }
        }
    }

    pub fn release_keybord_key(&mut self, keycode: u8) {
        for n in 0..self.keycode.len() {
            if self.keycode[n] == keycode {
                self.keycode[n] = 0;
                return;
            }
        }
    }

    pub fn get_report_and_reset(&mut self) -> MouseKeyboard{
        let ret = MouseKeyboard {
            x: 0,//self.x,
            y: 0,//self.y,
            buttons: make_button(self.left_button, self.middle_button, self.right_button),
            wheel: self.wheel,
            pan: 0,
            //
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: self.keycode.clone(),
        };
        self.reset();
        ret
    }
    pub fn increment_dpi(&mut self,direction : i16){
        self.sensor.increment_dpi(direction);
    }
    pub fn write_dpi(&mut self, dpi: u16) {
        rprintln!("{:}",dpi);
        self.sensor.set_dpi(dpi);
    }


    pub fn read_sensor(&mut self) {
        let status = self.sensor.read_status();
        match(status) {
            Ok(status) => {
                //rprintln!("{:?}",status);
                self.add_x_movement(status.dx as i8);
                self.add_y_movement(status.dy as i8);
            },
            _ => ()
        }
    }

}

fn make_button(left: bool, middle: bool, right: bool) -> u8 {
    u8::from(left) | u8::from(right) << 1 | u8::from(middle) << 2
}