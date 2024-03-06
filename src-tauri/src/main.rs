#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

// Dependencies
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, to_value, Map, Value};
use sqlx::{mysql::*, FromRow, MySqlPool, Row};
use std::collections::HashMap;
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
    snowflake: String,
    operator_username: String,
    user_username: String,
    email: String,
    service_tag: String,
    avatar: String,
    date_creation: chrono::DateTime<chrono::Utc>,
    date_update: Option<chrono::DateTime<chrono::Utc>>,
    exp: usize,
    iat: usize,
}

// Login Reponse From Apricaria API
// Option 'message' is reserved for errors only.
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    access_token: Option<String>,
    message: Option<String>,
}

// Guild Picture From Apricaria API
// Option 'message' is reserved for errors only.
#[derive(Debug, Serialize, Deserialize)]
struct GuildPictureResponse {
    picture_urls: Option<Vec<String>>,
    message: Option<String>,
}

// Guild Object
#[derive(Debug, sqlx::FromRow, Default, serde::Serialize)]
struct Guild {
    snowflake: String,
    team_tag: String,
    name: String,
    channel_admin: Option<String>,
    channel_broadcast: Option<String>,
    channel_event: Option<String>,
    channel_suggestion: Option<String>,
    channel_snippet: Option<String>,
    channel_rules: Option<String>,
    role_blinded: Option<String>,
    locale: String,
    disabled: bool,
    production: bool,
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
    let query = sqlx::query_as::<_, Guild>("SELECT guild.*, guild_settings.* FROM operator_team LEFT JOIN guild ON guild.team_tag = operator_team.team_tag LEFT JOIN guild_settings ON guild_settings.guild_snowflake = guild.snowflake LEFT JOIN operator_member ON operator_member.team_tag = operator_team.team_tag LEFT JOIN operator ON operator.snowflake = operator_member.snowflake WHERE username = ? AND guild.snowflake IS NOT NULL;").bind(username);
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

    if let Some(access_token) = parsed_response.access_token.as_deref() {
        // Decode Token
        let token = decode::<Claims>(
            &access_token,
            &DecodingKey::from_secret(env::var("JWT_TOKEN").unwrap().as_ref()),
            &Validation::default(),
        )
        .map_err(|e| format!("Failed to decode token: {:?}", e))?;

        // Raw Response JSON
        let mut claims: Value = to_value(&token.claims)
            .map_err(|e| format!("Failed to convert claims to JSON value: {:?}", e))?;

        // Add Access Token
        let mut fields = HashMap::new();
        fields.insert("access_token".to_string(), access_token.to_string());
        let modified_claims = merge(&claims, &fields);

        Ok(modified_claims)
    } else if let Some(error_message) = parsed_response.message {
        Err(error_message)
    } else {
        Err(String::from("Failed to serialize."))
    }
}

// Fetch Guild Picture
#[tauri::command]
async fn guild_picture(
    token: &str,
    snowflakes: Vec<String>,
) -> Result<GuildPictureResponse, String> {
    // Prepare Client
    dotenv().ok();
    let client = reqwest::Client::new();
    let prefix: String = if env::var("TAURI_BUILD").is_ok() {
        env::var("PRODUCTION_HTTP_PREFIX").unwrap()
    } else {
        env::var("DEVELOPMENT_HTTP_PREFIX").unwrap()
    };

    // Prepare Payload
    let payload = json!(snowflakes);
    let payload_ser = serde_json::to_string(&payload)
        .map_err(|e| format!("Failed to serialize JSON payload: {:?}", e))?;

    // Fetch Data
    let response = client
        .get(format!("{}/guilds/picture", prefix))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .body(payload_ser)
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?
        .text()
        .await
        .map_err(|e| format!("Failed to retrieve response body: {:?}", e))?;

    // Full JSON
    let parsed_response: GuildPictureResponse = serde_json::from_str(&response.as_str())
        .map_err(|e| format!("JSON syntax error: {:?}", e))?;

    if let Some(picture_urls) = parsed_response.picture_urls.as_deref() {
        Ok(parsed_response)
    } else if let Some(error_message) = parsed_response.message {
        Err(error_message)
    } else {
        Err(String::from("Failed to serialize."))
    }
}

// Merge JSON Keys
fn merge(v: &Value, fields: &HashMap<String, String>) -> Value {
    match v {
        Value::Object(m) => {
            let mut cloned_map = m.clone();
            for (k, v) in fields {
                cloned_map.insert(k.clone(), Value::String(v.clone()));
            }
            Value::Object(cloned_map)
        }
        _ => v.clone(),
    }
}

// Entry Point
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![fetch_guild, login, guild_picture])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application.");
}
