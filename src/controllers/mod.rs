pub mod middleware;

// Export the appropriate OAuth2 module as "oauth2" based on feature flags
// Priority: axum_session takes precedence over use_tower_sessions
#[cfg(all(feature = "axum_session", not(feature = "use_tower_sessions")))]
pub use oauth2_axum_session as oauth2;

#[cfg(feature = "use_tower_sessions")]
pub use oauth2_tower_sessions as oauth2;

// Keep the original module names available for direct access if needed
#[cfg(feature = "axum_session")]
pub mod oauth2_axum_session;

#[cfg(feature = "use_tower_sessions")]
pub mod oauth2_tower_sessions;
