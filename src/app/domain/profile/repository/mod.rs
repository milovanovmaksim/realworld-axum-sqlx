pub mod entities;

use async_trait::async_trait;

#[async_trait]
pub trait ProfileRepository: Send + Sync + 'static {}
