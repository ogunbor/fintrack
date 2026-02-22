pub mod auth;
pub mod users;

// Re-export configure functions for easy access
pub use auth::configure as configure_auth;
pub use users::configure as configure_users;