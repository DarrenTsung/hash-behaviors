use super::*;
use crate::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Hashes the key and indexes into the set of targets.
#[derive(Debug, Default)]
pub struct Simple(Vec<Target>);

impl std::fmt::Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Simple")
    }
}

impl HashRouter for Simple {
    fn set_targets(&mut self, targets: Vec<Target>) {
        self.0 = targets;
    }

    fn route(&self, key: &str) -> Target {
        let key_hash = {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish()
        };

        self.0[key_hash as usize % self.0.len()]
    }
}
