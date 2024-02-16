#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

// Dependencies
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, to_value, Value};
use sqlx::{mysql::*, FromRow, MySqlPool, Row};
use std::env;
use tokio::sync::OnceCell;

// Database Connection Pool
static POOL: OnceCell<MySqlPool> = OnceCell::const_new();
async fn get_pool() -> &'static MySqlPool {
    POOL.get_or_init(|| async {
        dotenv().ok();
        let database_url: String = match env::var("DATABASE_TOKEN") {
            Ok(res) => String::from(res),
            Err(error) => panic!("Database Key not found. {:?}", error),
        };
        MySqlPool::connect(&database_url).await.unwrap()
    })
    .await
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: u32,
    operator_username: String,
    user_username: String,
    snowflake: String,
    avatar: String,
    date_creation: String,
    exp: usize,
    iat: usize,
}

// Login Reponse From Stelleri API
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    access_token: String,
}

// Guild Object
#[derive(Debug, sqlx::FromRow, Default, serde::Serialize)]
struct Guild {
    snowflake: String,
    operator_id: Option<String>,
    operator_name: Option<String>,
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
    guild_date_update: Option<chrono::DateTime<chrono::Utc>>,
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

// Fetch specific Guild
#[tauri::command]
async fn fetch_guild(username: String) -> Result<String, String> {
    let query = sqlx::query_as::<_, Guild>("SELECT guild.*, guild_settings.* FROM operator LEFT JOIN guild ON guild.operator_id = operator.snowflake LEFT JOIN guild_settings ON guild.snowflake = guild_settings.guild_snowflake WHERE username = ?;").bind(username);
    let guild: Vec<Guild> = match query.fetch_all(get_pool().await).await {
        Ok(result) => result,
        Err(e) => return Err(format!("Failed to fetch Guild: {:?}", e)),
    };

    let json_string = serde_json::to_string(&guild)
        .map_err(|e| format!("Failed to serialize Guild to JSON: {:?}", e))?;
    Ok(json_string)
}

// Session Login
#[tauri::command]
async fn login(username: &str, password: &str) -> Result<Value, String> {
    // Prepare Client
    dotenv().ok();
    let client = reqwest::Client::new();
    let prefix: String = if env::var("TAURI_BUILD").is_ok() {
        env::var("PRODUCTION_HTTP_PREFIX").unwrap()
    } else {
        env::var("DEVELOPMENT_HTTP_PREFIX").unwrap()
    };

    // Prepare Payload
    let payload = json!({
        "username": username,
        "password": password,
    });
    let payload_ser = serde_json::to_string(&payload)
        .map_err(|e| format!("Failed to serialize JSON payload: {:?}", e))?;

    // Fetch Data
    let response = client
        .post(format!("{}/login", prefix))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload_ser)
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?
        .text()
        .await
        .map_err(|e| format!("Failed to retrieve response body: {:?}", e))?;

    // Full JSON
    let parsed_response: LoginResponse = serde_json::from_str(response.as_str())
        .map_err(|e| format!("JSON syntax error: {:?}", e))?;

    // Decode Token
    let token = decode::<Claims>(
        &parsed_response.access_token,
        &DecodingKey::from_secret(env::var("JWT_TOKEN").unwrap().as_ref()),
        &Validation::default(),
    )
    .map_err(|e| format!("Failed to decode token: {:?}", e))?;

    // Payload JSON
    let mut claims_as_value: Value = to_value(&token.claims)
        .map_err(|e| format!("Failed to convert claims to JSON value: {:?}", e))?;
    Ok(claims_as_value)
}

// Entry Point
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_guild, login])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application.");
}
