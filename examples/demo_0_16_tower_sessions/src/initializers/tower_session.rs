use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};

pub struct AxumSessionInitializer;

#[async_trait]
impl Initializer for AxumSessionInitializer {
    fn name(&self) -> String {
        "axum-session".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::minutes(5)));
        let router = router.layer(session_layer);
        Ok(router)
    }
}
