use std::collections::HashMap;

use crate::models::GsiWebhookEvent;

pub trait HeroStrategy {
    fn execute(&self, event: &GsiWebhookEvent, items_positions: HashMap<String, String>, ability_positions: HashMap<String, String>);
}

pub mod huskar;
pub mod kez;
pub mod default;
pub mod morph;