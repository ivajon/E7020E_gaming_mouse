from mouse import *
import os
mouse = Mouse(vendor_id = 0xc410,product_id = 0x0000)

def handle_input(args):
    section = args[0].lower()
    if  "dpi" in section:
            if len(args) < 2:
                return
            print(f"Setting the dpi to : {args[1]}")
            mouse.set_dpi(int(args[1]))
    if "rgb" in section:
        print("Setting the rgb color")
        pass
    if "macro" in section:
        print("Setting the macro")
        pass
    if "help" in section:
        print_ui()
    if "exit" in section:
        os._exit(0)


def print_ui():
    print("""
    Commands: 
    dpi <dpi> -> Sets the dpi of the mouse
    rgb <r> <g> <b> -> sets the rgb color
    macro <macro> -> Modifies the list of macros
    help -> prints this message
    exit -> exits the program
    """)
while 1:
    user_inp = input("Enter a command: ").split()
    os.system('cls' if os.name == 'nt' else 'clear')
    if len(user_inp) == 0:
        continue
    handle_input(user_inp)
    