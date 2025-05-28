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
        if keys.contains(&Keycode::Home) {

            trigger_sequence(&mut enigo);

            // Prevent repeated triggering by waiting for the key to be released
            // while device_state.get_keys().contains(&Keycode::Space) {
            //     thread::sleep(time::Duration::from_millis(10));
            // }
        }

        if keys.contains(&Keycode::Q) {
            println!("trigger q");
            enigo.button(Button::Right, Click);
            thread::sleep(time::Duration::from_millis(250));
            enigo.key(Key::Unicode('l'), Click);
        }

        thread::sleep(time::Duration::from_millis(10));
    }
}

fn trigger_sequence(enigo: &mut Enigo) {
    println!("Triggering sequence...");

    enigo.key(Key::Unicode('l'), Click); // Press 'c'
    enigo.key(Key::Unicode('l'), Click); // Press 'c'
    enigo.key(Key::Unicode('l'), Click); // Press 'c'
    thread::sleep(time::Duration::from_millis(200)); // Wait 50ms
    enigo.key(Key::Unicode('d'), Click); // Press 'c'
    enigo.key(Key::Unicode('d'), Click); // Press 'c'
    enigo.key(Key::Unicode('d'), Click); // Press 'c'
    thread::sleep(time::Duration::from_millis(200)); // Wait 50ms
    enigo.key(Key::Unicode('w'), Click); // Press 'c'
    enigo.key(Key::Unicode('w'), Click); // Press 'c'
    enigo.key(Key::Unicode('w'), Click); // Press 'c'
    enigo.key(Key::Unicode('w'), Click); // Press 'c'
    enigo.key(Key::Unicode('w'), Click); // Press 'c'
    // thread::sleep(time::Duration::from_millis(200)); // Wait 50ms

    // thread::sleep(time::Duration::from_millis(30)); // Wait 50ms
    // enigo.key(Key::Unicode('w'), Click); // Press 'c'
    // enigo.key(Key::Unicode('w'), Click); // Press 'c'
    // thread::sleep(time::Duration::from_millis(100)); // Wait 50ms
    
    // enigo.key(Key::Unicode('q'), Click); // Press 'z'
    // thread::sleep(time::Duration::from_millis(30)); // Wait 50ms
    // enigo.key(Key::Unicode('q'), Click); // Press 'z'
    // enigo.key(Key::Unicode('q'), Click); // Press 'z'
    // thread::sleep(time::Duration::from_millis(1400)); // Wait 50ms

    // // if there is aganing
    // enigo.key(Key::Unicode('d'), Click); // Press 'c'
    println!("Sequence complete.");
}
