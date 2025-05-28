use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use lazy_static::lazy_static;

pub const BLINK_ITEM_NAME: &str = "item_blink";

lazy_static! {
    pub static ref SLOT_KEY_MAPPING: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("slot0", "z");
        m.insert("slot1", "x");
        m.insert("slot2", "c");
        m.insert("slot3", "v");
        m.insert("slot4", "b");
        m.insert("slot5", "n");
        m.insert("neutral", "0");
        return m
    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub can_cast: Option<bool>,
    pub cooldown: Option<u32>,
    pub item_level: Option<u32>,
    pub passive: Option<bool>,
    pub purchaser: Option<u32>,
    pub charges: Option<u32>,
    pub item_charges: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Items {
    pub neutral0: Item,
    pub slot0: Item,
    pub slot1: Item,
    pub slot2: Item,
    pub slot3: Item,
    pub slot4: Item,
    pub slot5: Item,
    pub slot6: Item,
    pub slot7: Item,
    pub slot8: Item,
    pub stash0: Item,
    pub stash1: Item,
    pub stash2: Item,
    pub stash3: Item,
    pub stash4: Item,
    pub stash5: Item,
    pub teleport0: Item,
}

impl Items {
    pub fn find_item_position(&self, item_name: &str) -> Option<String> {
        let items = vec![
            ("neutral0", &self.neutral0),
            ("slot0", &self.slot0),
            ("slot1", &self.slot1),
            ("slot2", &self.slot2),
            ("slot3", &self.slot3),
            ("slot4", &self.slot4),
            ("slot5", &self.slot5),
            ("slot6", &self.slot6),
            ("slot7", &self.slot7),
            ("slot8", &self.slot8),
            ("stash0", &self.stash0),
            ("stash1", &self.stash1),
            ("stash2", &self.stash2),
            ("stash3", &self.stash3),
            ("stash4", &self.stash4),
            ("stash5", &self.stash5),
            ("teleport0", &self.teleport0),
        ];

        for (position, item) in items {
            if item.name == item_name {
                return Some(position.to_string());
            }
        }
        None
    }
}

impl Items {
    pub fn find_item_positions_v2(&self) -> HashMap<String, String> {
        let items = vec![
            ("neutral0", &self.neutral0),
            ("slot0", &self.slot0),
            ("slot1", &self.slot1),
            ("slot2", &self.slot2),
            ("slot3", &self.slot3),
            ("slot4", &self.slot4),
            ("slot5", &self.slot5),
            ("slot6", &self.slot6),
            ("slot7", &self.slot7),
            ("slot8", &self.slot8),
            ("stash0", &self.stash0),
            ("stash1", &self.stash1),
            ("stash2", &self.stash2),
            ("stash3", &self.stash3),
            ("stash4", &self.stash4),
            ("stash5", &self.stash5),
            ("teleport0", &self.teleport0),
        ];

        let mut item_positions = HashMap::new();

        for (position, item) in items {
            item_positions.insert(item.name.clone(), position.to_string());
        }

        item_positions
    }

    pub fn get_item_by_slot(&self, slot: &str) -> Option<&Item> {
        match slot {
            "neutral0" => Some(&self.neutral0),
            "slot0" => Some(&self.slot0),
            "slot1" => Some(&self.slot1),
            "slot2" => Some(&self.slot2),
            "slot3" => Some(&self.slot3),
            "slot4" => Some(&self.slot4),
            "slot5" => Some(&self.slot5),
            "slot6" => Some(&self.slot6),
            "slot7" => Some(&self.slot7),
            "slot8" => Some(&self.slot8),
            "stash0" => Some(&self.stash0),
            "stash1" => Some(&self.stash1),
            "stash2" => Some(&self.stash2),
            "stash3" => Some(&self.stash3),
            "stash4" => Some(&self.stash4),
            "stash5" => Some(&self.stash5),
            "teleport0" => Some(&self.teleport0),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ability {
    pub ability_active: bool,
    pub can_cast: bool,
    pub cooldown: u32,
    pub level: u32,
    pub name: String,
    pub passive: bool,
    pub ultimate: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Abilities {
    pub ability0: Ability,
    pub ability1: Ability,
    pub ability2: Ability,
    pub ability3: Ability,
    pub ability4: Ability,
    pub ability5: Ability,
}

impl Abilities {
    pub fn find_ability_positions(&self) -> HashMap<String, String> {
        let items = vec![
            ("ability0", &self.ability0),
            ("ability1", &self.ability1),
            ("ability2", &self.ability2),
            ("ability3", &self.ability3),
            ("ability4", &self.ability4),
            ("ability5", &self.ability5),
        ];

        let mut ability_positions = HashMap::new();

        for (position, item) in items {
            ability_positions.insert(item.name.clone(), position.to_string());
        }

        ability_positions
    }

    pub fn find_ability_by_name(&self, ability_name: &str) -> Option<&Ability> {
        let abilities = vec![
            ("ability0", &self.ability0),
            ("ability1", &self.ability1),
            ("ability2", &self.ability2),
            ("ability3", &self.ability3),
            ("ability4", &self.ability4),
            ("ability5", &self.ability5),
        ];

        for (position, ability) in abilities {
            if ability.name == ability_name {
                return Some(ability);
            }
        }
        None
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hero {
    pub aghanims_scepter: bool,
    pub aghanims_shard: bool,
    pub alive: bool,
    pub attributes_level: u32,
    #[serde(rename = "break")]
    pub is_break: bool,
    pub buyback_cooldown: u32,
    pub buyback_cost: u32,
    pub disarmed: bool,
    pub facet: u32,
    pub has_debuff: bool,
    pub health: u32,
    pub health_percent: u32,
    pub hexed: bool,
    pub id: u32,
    pub level: u32,
    pub magicimmune: bool,
    pub mana: u32,
    pub mana_percent: u32,
    pub max_health: u32,
    pub max_mana: u32,
    pub muted: bool,
    pub name: String,
    pub respawn_seconds: u32,
    pub silenced: bool,
    pub smoked: bool,
    pub stunned: bool,
    pub talent_1: bool,
    pub talent_2: bool,
    pub talent_3: bool,
    pub talent_4: bool,
    pub talent_5: bool,
    pub talent_6: bool,
    pub talent_7: bool,
    pub talent_8: bool,
    pub xp: u32,
    pub xpos: i32,
    pub ypos: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub clock_time: i32
}

impl Hero {
    pub fn is_alive(&self) -> bool {
        return self.alive;
    }

    pub fn is_stunned(&self) -> bool {
        if self.stunned {
            return true;
        }

        return false;
    }

    pub fn is_in_danger(&self) -> bool {
        if self.muted || self.disarmed || self.hexed || self.silenced || self.stunned {
            return true;
        }

        return false;
    }

    pub fn has_debuff(&self) -> bool {
        if self.has_debuff {
            return true;
        }

        return false;
    }

    pub fn should_use_bkb(&self) -> bool {
        if self.health_percent <= 50 && self.is_in_danger() {
            return true;
        }

        if self.health_percent <= 25 {
            return true;
        }

        return false;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GsiWebhookEvent {
    pub hero: Hero,
    pub abilities: Abilities,
    pub items: Items,
    pub map: Map,
}