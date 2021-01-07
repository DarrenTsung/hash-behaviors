mod consistent;
mod rendezvous;
mod simple;

pub use self::consistent::*;
pub use self::rendezvous::*;
pub use self::simple::*;

pub trait HashRouter {
    fn set_targets(&mut self, targets: Vec<String>);
    fn route(&self, key: &str) -> &str;
}
