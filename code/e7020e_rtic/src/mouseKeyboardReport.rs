use crate::hidDescriptors::MouseKeyboard;
use rtt_target::{rprintln, rtt_init_print};

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
}

impl MouseKeyboardState {
    pub fn new() -> MouseKeyboardState {
        MouseKeyboardState {
            x: 0,
            y: 0,
            wheel: 0,
            left_button: false,
            right_button: false,
            middle_button: false,
            keycode: [0, 0, 0, 0, 0, 0]
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
        rprintln!("push_right state");
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
            x: self.x,
            y: self.y,
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
}

fn make_button(left: bool, middle: bool, right: bool) -> u8 {
    u8::from(left) | u8::from(right) << 1 | u8::from(middle) << 2
}