use crate::mouseKeyboardReport::MouseKeyboardState;

pub const MACRO_SIZE: usize = 5;

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
            (self.functions[i], 0, self.hold_times[i])
        } else {
            (self.functions[i], self.delays[i+1], self.hold_times[i])
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
    
    fn change_macro_function(&mut self, macro_nr: usize, index: usize, f: Function) {
        if macro_nr < self.macros.len() {
            let m_sequence = &mut self.macros[macro_nr];
            m_sequence.functions[index] = f;
        }
    }

    fn change_macro_time(&mut self, macro_nr: usize, index: usize, time: u32) {
        if macro_nr < self.macros.len() {
            let m_sequence = &mut self.macros[macro_nr];
            m_sequence.hold_times[index] = time;
        }
    }

    fn change_macro_delay(&mut self, macro_nr: usize, index: usize, delay: u32) {
        if macro_nr < self.macros.len() {
            let m_sequence = &mut self.macros[macro_nr];
            m_sequence.delays[index] = delay;
        }
    }
    
    fn change_button_mapping(&mut self, button_id: u8, m: MacroType) {
        match button_id {
            0 => self.left_button = m,
            1 => self.right_button = m,
            2 => self.middle_button = m,
            3 => self.scroll_up = m,
            4 => self.scroll_down = m,
            5 => self.side_button_front = m,
            6 => self.side_button_back = m,
            _ => ()
        }
    }

    /// this handle configuration via a binary blob
    /// 
    /// 8 byte in total
    /// 
    /// 1 byte - deside system <- don't touch
    /// 2 byte - subcommand
    /// 3 -8 byte - data
    /// 
    /// macro nr (8 bit)
    /// 
    /// Subcommands:
    /// 0 : change button to single function - args: button id, function id, keycode
    /// 1 : change button to multiple macro - args: button id, macro nr
    /// 2 : change function in macro - args: macro nr, index (8 bit),  Function id, keycode
    /// 3 : change delay in macro - args: macro nr, index (8 bit), data
    /// 4 : change time in macro - args: macro nr, index (8 bit), data
    /// 
    /// Button ids (8 bit):
    /// 0 left
    /// 1 right
    /// 2 middle
    /// 3 scroll-up
    /// 4 scroll-down
    /// 5 front
    /// 6 back
    /// 
    /// Function ids (8 bit):
    /// 0 leftclick
    /// 1 rightclick
    /// 2 middleclick
    /// 3 scroll-up
    /// 4 scroll-down
    /// 5 dpi-up
    /// 6 dpi-down
    /// 7 push-key
    /// 8 End
    /// 9 Nothing
    /// 
    pub fn handle_binary_config(&mut self, bytes: &[u8; 8]) {
        let subcommand: u8 = bytes[1];

        match subcommand {
            0 => {
                self.change_button_mapping(
                    bytes[2],
                    MacroType::MacroSingle(
                        Function::from_id(
                            bytes[3], bytes[4]
                        )
                    )
                );
            }
            1 => {
                self.change_button_mapping(
                    bytes[2],
                    MacroType::MacroMultiple(usize::from(bytes[3]))
                );
            }
            2 => {
                self.change_macro_function(
                    usize::from(bytes[2]),
                    usize::from(bytes[3]),
                    Function::from_id(bytes[4], bytes[5])
                );
            }
            3 => {
                self.change_macro_delay(
                    usize::from(bytes[2]),
                    usize::from(bytes[3]),
                    u32::from_be_bytes(bytes[4..8].try_into().unwrap())
                );
            }
            4 => {
                self.change_macro_time(
                    usize::from(bytes[2]),
                    usize::from(bytes[3]),
                    u32::from_be_bytes(bytes[4..8].try_into().unwrap())
                )
            }
            _ => ()
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
            mouse.scroll_up();
        },
        Function::ScrollDown => {
            mouse.scroll_down();
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
    DpiUp,
    DpiDown,
    PressKeyboard(u8),
    Nothing,
    End,
}

impl Function {
    /// Function ids (8 bit):
    /// 0 leftclick
    /// 1 rightclick
    /// 2 middleclick
    /// 3 scroll-up
    /// 4 scroll-down
    /// 5 dpi-up
    /// 6 dpi-down
    /// 7 push-key
    /// 8 End
    /// 9 Nothing
    fn from_id(id: u8, keycode: u8) -> Function {
        match id {
            0 => Function::LeftClick,
            1 => Function::RightClick,
            2 => Function::MiddleClick,
            3 => Function::ScrollUp,
            4 => Function::ScrollDown,
            5 => Function::DpiUp,
            6 => Function::DpiDown,
            7 => Function::PressKeyboard(keycode),
            8 => Function::End,
            9 => Function::Nothing,
            _ => Function::Nothing,
        }
    }
}
