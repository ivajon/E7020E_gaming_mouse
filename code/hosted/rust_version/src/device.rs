use hidapi::DeviceInfo;
use hidapi::HidApi;
use hidapi::HidDevice;
use std::string;
pub type VID = u16;
pub type PID = u16;
pub type DEVICE = (VID,PID);
pub type DATA = [u8;8];
enum functionId{
    leftclick   = 0,
    rightclick  = 1,
    middleclick = 2,
    scroll_up   = 3,
    scroll_down = 4,
    dpi_up      = 5,
    dpi_down    = 6,
    push_key    = 7,
    End         = 8,
    Nothing     = 9,
}
pub fn print_str_to_func_id_help(){
    println!("
        A function id must be one of these
        --left-click
        --right-click
        --middle-click
        --scroll-up
        --scroll-down
        --dpi-up
        --dpi-down
        --push-key
        --end
        --nothing")
}
pub fn str_to_function_id(str : &str) -> u8{
    
        let mut ret: u8 = 255;
        match str {
            "--left-click" => ret = 0,
            "--right-click" => ret = 1,
            "--middle-click" => ret = 2,
            "--scroll-up" => ret = 3,
            "--scroll-down" => ret = 4,
            "--dpi-up" => ret = 5,
            "--dpi-down" => ret = 6,
            "--push-key" => ret = 7,
            "--end" => ret = 8,
            "--nothing" => ret = 9,
            _=>{
                print_str_to_func_id_help();
            }
        }
        ret
    }
fn print_str_to_button_id_help(){
    println!("
        A function id must be one of these
        --button-left
        --button-right
        --button-middle
        --scroll-up
        --scroll-down
        --button-front
        --button-back
        ");

}
pub fn str_to_button_id(str : &str) -> u8{
    let mut ret : u8 = 255;
    match str {
        "--button-left" => ret = 0,
        "--button-right" => ret = 1,
        "--button-middle" => ret = 2,
        "--scroll-up" => ret = 3,
        "--scroll-down" => ret = 4,
        "--button-front" => ret = 5,
        "--button-back" => ret = 6,
        _ => {}
    } 
    ret

}

fn print_macro_help(){
    println!("
        Possible commands are :
            --button-to-single-function <button_id> <function_id> <key_code>
            --button-to-multiple-macros <button_id> <macro_nr>
            --change-function-in-macro  <macro_nr>  <index>       <function_id> <keycode ( u8 )>
            --change-delay-in-macro     <macro_nr>  <index>       <data ( u8 )>
            --change-time-in-macro      <macro_nr>  <index>       <data ( u8 )>
        ------------------------------------------------------------------------------------
        Possible button ids are :
            --button-left
            --button-right
            --button-middle
            --scroll-up
            --scroll-down
            --button-front
            --button-back
        ------------------------------------------------------------------------------------
        Possible function ids are : 
            --left-click
            --right-click
            --middle-click
            --scroll-up
            --scroll-down
            --dpi-up
            --dpi-down
            --push-key
            --end
            --nothing
        ====================================================================================

        ")
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
/// 6 backarg
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
pub fn handle_macro_api(arg : [String;8])->[u8;8]{
    let mut ret : [u8;8] = [0x04,0,0,0,0,0,0,0];
    match arg[1].as_str(){
        "--button-to-single-function"=>{
            // do change things
            ret[1] = 0;
            ret[2] = str_to_button_id(arg[2].as_str());
            ret[3] = str_to_function_id(arg[3].as_str());
            // Todo implement keycodes
            let mut keycode  = arg[4].parse::<u8>();
            match(keycode){
                Ok(keycode)=>{
                    ret[4] = keycode;
                },
                _=>{
                    ret[4] = 0;
                    return ret;
                }
            }
 
        },
        "--button-to-multiple-macros"=>{
            // do things
            ret[1] = 1;
            ret[2] = str_to_button_id(arg[2].as_str());
            let mut macro_id  = arg[3].parse::<u8>();
            match(macro_id){
                Ok(id)=>{
                    ret[3] = id;
                },
                _=>{
                    println!("The macro id needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }
        },
        "--change-function-in-macro"=>{
            //change function in macro - args: macro nr, index (8 bit),  Function id, keycode
            ret[1] = 2;
            let mut macro_id  = arg[2].parse::<u8>();
            match(macro_id){
                Ok(id)=>{
                    ret[2] = id;
                },
                _=>{
                    println!("The macro id needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }
            let mut index  = arg[3].parse::<u8>();
            match(index){
                Ok(index)=>{
                    ret[3] = index;
                },
                _=>{
                    println!("The index needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }
            ret[4] = str_to_function_id(arg[4].as_str());
            // Todo implement keycodes
            let mut keycode  = arg[5].parse::<u8>();
            match(keycode){
                Ok(keycode)=>{
                    ret[5] = keycode;
                },
                _=>{
                    println!("The index needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }

            
        },
        "--change-delay-in-macro"=>{
            // change delay in macro - args: macro nr, index (8 bit), data
            ret[1] = 3;
            let mut macro_id  = arg[2].parse::<u8>();
            match(macro_id){
                Ok(id)=>{
                    ret[2] = id;
                },
                _=>{
                    println!("The macro id needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }
            let mut index  = arg[3].parse::<u8>();
            match(index){
                Ok(index)=>{
                    ret[3] = index;
                },
                _=>{
                    println!("The index needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }
            let mut delay  = arg[4].parse::<u32>();
            match(delay){
                Ok(delay)=>{
                    ret[4] = (delay>>24)as u8 ;
                    ret[5] = (delay>>16) as u8;
                    ret[6] = (delay>>8) as u8;
                    ret[7] = (delay) as u8;
                },
                _=>{
                    println!("The delay needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }
            
        },
        "--change-time-in-macro"=>{
            // change time in macro - args: macro nr, index (8 bit), data
            ret[1] = 4;
            let mut macro_id  = arg[2].parse::<u8>();
            match(macro_id){
                Ok(id)=>{
                    ret[2] = id;
                },
                _=>{
                    println!("The macro id needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }
            let mut index  = arg[3].parse::<u8>();
            match(index){
                Ok(index)=>{
                    ret[3] = index;
                },
                _=>{
                    println!("The index needs to be 8 bits");
                    ret[0] = 255;
                    return ret;
                }
            }
            let mut delay  = arg[4].parse::<u8>();
            match(delay){
                Ok(delay)=>{
                    ret[4] = delay;
                },
                _=>{
                    ret[4] = 0;
                    return ret;
                }
            }
        }
        _=>{
            print_macro_help();
        }

    }
    ret


}
pub fn handle_dpi_api(arg : [String;8])->[u8;8]{
    let mut Dpi  = arg[1].parse::<u16>();
    match(Dpi){
        Ok(dpi)=>{
            [2,(dpi>>8) as u8,dpi as u8,0,0,0,0,0]
        },
        _=>{
            [255,0,0,0,0,0,0,0]
        }
    }
}
pub fn handle_rgb_api(arg : [String;8])->[u8;8]{
    println!("Not implemented yet!");
    [255,0,0,0,0,0,0,0]
}



pub struct Device{
    pub hid_device : DEVICE,
    pub api : HidApi
}

impl Device{
    pub fn new(target : DEVICE)-> Device{
        let mut api :HidApi = HidApi::new().unwrap();

        let mut device : DEVICE = (0,0);
        println!("Searching for valid devices");
        for itter in api.device_list() {
            if(target == (itter.vendor_id() as u16,itter.product_id() as u16)){
                println!(
                    "VID {:04x}: PID {:04x}",
                    itter.vendor_id(),
                    itter.product_id()
                );
                println!("manufacturer {:?}", itter.manufacturer_string());
                println!("product {:?}", itter.product_string());
                println!("serial {:?}", itter.serial_number());
                // we found what we were looking for, return
                break;
            }
        }

        Device{
            hid_device : target,
            api : api,
            
        }
    }
    pub fn write_data(&mut self, data : DATA)
    {
        println!("Writing data {:?}",data);
        match self.api.open(self.hid_device.0,self.hid_device.1)
        {
            Ok(device)=>{
                device.write(&data);      
            }
            _=>{}
        }         
    }
    pub fn repeated_write(&mut self,device:HidDevice, data : DATA)
    {
        println!("Writing data {:?}",data);
        
        device.write(&data);              
    }
}