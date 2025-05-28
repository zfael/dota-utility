use crate::models::{GsiWebhookEvent, BLINK_ITEM_NAME, SLOT_KEY_MAPPING};
use crate::strategies::HeroStrategy;
use enigo::{
    Button, Coordinate, 
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub struct MorphStrategy;

lazy_static::lazy_static! {
    static ref ACTION_TRIGGERED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

impl HeroStrategy for MorphStrategy {
    fn execute(&self, event: &GsiWebhookEvent, item_positions: HashMap<String, String>, ability_positions: HashMap<String, String>) {
        // println!("Executing strategy for morphling");
        // println!("debug item {:?}", event.abilities);
        // on_ult(event, item_positions.clone());
        // on_low_health(event, item_positions.clone(), ability_positions.clone());
        // use_bkb(event, item_positions.clone());
        control_shift(event, item_positions.clone());
    }
}

fn control_shift(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {

    if event.hero.is_alive() == false {
        return;
    }

    if event.hero.health_percent >70 {
        if let Ok(mut lock) = ACTION_TRIGGERED.try_lock() {
            if *lock {
                println!("Releasing lock as health is above 90%");
                let mut enigo = Enigo::new(&Settings::default()).unwrap();
                enigo.key(Key::Unicode('d'), Click);
                *lock = false;
            }
        }
        return;
    }


    // If health is 90% or below, check if the action is already triggered
    if let Ok(mut lock) = ACTION_TRIGGERED.try_lock() {
        if *lock {
            // Action is already triggered, do nothing
            return;
        }

        // Lock the action and simulate pressing the "E" key
        println!("Health is below 90%, triggering action and locking...");
        *lock = true;

        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        enigo.key(Key::Unicode('f'), Click);
        
    } else {
        // Lock is already held, skip this iteration
        println!("Lock is already held, skipping...");
    }


}