use anyhow::Result;
use dashmap::DashMap;
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    pub data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter.value_mut() += 1;
        Ok(())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
