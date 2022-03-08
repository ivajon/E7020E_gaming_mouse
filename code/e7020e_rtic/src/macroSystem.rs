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


    pub fn push_left(&self, &mut mouse: MouseKeyboardState) {
        do_function(self.left_button, mouse);
    }

    pub fn push_right(&self, &mut mouse: MouseKeyboardState) {
        do_function(self.right_button, mouse);
    }

    pub fn push_middle(&self, &mut mouse: MouseKeyboardState) {
        do_function(self.middle_button, mouse);
    }

    pub fn push_side_front(&self, &mut mouse: MouseKeyboardState) {
        do_function(self.side_button_front, mouse);
    }

    pub fn push_side_back(&self, &mut mouse: MouseKeyboardState) {
        do_function(self.side_button_back, mouse);
    }

    pub fn release_left(&self, &mut mouse: MouseKeyboardState) {
        end_function(self.left_button, mouse);
    }

    pub fn release_right(&self, &mut mouse: MouseKeyboardState) {
        end_function(self.right_button, mouse);
    }

    pub fn release_middle(&self, &mut mouse: MouseKeyboardState) {
        end_function(self.middle_button, mouse);
    }

    pub fn release_side_front(&self, &mut mouse: MouseKeyboardState) {
        end_function(self.side_button_front, mouse);
    }

    pub fn release_side_back(&self, &mut mouse: MouseKeyboardState) {
        end_function(self.side_button_back, mouse);
    }

    pub fn scroll_up(&self, &mut mouse: MouseKeyboardState) {
        do_function(self.scroll_up, mouse);
    }

    pub fn scroll_down(&self, &mut mouse: MouseKeyboardState) {
        do_function(self.scroll_down, mouse);
    }
}

fn do_function(f: Function, mouse: MouseKeyboardState) {
    match f {
        LeftClick => {
            mouse.push_left();
        },
        RightClick => {
            mouse.push_right();
        },
        MiddleClick => {
            mouse.push_middle();
        },
        ScrollUp => {
            mouse.wheel_up();
        },
        ScrollDown => {
            mouse.wheel_down();
        },
        PressKeyboard(key) => {
            mouse.push_keybord_key(key);
        },
        _ => (),
    }
}

fn end_function(f: Function, mouse: MouseKeyboardState) {
    match f {
        LeftClick => {
            mouse.release_left();
        },
        RightClick => {
            mouse.release_right();
        },
        MiddleClick => {
            mouse.release_middle();
        },
        PressKeyboard(key) => {
            mouse.release_keybord_key(key);
        },
        _ => (),
    }
}

pub enum Function {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
    PressKeyboard(u8),
    Nothing,
}
