use std::fmt;

mod consistent;
mod rendezvous;
mod simple;

use self::consistent::*;
use self::rendezvous::*;
use self::simple::*;

use crate::*;

pub(crate) trait HashRouter: fmt::Display {
    fn set_targets(&mut self, targets: Vec<Target>);
    fn route(&self, key: &str) -> Target;
}

pub(crate) fn routers() -> Vec<Box<dyn HashRouter>> {
    vec![
        Box::new(Simple::default()),
        Box::new(Consistent::default()),
        Box::new(Rendezvous::default()),
    ]
}
