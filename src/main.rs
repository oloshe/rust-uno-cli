#![warn(unused)]

use loopper::Loopper;

mod net;
mod base;

mod dealer;
mod loopper;
fn main() {
    let mut looper = Loopper::new();
    looper.bootstrap();
}