use warp::Filter;
use serde_json::Value;
use enigo::{
    Button, Coordinate, 
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use std::sync::{Arc, Mutex};
use std::convert::Infallible;
use std::{thread, time};

lazy_static::lazy_static! {
    static ref ACTION_TRIGGERED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[tokio::main]
async fn main() {

    // Define the endpoint to receive any JSON object
    let gsi_route = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|json_body: Value| {
            let json_string = json_body.to_string();
            println!("Received JSON: {}", json_string);
            
            // Check if hero.health is less than 300 and press "x" key twice
            armlet_toggle(json_body);

            warp::reply::json(&"Event received successfully")
        });

    // Start the server
    warp::serve(gsi_route)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

fn armlet_toggle(json_body: Value) {
    if let Some(alive) = json_body.get("hero")
                                  .and_then(|hero| hero.get("alive"))
                                  .and_then(|a| a.as_bool()) {
        if !alive {
            return;
        }
    }
    
    if let Some(health) = json_body.get("hero")
                                   .and_then(|hero| hero.get("health"))
                                   .and_then(|h| h.as_i64()) {
        if health < 320 {
            let mut action_triggered = ACTION_TRIGGERED.lock().unwrap();
            if !*action_triggered {
                *action_triggered = true;
                let mut enigo = Enigo::new(&Settings::default()).unwrap();
                enigo.key(Key::Unicode('x'), Click); 
                enigo.key(Key::Unicode('x'), Click); 
                thread::sleep(time::Duration::from_millis(300)); // Wait 50ms
                *action_triggered = false;
            }
        }
    }
}