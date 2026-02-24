use actix_web::{middleware::from_fn, web, App, HttpServer};
use fintrack::{api, configuration, AppState, Settings};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::from_env()
        .expect("Failed to load configuration");

    let pool = configuration::database::create_pool(&settings.database_url)
        .await
        .expect("Failed to create database pool");

    let app_state = web::Data::new(AppState {
        pool,
        jwt_secret: settings.jwt_secret.clone(),
    });

    println!("Server starting at http://{}:{}", settings.host, settings.port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // Public routes with rate limiting
            .service(
                web::scope("/auth")
                    .wrap(configuration::auth_rate_limiter())
                    .configure(api::configure_auth)
            )
            // Protected routes with JWT THEN rate limiting
            .service(
                web::scope("/api")
                    .wrap(from_fn(api::verify_jwt))
                    .wrap(configuration::api_rate_limiter())
                    .configure(api::configure_users)
                    .configure(api::configure_categories)
                    .configure(api::configure_transactions)
            )
    })
    .bind((settings.host.as_str(), settings.port))?
    .run()
    .await
}