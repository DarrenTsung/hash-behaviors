use std::fmt;

mod consistent;
mod rendezvous;
mod simple;

use self::consistent::*;
use self::rendezvous::*;
use self::simple::*;

pub trait HashRouter: fmt::Display {
    fn set_targets(&mut self, targets: Vec<String>);
    fn route(&self, key: &str) -> &str;
}

pub fn routers() -> Vec<Box<dyn HashRouter>> {
    vec![
        Box::new(Simple::default()),
        Box::new(Consistent::default()),
        Box::new(Rendezvous::default()),
    ]
}
