# Awesome gaming mouse

## Members

- Erik Serrander
- eriser-8@student.ltu.se
  
- Ivar Jönsson
- ivajns-9@student.ltu.se


## High level specification

### Basic
- Left and right click
- relative position

### Higher
- scroll wheel
- macro system
- Side buttons

### Stretch
- Computer communication
- LEDs, i.e. debug/ui
- Rumble motor
- Perfect spray


### Implementation
- Unit testing
- Cuntinuous integration



## Specification
---
### Mouse buttons
The buttons used are D2F-01F since these are well regarded by users.
We have decided to have 4 main buttons on the mouse, 2 on the front and 2 on the side.


![front_buttons](images/front_buttons.png)
![side_buttons](images/side_buttons.png)

To reduce the bouncing of the buttons a 10u capacitor is places to ground from the signal trace. This will allow the signal to be pulled low for high frequencies. 

To step down the voltage a nfet transistor is used to drive a 3v3 load from the 5v source. This allows the micro controller to read the signal directly.

To drive the buttons a 5v voltage is used with a series resistor of 4.7 k to limit the current to close to 1mA. The mosfet used to limit the output acts as a decoupler thus not draining anny current ( in theory ).

### Mouse wheel
![mouse_wheel](images/mouse_wheel.png)

The mouse wheel is also debounced using the same capacitor values as the mouse button. This should not be as needed but better safe than sorry.

The mouse wheel also uses the same nfet to step down the voltage.

### RGB
Since RGB makes electronics better we include an array of multiplexed rgb leds. This will allow us to display everything from cool animations to debug info. 





## Grading

- Erik Serrander
  - Expected contributions towards grade 4
    - side buttons
    - diods
  - Expected contributions towards grade 5
    - Computer communication
    - Rumble motor
  
- Ivar Jönsson
  - Expected contributions towards grade 4
    - Mouse wheel
    - diods
  - Expected contributions towards grade 5
    - Computer communication
    - Perfect spray
