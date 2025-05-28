use enigo::{Direction::Click, Enigo, Key, Keyboard, Mouse, Settings};
use std::{thread, time::Duration};
use device_query::{DeviceQuery, DeviceState};


fn click_wait(enigo: &mut Enigo) {
    enigo.button(enigo::Button::Left, enigo::Direction::Click);
    thread::sleep(Duration::from_secs(1));
}

fn log_mouse_position() {
    let device_state = DeviceState::new();
    let mouse_position = device_state.get_mouse();
    println!("Mouse position: x = {}, y = {}", mouse_position.coords.0, mouse_position.coords.1);
}


fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    log_mouse_position();

    // Initial delay
    thread::sleep(Duration::from_secs(2));

    let mut option = true;
    // Infinite loop
    loop {
        // Move mouse to (121, 322) and click
        enigo.move_mouse(121, 322, enigo::Coordinate::Abs);
        click_wait(&mut enigo);

        if !option {
            option = true;
            enigo.move_mouse(1639, 583, enigo::Coordinate::Abs);
            enigo.button(enigo::Button::Left, enigo::Direction::Click);
            thread::sleep(Duration::from_secs(1));
        } else {
            option = false;
            enigo.move_mouse(1462, 771, enigo::Coordinate::Abs);
            thread::sleep(Duration::from_millis(100));
            enigo.button(enigo::Button::Left, enigo::Direction::Click);
            thread::sleep(Duration::from_secs(1));
        }

        // Move mouse to (1462, 771) and click
        enigo.move_mouse(1462, 771, enigo::Coordinate::Abs);
        thread::sleep(Duration::from_millis(100));
        enigo.button(enigo::Button::Left, enigo::Direction::Click);
        thread::sleep(Duration::from_secs(1));

        // Perform multiple clicks
        click_wait(&mut enigo);
        click_wait(&mut enigo);
        click_wait(&mut enigo);

        // Press and release "f" key
        enigo.key(Key::Unicode('f'), Click);
        thread::sleep(Duration::from_secs(4));

        // Press and release "d" key
        enigo.key(Key::Unicode('d'), Click);
        thread::sleep(Duration::from_secs(1));

        // Move mouse to (1468, 696) and click
        enigo.move_mouse(1468, 696, enigo::Coordinate::Abs);
        enigo.button(enigo::Button::Left, enigo::Direction::Click);
        thread::sleep(Duration::from_secs(1));

        enigo.move_mouse(1816, 540, enigo::Coordinate::Abs);
        enigo.button(enigo::Button::Left, enigo::Direction::Click);
        thread::sleep(Duration::from_secs(1));

        // Press and release "k" key
        enigo.key(Key::Unicode('k'), Click);
        thread::sleep(Duration::from_secs(1));
    }
}