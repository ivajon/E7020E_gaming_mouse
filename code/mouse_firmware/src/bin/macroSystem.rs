use crate::mouseKeyboardReport::MouseKeyboardState;

pub struct MacroConfig {
    left_button: Function,
    right_button: Function,
    middle_button: Function,
    scroll_up: Function,
    scroll_down: Function,
    side_button_front: Function,
    side_button_back: Function,
}

struct MacroSequence {
    functions: [Function; 5],
    delays: [u8; 5],
    hold_times: [u8; 5],
}

impl MacroConfig {
    pub fn new() -> MacroConfig {
        MacroConfig {
            left_button: Function::LeftClick,
            right_button: Function::RightClick,
            middle_button: Function::MiddleClick,
            scroll_up: Function::ScrollUp,
            scroll_down: Function::ScrollDown,
            side_button_front: Function::Nothing,
            side_button_back: Function::Nothing,
        }
    }

    pub fn update_config(
        &mut self,
        left_button: Function,
        right_button: Function,
        middle_button: Function,
        scroll_up: Function,
        scroll_down: Function,
        side_button_front: Function,
        side_button_back: Function,
    ) {
        self.left_button = left_button;
        self.right_button = right_button;
        self.middle_button = middle_button;
        self.scroll_up = scroll_up;
        self.scroll_down = scroll_down;
        self.side_button_front = side_button_front;
        self.side_button_back = side_button_back;
    }


    pub fn push_left(&self, mouse: &mut MouseKeyboardState) {
        do_function(self.left_button, mouse);
    }

    pub fn push_right(&self, mouse: &mut MouseKeyboardState) {
        do_function(self.right_button, mouse);
    }

    pub fn push_middle(&self, mouse: &mut MouseKeyboardState) {
        do_function(self.middle_button, mouse);
    }

    pub fn push_side_front(&self, mouse: &mut MouseKeyboardState) {
        do_function(self.side_button_front, mouse);
    }

    pub fn push_side_back(&self, mouse: &mut MouseKeyboardState) {
        do_function(self.side_button_back, mouse);
    }

    pub fn release_left(&self, mouse: &mut MouseKeyboardState) {
        end_function(self.left_button, mouse);
    }

    pub fn release_right(&self, mouse: &mut MouseKeyboardState) {
        end_function(self.right_button, mouse);
    }

    pub fn release_middle(&self, mouse: &mut MouseKeyboardState) {
        end_function(self.middle_button, mouse);
    }

    pub fn release_side_front(&self, mouse: &mut MouseKeyboardState) {
        end_function(self.side_button_front, mouse);
    }

    pub fn release_side_back(&self, mouse: &mut MouseKeyboardState) {
        end_function(self.side_button_back, mouse);
    }

    pub fn scroll_up(&self, mouse: &mut MouseKeyboardState) {
        do_function(self.scroll_up, mouse);
    }

    pub fn scroll_down(&self, mouse: &mut MouseKeyboardState) {
        do_function(self.scroll_down, mouse);
    }
}

fn do_function(f: Function, mouse: &mut MouseKeyboardState) {
    match f {
        Function::LeftClick => {
            mouse.push_left();
        },
        Function::RightClick => {
            mouse.push_right();
        },
        Function::MiddleClick => {
            mouse.push_middle();
        },
        Function::ScrollUp => {
            mouse.wheel_up();
        },
        Function::ScrollDown => {
            mouse.wheel_down();
        },
        Function::PressKeyboard(key) => {
            mouse.push_keybord_key(key);
        },
        _ => (),
    }
}

fn end_function(f: Function, mouse: &mut MouseKeyboardState) {
    match f {
        Function::LeftClick => {
            mouse.release_left();
        },
        Function::RightClick => {
            mouse.release_right();
        },
        Function::MiddleClick => {
            mouse.release_middle();
        },
        Function::PressKeyboard(key) => {
            mouse.release_keybord_key(key);
        },
        _ => (),
    }
}

#[derive(Copy, Clone)]
pub enum Function {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
    PressKeyboard(u8),
    Nothing,
    Macro1,
    Macro2,
    Macro3,
}
