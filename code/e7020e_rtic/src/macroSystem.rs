use crate::mouseKeyboardReport::MouseKeyboardState;

pub struct MacroConfig {
    pub left_button: MacroType,
    pub right_button: MacroType,
    pub middle_button: MacroType,
    pub scroll_up: MacroType,
    pub scroll_down: MacroType,
    pub side_button_front: MacroType,
    pub side_button_back: MacroType,
}

#[derive(Debug)]
pub struct MacroSequence {
    pub functions: [Function; 5],
    pub delays: [u32; 5],
    pub hold_times: [u32; 5],
}

impl MacroConfig {
    pub fn new() -> MacroConfig {
        MacroConfig {
            left_button: MacroType::MacroSingle(Function::LeftClick),
            right_button: MacroType::MacroSingle(Function::RightClick),
            middle_button: MacroType::MacroSingle(Function::MiddleClick),
            scroll_up: MacroType::MacroSingle(Function::ScrollUp),
            scroll_down: MacroType::MacroSingle(Function::ScrollDown),
            side_button_front: MacroType::MacroSingle(Function::Nothing),
            side_button_back: MacroType::MacroSingle(Function::Nothing),
        }
    }

    pub fn update_config(
        &mut self,
        left_button: MacroType,
        right_button: MacroType,
        middle_button: MacroType,
        scroll_up: MacroType,
        scroll_down: MacroType,
        side_button_front: MacroType,
        side_button_back: MacroType,
    ) {
        self.left_button = left_button;
        self.right_button = right_button;
        self.middle_button = middle_button;
        self.scroll_up = scroll_up;
        self.scroll_down = scroll_down;
        self.side_button_front = side_button_front;
        self.side_button_back = side_button_back;
    }
}

pub fn do_function(f: Function, mouse: &mut MouseKeyboardState) {
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

pub fn end_function(f: Function, mouse: &mut MouseKeyboardState) {
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

pub enum MacroType {
    MacroMultiple(MacroSequence),
    MacroSingle(Function),
}

#[derive(Copy, Clone, Debug)]
pub enum Function {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
    PressKeyboard(u8),
    Nothing,
    End,
}
