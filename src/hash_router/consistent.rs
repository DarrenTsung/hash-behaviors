use super::*;
use crate::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const HASHES_TO_GENERATE_PER_TARGET: u32 = 200;

/// Uses a consistent hashing algorithm as described here:
/// https://www.metabrew.com/article/libketama-consistent-hashing-algo-memcached-clients.
#[derive(Debug, Default)]
pub struct Consistent {
    targets: Vec<Target>,
    target_hashes: Vec<(u64, usize)>,
}

impl std::fmt::Display for Consistent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Consistent")
    }
}

impl HashRouter for Consistent {
    fn set_targets(&mut self, targets: Vec<Target>) {
        self.target_hashes = {
            let mut hashes = vec![];
            for (target_index, target) in targets.iter().enumerate() {
                for salt in 0..HASHES_TO_GENERATE_PER_TARGET {
                    let target_hash = {
                        let mut hasher = DefaultHasher::new();
                        target.hash(&mut hasher);
                        salt.hash(&mut hasher);
                        hasher.finish()
                    };
                    hashes.push((target_hash, target_index));
                }
            }
            hashes.sort_by_key(|(hash, _)| *hash);
            hashes
        };
        self.targets = targets;
    }

    fn route(&self, key: &str) -> Target {
        let key_hash = {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish()
        };

        let target_hashes_index = match self
            .target_hashes
            .binary_search_by_key(&key_hash, |(hash, _)| *hash)
        {
            Ok(found_index) => found_index,
            Err(insert_index) => {
                if insert_index == self.target_hashes.len() {
                    0
                } else {
                    insert_index
                }
            }
        };
        let target_index = self.target_hashes[target_hashes_index].1;

        self.targets[target_index]
    }
}
