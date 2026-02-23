use actix_web::{middleware::from_fn, web, App, HttpServer};
use fintrack::{api, configuration, AppState, Settings};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let settings = Settings::from_env()
        .expect("Failed to load configuration");

    // Create database pool
    let pool = configuration::database::create_pool(&settings.database_url)
        .await
        .expect("Failed to create database pool");

    // Create app state
    let app_state = web::Data::new(AppState {
        pool,
        jwt_secret: settings.jwt_secret.clone(),
    });

    println!("Server starting at http://{}:{}", settings.host, settings.port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // Public routes (no auth required)
            .configure(api::configure_auth)
            // Protected routes (auth required)
            .service(
                web::scope("/api")
                    .wrap(from_fn(api::verify_jwt))  // ← Apply middleware
                    .configure(api::configure_users)
                    .configure(api::configure_categories)
            )
    })
    .bind((settings.host.as_str(), settings.port))?
    .run()
    .await
}
