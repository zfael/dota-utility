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
use std::time::{Duration, Instant};

lazy_static::lazy_static! {
    static ref PHASE_BOOTS_TRIGGER: Arc<Mutex<Option<(Instant, f32, f32)>>> = Arc::new(Mutex::new(None));
}

pub struct DefaultStrategy;

impl HeroStrategy for DefaultStrategy {
    fn execute(&self, event: &GsiWebhookEvent, item_positions: HashMap<String, String>, ability_positions: HashMap<String, String>) {
        on_low_health(event, item_positions.clone());
        on_phase_boots(event, item_positions.clone());
    }
}

fn on_phase_boots(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {
    println!("debug item {:?}", item_positions);
    if event.hero.is_alive() == false {
        return;
    }

    let phase_boots_position = item_positions.get("item_phase_boots");
    if phase_boots_position.is_none() {
        return;
    }

    let slot_position = SLOT_KEY_MAPPING.get(phase_boots_position.unwrap().as_str());
    if slot_position.is_none() {
        return;
    }

    let phase_boots = event.items.get_item_by_slot(phase_boots_position.unwrap());
    if phase_boots.is_none() {
        return;
    }

    let phase_boots = phase_boots.unwrap();
    if phase_boots.can_cast == Some(true) {
        let now = Instant::now();
        if let Ok(mut last_pressed) = PHASE_BOOTS_TRIGGER.try_lock() {
            let hero_x = event.hero.xpos;
            let hero_y = event.hero.ypos;
            match *last_pressed {
            Some((instant, last_x, last_y)) => {
                if now.duration_since(instant) >= Duration::from_secs(1) {
                // Only trigger if hero has moved significantly (e.g., > 10 units)
                if ((hero_x as f32 - last_x).abs() > 10.0) || ((hero_y as f32 - last_y).abs() > 10.0) {
                    let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
                    let mut enigo = Enigo::new(&Settings::default()).unwrap();
                    enigo.key(Key::Unicode(slot_position_unwrap), Click);
                    *last_pressed = Some((now, hero_x as f32, hero_y as f32));
                }
                }
            }
            None => {
                *last_pressed = Some((now, hero_x as f32, hero_y as f32));
            }
            }
        }
        
    }
}

fn on_low_health(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {
    // println!("debug item {:?}", item_positions);

    if event.hero.is_alive() == false {
        return;
    }

    let health_percent = event.hero.health_percent;

    let mut enigo = Enigo::new(&Settings::default()).unwrap();


    let early = if event.map.clock_time < 10 { true } else { false };
    let magic_wand_threshold = 20;
    let faerie_fire_threshold = 15;
    let famango_threshold = if early { 12 } else { 25 };
    let cheese_threshold = 10;

    if health_percent < famango_threshold {
        let famango1 = item_positions.get("item_greater_famango");
        
        if !famango1.is_none() {
            let slot_position = SLOT_KEY_MAPPING.get(famango1.unwrap().as_str());
            if !slot_position.is_none() {
                let mw = event.items.get_item_by_slot(famango1.unwrap());
                if !mw.is_none() {
                    let mw = mw.unwrap();
                    if mw.can_cast == Some(true) {
                        let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
                        enigo.key(Key::Unicode(slot_position_unwrap), Click);
                    }
                }
            }
        }

        let famango2 = item_positions.get("item_great_famango");
        
        if !famango2.is_none() {
            let slot_position = SLOT_KEY_MAPPING.get(famango2.unwrap().as_str());
            if !slot_position.is_none() {
                let mw = event.items.get_item_by_slot(famango2.unwrap());
                if !mw.is_none() {
                    let mw = mw.unwrap();
                    if mw.can_cast == Some(true) {
                        let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
                        enigo.key(Key::Unicode(slot_position_unwrap), Click);
                    }
                }
            }
        }
    }
    


    if health_percent < magic_wand_threshold {
        let magic_wand = item_positions.get("item_magic_wand");
        if !magic_wand.is_none() {
            let slot_position = SLOT_KEY_MAPPING.get(magic_wand.unwrap().as_str());
            if !slot_position.is_none() {
                let mw = event.items.get_item_by_slot(magic_wand.unwrap());
                if !mw.is_none() {
                    let mw = mw.unwrap();
                    if mw.can_cast == Some(true) {
                        let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
                        enigo.key(Key::Unicode(slot_position_unwrap), Click);
                    }
                }
            }
        }
    }

    if health_percent < faerie_fire_threshold {
        let faerie_position = item_positions.get("item_faerie_fire");
        if !faerie_position.is_none() {

            let slot_position = SLOT_KEY_MAPPING.get(faerie_position.unwrap().as_str());
            if !slot_position.is_none() {
                let faerie = event.items.get_item_by_slot(faerie_position.unwrap());
                if !faerie.is_none() {
                    
                    let faerie = faerie.unwrap();
                    if faerie.can_cast == Some(true) {
                        let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
                        enigo.key(Key::Unicode(slot_position_unwrap), Click);
                    }
                }
            }
        }
    }

    if health_percent < cheese_threshold {
        let magic_wand = item_positions.get("item_cheese");
        if !magic_wand.is_none() {
            let slot_position = SLOT_KEY_MAPPING.get(magic_wand.unwrap().as_str());
            if !slot_position.is_none() {
                let mw = event.items.get_item_by_slot(magic_wand.unwrap());
                if !mw.is_none() {
                    let mw = mw.unwrap();
                    if mw.can_cast == Some(true) {
                        let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
                        enigo.key(Key::Unicode(slot_position_unwrap), Click);
                    }
                }
            }
        }
    }
}