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
        if keys.contains(&Keycode::Q) {
            println!("trigger q");
            enigo.button(Button::Right, Click);
            thread::sleep(time::Duration::from_millis(100));
            enigo.key(Key::Unicode('l'), Click);
        }
        
        if keys.contains(&Keycode::W) {
            println!("trigger W");
            enigo.button(Button::Right, Click);
            thread::sleep(time::Duration::from_millis(100));
            enigo.key(Key::Unicode('k'), Click);
        }
        
        if keys.contains(&Keycode::E) {
            println!("trigger E");
            enigo.button(Button::Right, Click);
            thread::sleep(time::Duration::from_millis(100));
            enigo.key(Key::Unicode('j'), Click);
        }

        thread::sleep(time::Duration::from_millis(20));
    }
}