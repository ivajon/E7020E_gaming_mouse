use core::task;
use stm32f4xx_hal::gpio::*;
use crate::mouseKeyboardReport::MouseKeyboardState;
type Button = ErasedPin<Input<PullDown>>;
type Mousestate = MouseState;
pub struct mouse_handeler{
    left_button:  Button,
    right_button: Button,
    middle_button: Button,
    scroll_up: Button,
    scroll_down: Button,
    side_button_front: Button,
    side_button_back: Button,
    mouse : MouseState
}

impl mouse_handeler

{ 
    pub fn new
        (left_button : Button,
        right_button : Button,
        middle_button : Button,
        scroll_up : Button,
        scroll_down : Button,
        side_button_front : Button,
        side_button_back : Button,
        mouse : Mousestate) -> mouse_handeler
    {
        
            let mut mouse_handeler = mouse_handeler{
                left_button:  left_button,
                right_button: right_button,
                middle_button: middle_button,
                scroll_up: scroll_up,
                scroll_down: scroll_down,
                side_button_front: side_button_front,
                side_button_back: side_button_back,
                mouse : mouse,
            };
            return mouse_handeler;  
        }

        pub fn left_button_handle(&mut self){
            if self.left_button.is_low(){
                self.mouse.push_left();
            }
            else{
                self.mouse.release_left();
            }
        }
        pub fn right_button_handle(&mut self){
            if self.right_button.is_low(){
                self.mouse.push_right();
            }
            else{
                self.mouse.release_right();
            }
        }
        
        pub fn middle_button_handle(&mut self){
            if self.middle_button.is_low(){
                self.mouse.push_middle();
            }
            else{
                self.mouse.release_middle();
            }
        }

        pub fn scroll_handle(&mut self){
            if self.scroll_up.is_low(){
            }
        }

}
