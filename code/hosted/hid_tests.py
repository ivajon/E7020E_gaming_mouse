from mouse import *
import os
mouse = Mouse(vendor_id = 0xc410,product_id = 0x0000)

def handle_input(args):
    section = args[0].lower()
    if "help" in section:
        if len(args) == 1:
            print_ui()
        elif "dpi" in args[1].lower():
            print("""
                Sets the dpi of the mouse
                dpi <dpi>
                where dpi is an int
            """)
        elif "rgb" in args[1].lower():
            print("""
                Sets the rgb color of the mouse
                rgb <r> <g> <b>
                where r,g,b are ints
            """)
        elif "macro" in args[1].lower():
            print("""
                You caught me, this interface is not done yet
                come back later
            """)
        else:
            print_ui()

    
    elif  "dpi" in section:
            if len(args) < 2:
                return
            print(f"Setting the dpi to : {args[1]}")
            mouse.set_dpi(int(args[1]))
            
    elif "rgb" in section:
        print("Setting the rgb color")
        pass
    elif "macro" in section:
        print("Setting the macro")
        pass
    elif "exit" in section:
        os._exit(0)


def print_ui():
    print("""
    Replace <*> with the value you want to insert

    Commands: 
    dpi <dpi>           -> Sets the dpi of the mouse
    rgb <r> <g> <b>     -> sets the rgb color
    macro <macro>       -> Modifies the list of macros
    help                -> prints this message
    exit                -> exits the program
    """)
while 1:
    user_inp = input("Enter a command: ").split()
    os.system('cls' if os.name == 'nt' else 'clear')
    if len(user_inp) == 0:
        continue
    handle_input(user_inp)
    