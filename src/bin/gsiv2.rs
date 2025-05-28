mod models;
mod strategies;

use strategies::{default::DefaultStrategy, huskar::HuskarStrategy, kez::KezStrategy, morph::MorphStrategy, HeroStrategy};
use models::{GsiWebhookEvent, BLINK_ITEM_NAME, SLOT_KEY_MAPPING};

use warp::Filter;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use enigo::{
    Button, Coordinate, 
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use std::time::{SystemTime};

#[tokio::main]
async fn main() {

    let route = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        // .map(|json_body: Value| {
        //     let json_string = json_body.to_string();
        //     println!("Received JSON: {}", json_string);
            
        //     warp::reply::json(&"Event received successfully")
        // });
        .map(|event: GsiWebhookEvent| {
            println!("Event received: {:?}", SystemTime::now());
            
            let item_positions = event.items.find_item_positions_v2();
            let ability_positions = event.abilities.find_ability_positions();

            let hero_name = &event.hero.name;
            let strategy: Box<dyn HeroStrategy> = match hero_name.as_str() {
                "npc_dota_hero_huskar" => Box::new(HuskarStrategy),
                "npc_dota_hero_kez" => Box::new(KezStrategy),
                "npc_dota_hero_morphling" => Box::new(MorphStrategy),
                _ => Box::new(DefaultStrategy),
            };

            strategy.execute(&event, item_positions, ability_positions);

            warp::reply::json(&"Event received successfully")
        });

    // Start the server
    warp::serve(route)
        .run(([127, 0, 0, 1], 3000))
        .await;
}