use crate::mouseKeyboardReport::MouseKeyboardState;

const MACRO_SIZE: usize = 5;

pub struct MacroConfig {
    pub left_button: MacroType,
    pub right_button: MacroType,
    pub middle_button: MacroType,
    pub scroll_up: MacroType,
    pub scroll_down: MacroType,
    pub side_button_front: MacroType,
    pub side_button_back: MacroType,
    pub macros: [MacroSequence; 5],
}

#[derive(Debug)]
pub struct MacroSequence {
    pub functions: [Function; MACRO_SIZE],
    pub delays: [u32; MACRO_SIZE],
    pub hold_times: [u32; MACRO_SIZE],
}

impl MacroSequence {
    fn new() -> MacroSequence {
        MacroSequence {
            functions: [Function::End; MACRO_SIZE],
            delays: [0; MACRO_SIZE],
            hold_times: [0; MACRO_SIZE],
        }
    }

    fn get_firs_delay(&self) -> u32 {
        self.delays[0]
    }

    fn get_parameters(&self, i: usize) -> (Function, u32, u32) {
        if i == MACRO_SIZE - 1 {
            (self.functions[i], self.delays[i+1], self.hold_times[i])
        } else {
            (self.functions[i], 0, self.hold_times[i])
        }
    }

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
            macros: [
                MacroSequence::new(),
                MacroSequence::new(),
                MacroSequence::new(),
                MacroSequence::new(),
                MacroSequence::new(),
            ]
        }
    }

    pub fn get_macro_first_delay(&self, macro_nr: usize) -> u32 {
        self.macros[macro_nr].get_firs_delay()
    }

    pub fn get_macro_params(&self, macro_nr: usize, sequence_nr: usize) -> (Function, u32, u32) {
        self.macros[macro_nr].get_parameters(sequence_nr)
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

    pub fn change_macro(&mut self, macro_nr: usize, functions: [Function; 5], delays: [u32; 5], hold_times: [u32; 5]) {
        if macro_nr < self.macros.len() {
            let m_sequence = &mut self.macros[macro_nr];
            copy_array::<Function>(functions, &mut m_sequence.functions);
            copy_array::<u32>(delays, &mut m_sequence.delays);
            copy_array::<u32>(hold_times, &mut m_sequence.hold_times);
        }
    }
}

fn copy_array<T: Copy>(sorce: [T; MACRO_SIZE], destination: &mut [T]) {
    if sorce.len() != destination.len() {
        return;
    }

    for i in 0..sorce.len() {
        destination[i] = sorce[i];
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

#[derive(Copy, Clone)]
pub enum MacroType {
    MacroMultiple(usize),
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
