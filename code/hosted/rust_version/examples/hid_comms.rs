use std::string;
use hid_host::device::*;
use std::io;
use std::env;
use std::fs;
use std::path::Path;
use std::fmt;


fn handle_text(arg : [String;8],device :& Device)->[u8;8]{
    match arg[0].as_str() {
        "rgb" =>{
            handle_rgb_api(arg)
        },
        "mouse"=>{
            handle_dpi_api(arg)
        },
        "macro"=>{
            handle_macro_api(arg,device)
        },
        _=>{
            // Send garbage
            [255,0,0,0,0,0,0,0]
        }
    }
}
fn str_splitter(str : String)->[String;8]{
    let mut splt = str.split_whitespace();
    let mut args:[String;8] = [String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),String::new()];
    let mut itter = 0;
    for char in splt{
        if itter == 8{
            break;
        }
        args[itter] = String::from(char);
        itter+=1;
    }
    args
}
fn read_std_in()->[String;8]{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    str_splitter(buffer)
}
fn config_loader(device :&mut Device,file_name : &mut String){
            let mut path = std::env::current_dir().unwrap();//Path::new("");
            path = std::env::current_dir().unwrap().join(format!("./{}.cfg",file_name));
            let contents = fs::read_to_string(path)
                .expect("Something went wrong reading the file");
            let lines = contents.split('\n');
            match device.api.open(device.hid_device.0,device.hid_device.1){
                Ok(hid)=>{
                    for line in lines{
                        if !line.contains("//"){
                            let args = str_splitter(String::from(line));
                            let mut data_write = handle_text(args,device);
                            //println!("Writing data {:?}",data_write);
                            hid.write(&data_write);

                        }
                        
                    }
                },
                _=>{}
            
            }
}
fn main() {
    let mut device : Device = Device::new((0xc410,0));

    loop{
        let mut args = read_std_in();
        if args[0].as_str() == "load-file"{
            config_loader(&mut device,&mut args[1]);
        }
        else{
            let mut data_write = handle_text(args,&device);
            device.write_data(data_write);
        }
    }

}