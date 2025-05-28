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

pub struct HuskarStrategy;

lazy_static::lazy_static! {
    static ref ACTION_TRIGGERED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref BERSEKERBLOOD_KEY_PRESSED_AT: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
}

impl HeroStrategy for HuskarStrategy {
    fn execute(
        &self,
        event: &GsiWebhookEvent,
        item_positions: HashMap<String, String>,
        ability_positions: HashMap<String, String>,
    ) {
        println!("huskar strategy");
        armlet_toggle(event, item_positions.clone());
        //on_ult(event, item_positions.clone());
        on_low_health(event, item_positions.clone());
        // use_bkb(event, item_positions.clone());
    }
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
        return;
    }

    let slot_position = SLOT_KEY_MAPPING.get(bkb_position.unwrap().as_str());
    if slot_position.is_none() {
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

fn on_low_health(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {
    //println!("debug item {:?}", item_positions);

    if event.hero.is_alive() == false {
        return;
    }

    let health_percent = event.hero.health_percent;

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Check if hero has debuff
    if event.hero.has_debuff() {
        let now = Instant::now();
        if let Ok(mut last_pressed) = BERSEKERBLOOD_KEY_PRESSED_AT.try_lock() {
            match *last_pressed {
                Some(instant) => {
                    if now.duration_since(instant) >= Duration::from_secs(2) {
                        let bersekerblood = &event.abilities.ability2;
                        if bersekerblood.cooldown <= 0 {
                            enigo.key(Key::Unicode('e'), Click);
                            *last_pressed = Some(now);
                        }
                    }
                }
                None => {
                    *last_pressed = Some(now);
                }
            }
        }
    }

    // if health_percent < 30 {
    //     let satanic_position = item_positions.get("item_satanic");
    //     if !satanic_position.is_none() {
    //         let slot_position = SLOT_KEY_MAPPING.get(satanic_position.unwrap().as_str());
    //         if !slot_position.is_none() {
    //             let satanic = event.items.get_item_by_slot(satanic_position.unwrap());
    //             if !satanic.is_none() {
    //                 let satanic = satanic.unwrap();
    //                 if satanic.can_cast == Some(true) {
    //                     let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();
    //                     enigo.key(Key::Unicode(slot_position_unwrap), Click);
    //                 }
    //             }
    //         }
    //     }
    // }

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

fn on_ult(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {
    // println!("debug item {:?}", event.items.neutral0.name);

     if event.hero.is_alive() == false {
        return;
    }

    let ult = &event.abilities.ability4;
    if ult.cooldown <= 0 {
        return;
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let allowed = vec![
        "item_rippers_lash",
        "item_essence_ring",
        "item_gale_guard",
        "item_crippling_crossbow",
        "item_pyrrhic_cloak",
        "item_minotaur_horn"
    ];


    if allowed.contains(&event.items.neutral0.name.as_str()) {
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

fn armlet_toggle(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {
    let armlet_position = item_positions.get("item_armlet");
    if armlet_position.is_none() {
        return;
    }

    let slot_position = SLOT_KEY_MAPPING.get(armlet_position.unwrap().as_str());
    if slot_position.is_none() {
        return;
    }

    let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();

    if !event.hero.is_alive() {
        return;
    }

    let health = event.hero.health;

    if health < 320 {
        if let Ok(mut lock) = ACTION_TRIGGERED.try_lock() {
            if *lock {
                // println!("acquired already 111");
                return; // Action is already triggered, do nothing
            }

            if event.hero.is_stunned() {
                // println!("stunned, not triggering action...\n");
                return; // Hero is stunned, do nothing
            }

            *lock = true; // Lock the action trigger state
            // println!("Action triggered, locking...\n");
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            enigo.key(Key::Unicode(slot_position_unwrap), Click);
            thread::sleep(time::Duration::from_millis(1));
            enigo.key(Key::Unicode(slot_position_unwrap), Click);
        }
        
    } else {
        if let Ok(mut lock) = ACTION_TRIGGERED.try_lock() {
            if *lock {
                // println!("already acquired, releasing lock...\n");
                *lock = false; // Reset the action trigger state
                return; // Action is already triggered, do nothing
            }
        }
    }
}


// fn armlet_toggle(event: &GsiWebhookEvent, item_positions: HashMap<String, String>) {
//     let armlet_position = item_positions.get("item_armlet");
//     if armlet_position.is_none() {
//         return;
//     }

//     let slot_position = SLOT_KEY_MAPPING.get(armlet_position.unwrap().as_str());
//     if slot_position.is_none() {
//         return;
//     }

//     let slot_position_unwrap = slot_position.unwrap().chars().next().unwrap();

//     if !event.hero.is_alive() {
//         return;
//     }

//     let health = event.hero.health;

//     if health < 320 {
//         let mut action_triggered = ACTION_TRIGGERED.lock().unwrap();
//         if !*action_triggered {
//             *action_triggered = true;
//             let mut enigo = Enigo::new(&Settings::default()).unwrap();
//             enigo.key(Key::Unicode(slot_position_unwrap), Click);
//             enigo.key(Key::Unicode(slot_position_unwrap), Click);
//             thread::sleep(time::Duration::from_millis(300)); // Wait 50ms
//             *action_triggered = false;
//         }
//     }
// }
