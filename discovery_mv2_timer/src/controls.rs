mod init;
mod interrupt;

pub use init::init_buttons;

use crate::counter::Counter;
use core::cell::RefCell;
use cortex_m::interrupt::{free as interrupt_free, Mutex};
use microbit::{board::Buttons, hal::gpiote::Gpiote};

pub static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
pub static COUNTER: Mutex<RefCell<Counter>> = Mutex::new(RefCell::new(Counter::None));

pub fn get_counter(reset: bool) -> Counter {
    interrupt_free(|cs| {
        let counter = *COUNTER.borrow(cs).borrow();
        if reset {
            *COUNTER.borrow(cs).borrow_mut() = Counter::None;
        }
        counter
    })
}
