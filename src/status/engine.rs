use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct EngineStatus {
    pub commit: Option<String>,
    pub db: DbStatus,
    pub cache: CacheStatus,
}

impl EngineStatus {
    pub fn healthy(&self) -> bool {
        self.db.healthy
            && self.db.error.is_none()
            && self.cache.healthy
            && self.cache.error.is_none()
    }
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct DbStatus {
    pub healthy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub max_connections: usize,
    pub min_connections: usize,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct CacheStatus {
    pub healthy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub pool_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let values_unhealthy = EngineStatus::default();

        assert!(!values_unhealthy.healthy());

        let mut values_healthy = EngineStatus::default();
        values_healthy.db.healthy = true;
        values_healthy.cache.healthy = true;

        assert!(values_healthy.healthy());
    }
}
