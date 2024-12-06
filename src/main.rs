use actix_web::{ web, App, HttpServer, HttpRequest, HttpResponse, cookie::Key };
use actix_session::{ Session, SessionMiddleware, storage::CookieSessionStore };

use dotenv::dotenv;
use std::env;

mod response;
use response::*;

mod redis_client;
use redis_client::Cache;
use serde_json;

mod websocket;
use websocket::*;

mod solana_h;
mod middleware;
use crate::middleware::*;

mod models;
use models::*;

mod ollama;
use ollama::*;

mod postgres_db;
use postgres_db::*;

use chrono::{ DateTime, Utc };

async fn greet(_req: HttpRequest, name: web::Path<String>, session: Session) -> HttpResponse {
    let _user = get_user_from_session(&session);

    HttpResponse::Ok().body(format!("Hello {}!", name))
}

async fn current_temperature(cache: web::Data<Cache>, name: web::Path<String>) -> HttpResponse {
    let cache_key = format!("temp:{}", name);

    if let Ok(Some(cached_value)) = cache.get_value(&cache_key) {
        let cached_measurement: Temperature = serde_json
            ::from_str(&cached_value)
            .expect("Valid JSON in cache");
        return response_ok("Temperature from cache", cached_measurement);
    }

    let measurement = Temperature {
        value: 20.0,
        location: name.to_string(),
        timestamp: chrono::Utc::now(),
    };

    let _ = cache.set_value(&cache_key, serde_json::to_string(&measurement).unwrap(), 300);

    response_ok("Temperature retrieved successfully", measurement)
}

async fn health_check() -> HttpResponse {
    let local_time: DateTime<Utc> = chrono::Utc::now();

    let server_health = ServerHealth {
        timestamp: local_time,
    };
    response_ok("api is healthy", server_health)
}


#[derive(serde::Deserialize)]
struct PromptRequest {
    model: String,
    prompt: String,
    temperature: f32, 
    max_tokens: u32
}

async fn generate_prompt(req: web::Json<PromptRequest>)  -> HttpResponse {
    let ollama_client = OllamaAI::new();

    let response = ollama_client.generate_text(
        &req.model,
        &req.prompt,
        Some(req.temperature),
        Some(req.max_tokens),
    ).await;

    match response {
        Ok(text) => response_ok("api is healthy", text),
        Err(err) => response_internal_server_error(err.to_string().as_str()),
    }
    
}
























































#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let redis_url = env::var("REMOTE_REDIS_URL").unwrap();
    let redis_client = Cache::new(&redis_url).expect("Redis connection failed");
    let cache_data = web::Data::new(redis_client);

    let postgresql_url = env::var("REMOTE_POSTGRESQL_URL").unwrap();
    let db = PostgresDb::new(&postgresql_url)
        .await
        .expect("Failed to connect to database");;

    let db_pool = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            // .service(fs::Files::new("/", "./client/dist").index_file("index.html"))
            // .service(fs::Files::new("/assets", "./client/dist/assets").index_file(".*"))
            .app_data(cache_data.clone())
            .app_data(db_pool.clone())

            .default_service(
                web::route().to(|| async {
                    response_not_found("route not found")
                })
            )

            .route("/health", web::get().to(health_check))

            .service(
                web
                    ::scope("/api")
                    .wrap(Auth) // Apply auth middleware only to /api routes
                    .route("/{name}", web::get().to(greet))
                    .route("/temperature/{name}", web::get().to(current_temperature))
                    .route("/ws/", web::get().to(websocket_handler))
                    .route("/ai/generate", web::post().to(generate_prompt))
            )
    })
        .bind("127.0.0.1:9080")?
        .run().await
}
