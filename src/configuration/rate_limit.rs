use actix_governor::{Governor, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_governor::governor::middleware::NoOpMiddleware;

type RateLimiter = Governor<PeerIpKeyExtractor, NoOpMiddleware>;

/// Create rate limiter for authentication endpoints (2 requests per second)
pub fn auth_rate_limiter() -> RateLimiter {
    Governor::new(
        &GovernorConfigBuilder::default()
            .requests_per_second(2)  // Use this instead!
            .burst_size(5)
            .finish()
            .unwrap()
    )
}

/// Create rate limiter for API endpoints (10 requests per second)
pub fn api_rate_limiter() -> RateLimiter {
    Governor::new(
        &GovernorConfigBuilder::default()
            .requests_per_second(10)
            .burst_size(20)
            .finish()
            .unwrap()
    )
}

/// Create rate limiter for expensive operations (5 requests per second)
pub fn expensive_rate_limiter() -> RateLimiter {
    Governor::new(
        &GovernorConfigBuilder::default()
            .requests_per_second(5)
            .burst_size(10)
            .finish()
            .unwrap()
    )
}