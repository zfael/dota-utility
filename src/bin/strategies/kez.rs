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

pub struct KezStrategy;

lazy_static::lazy_static! {
    static ref ACTION_TRIGGERED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

impl HeroStrategy for KezStrategy {
    fn execute(&self, event: &GsiWebhookEvent, item_positions: HashMap<String, String>, ability_positions: HashMap<String, String>) {
        println!("Executing strategy for kez");
        // println!("debug item {:?}", event.abilities);
        // on_ult(event, item_positions.clone());
        on_low_health(event, item_positions.clone(), ability_positions.clone());
        use_bkb(event, item_positions.clone());
    }
}

fn default(event: &GsiWebhookEvent, item_positions: HashMap<String, String>, ability_positions: HashMap<String, String>) {
}


fn use_bkb(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {
    if event.hero.is_alive() == false {
        return;
    }
    
    // println!("debug bkb1 {:?}", item_positions);
    if !event.hero.should_use_bkb() {
        return;
    }

    let bkb_position = item_positions.get("item_black_king_bar");
    if bkb_position.is_none() {
        println!("BKB not found, skipping BKB usage");
        return;
    }

    let slot_position = SLOT_KEY_MAPPING.get(bkb_position.unwrap().as_str());
    if slot_position.is_none() {
        println!("BKB slot position not found, skipping BKB usage");
        return;
    }

    let bkb = event.items.get_item_by_slot(bkb_position.unwrap());
    if bkb.is_none() {
        return;
    }
    
    let bkb = bkb.unwrap();
    if bkb.can_cast == Some(false) {
        return;
    }

    let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Unicode(slot_position_unwrap), Click);
}

fn on_low_health(event: &GsiWebhookEvent, item_positions: HashMap<String, String>, ability_positions: HashMap<String, String>) {
    if event.hero.is_alive() == false {
        return;
    }

    let health_percent = event.hero.health_percent;

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // let ult = ability_positions.get("kez_raptor_dance");
    
    println!("health_percent: {}", health_percent);
    if health_percent <= 20 {
        let ult = event.abilities.find_ability_by_name("kez_raptor_dance");
        if !ult.is_none() {
            let ult = ult.unwrap();
            if (ult.can_cast) {
                enigo.key(Key::Unicode('r'), Click);
            }
        }
    }
    
    if health_percent < 40 {
        let satanic_position = item_positions.get("item_satanic");
        if satanic_position.is_none() {
            println!("Satanic not found, skipping armlet toggle");
            return;
        }
        
        let slot_position = SLOT_KEY_MAPPING.get(satanic_position.unwrap().as_str());
        if slot_position.is_none() {
            println!("slot position not found, skipping armlet toggle");
            return;
        }

        let satanic = event.items.get_item_by_slot(satanic_position.unwrap());
        if satanic.is_none() {
            return;
        }
        println!("Health percent3: {}", health_percent);
        
        let satanic = satanic.unwrap();
        if satanic.can_cast == Some(false) {
            return;
        }
        println!("Health percent4: {}", health_percent);

        let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
        enigo.key(Key::Unicode(slot_position_unwrap), Click);
    }

}

fn on_ult(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {
    // println!("debug item {:?}", item_positions);
    let ult = &event.abilities.ability4;
    if ult.cooldown <= 0 {
        println!("Ult on cooldown, skipping ult check");
        return;
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    if event.items.neutral0.name != "empty" {
        if event.items.neutral0.can_cast == Some(true) {
            enigo.key(Key::Unicode('0'), Click);
        }
    }

    let blade_mail_position = item_positions.get("item_blade_mail");
    if !blade_mail_position.is_none() {
        let slot_position = SLOT_KEY_MAPPING.get(blade_mail_position.unwrap().as_str());
        if !slot_position.is_none() {
            let blade_main = event.items.get_item_by_slot(blade_mail_position.unwrap());
            if !blade_main.is_none() {
                let blade_main = blade_main.unwrap();
                if blade_main.can_cast == Some(true) {
                    let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
                    enigo.key(Key::Unicode(slot_position_unwrap), Click);
                }
            }
        }
    }
}