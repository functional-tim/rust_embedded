use super::SPEAKER;

use cortex_m::interrupt::free as interrupt_free;
use microbit::pac::{self, interrupt};

#[pac::interrupt]
fn TIMER0() {
    interrupt_free(|cs| {
        if let Some(speaker) = SPEAKER.borrow(cs).borrow_mut().as_mut() {
            speaker.handle_speaker_event();
        }
    })
}
