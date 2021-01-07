use super::*;
use crate::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Uses a rendezvous hashing algorithm (also known as highest-random-weight).
///
/// https://en.wikipedia.org/wiki/Rendezvous_hashing
/// https://medium.com/i0exception/rendezvous-hashing-8c00e2fb58b0
#[derive(Debug, Default)]
pub struct Rendezvous(Vec<Target>);

impl std::fmt::Display for Rendezvous {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rendezvous")
    }
}

impl HashRouter for Rendezvous {
    fn set_targets(&mut self, targets: Vec<Target>) {
        self.0 = targets;
    }

    fn route(&self, key: &str) -> Target {
        let targets = &self.0;

        let mut highest_value = None;
        let mut chosen_target = None;
        for target in targets {
            let target_key_hash = {
                let mut hasher = DefaultHasher::new();
                target.hash(&mut hasher);
                key.hash(&mut hasher);
                hasher.finish()
            };

            if highest_value.is_none() || target_key_hash > highest_value.expect("exists") {
                highest_value = Some(target_key_hash);
                chosen_target = Some(target);
            }
        }

        *chosen_target.expect("exists")
    }
}
