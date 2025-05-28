use enigo::{
    Button, Coordinate, 
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let device_state = DeviceState::new();

    println!("Listening for spacebar press globally...\nPress Ctrl+C to exit.");

    loop {       
        let keys = device_state.get_keys();

        // Check if the spacebar is pressed
        if keys.contains(&Keycode::W) {
            println!("trigger w");
            enigo.button(Button::Right, Click);
            thread::sleep(time::Duration::from_millis(250));
            enigo.key(Key::Unicode('l'), Click);
        }
        
        thread::sleep(time::Duration::from_millis(20));
    }
}