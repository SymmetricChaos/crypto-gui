use lazy_static::lazy_static;
use rand::{SeedableRng, prelude::StdRng};
use std::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL_RNG: Mutex<StdRng> = Mutex::new(StdRng::from_entropy());
}

pub fn seed_global_rng(n: u64) {
    *GLOBAL_RNG.lock().unwrap() = StdRng::seed_from_u64(n);
}
 
pub fn randomize_global_rng() {
    *GLOBAL_RNG.lock().unwrap() = StdRng::from_entropy();
}
 
pub fn get_gobal_rng() -> std::sync::MutexGuard<'static, StdRng> {
    GLOBAL_RNG.lock().unwrap()
}