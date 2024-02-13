#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use dotenv::dotenv;
use serde_json::error;
use sqlx::{mysql::*, Pool};
use sqlx::{FromRow, Row};
use std::env;

#[derive(Debug, sqlx::FromRow, Default)]
struct Guild {
    snowflake: String,
    register_snowflake: Option<String>,
    name: String,
    channel_admin: Option<String>,
    channel_event: Option<String>,
    channel_suggestion: Option<String>,
    channel_snippet: Option<String>,
    channel_rules: Option<String>,
    role_blinded: Option<String>,
    locale: String,
    disabled: bool,
    guild_date_creation: chrono::DateTime<chrono::Utc>,
    xp15: u32,
    xp50: u32,
    level_up_reward_base: u32,
    role_cosmetic_price: u32,
    role_cosmetic_power: u8,
    role_level_power: u8,
    role_level_max: u32,
    role_level_enable: bool,
    role_level_color: String,
    jackpot: u32,
    welcome: bool,
    xp_increase_normal: u32,
    xp_increase_slash: u32,
    xp_increase_purchase: u32,
}

// https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url: String = match env::var("DATABASE_TOKEN") {
        Ok(res) => String::from(res),
        Err(error) => panic!("Key not found. {:?}", error),
    };

    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let query = sqlx::query_as::<_, Guild>("SELECT * FROM guild LEFT JOIN guild_settings ON guild_settings.guild_snowflake = guild.snowflake;");
    let guilds: Vec<Guild> = query.fetch_all(&pool).await?;
    for guild in guilds.iter() {
        println!("{:?}", guild);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");

    Ok(())
}
