use usbd_hid::descriptor::MouseReport;

pub struct MouseState {
    x: i8,
    y: i8,
    wheel: i8,
    left_button: bool,
    right_button: bool,
    middle_button: bool,
}

impl MouseState {
    pub fn new() -> MouseState {
        MouseState {
            x: 0,
            y: 0,
            wheel: 0,
            left_button: false,
            right_button: false,
            middle_button: false
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
        self.left_button = false;
        self.right_button = false;
        self.middle_button = false;
    }

    pub fn get_report_and_reset(&mut self) -> MouseReport{
        let ret = MouseReport {
            x: self.x,
            y: self.y,
            buttons: make_button(self.left_button, self.middle_button, self.right_button),
            wheel: self.wheel,
            pan: 0
        };
        self.reset();
        ret
    }
}

fn make_button(left: bool, middle: bool, right: bool) -> u8 {
    u8::from(left) | u8::from(right) << 1 | u8::from(middle) << 2
}