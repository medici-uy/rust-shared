use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct Monitoring {
    pub commit: Option<String>,
    pub db: MonitoringDb,
    pub cache: MonitoringCache,
}

impl Monitoring {
    pub fn ok(&self) -> bool {
        self.db.ok && self.db.error.is_none() && self.cache.ok && self.cache.error.is_none()
    }
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct MonitoringDb {
    pub ok: bool,
    pub error: Option<String>,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub max_connections: usize,
    pub min_connections: usize,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct MonitoringCache {
    pub ok: bool,
    pub error: Option<String>,
    pub pool_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let values_not_ok = Monitoring::default();

        assert!(!values_not_ok.ok());

        let mut values_ok = Monitoring::default();
        values_ok.db.ok = true;
        values_ok.cache.ok = true;

        assert!(values_ok.ok());
    }
}
