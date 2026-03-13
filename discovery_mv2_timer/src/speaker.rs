use core::cell::RefCell;
use cortex_m::interrupt::{free as interrupt_free, Mutex};
use critical_section_lock_mut::LockMut;
use embedded_hal::{delay::DelayNs, digital::OutputPin};

use microbit::{
    hal::{
        gpio,
        pac::{self, interrupt},
        timer,
    },
    Board,
};

const BASE_FREQ: u32 = 440;
const FREQ_RISE: u32 = 220;
const RISE_TIME:u32 = 500_000;

type SpeakerPin = gpio::Pin<gpio::Output<gpio::PushPull>>;
type SirenTimer = timer::Timer<pac::TIMER0>;

pub struct Siren {
    timer: SirenTimer,
    speaker_pin: SpeakerPin,
    pin_high: bool,
    cur_time: u32,
}

impl Siren {
    pub fn new(speaker_pin: SpeakerPin, timer: SirenTimer) -> Self {
        Self {
            timer,
            speaker_pin,
            pin_high: false,
            cur_time: 0,
        }
    }

    pub fn start(&mut self) {
        self.speaker_pin.set_low().unwrap();
        self.pin_high = false;
        self.cur_time = 0;
        self.timer.enable_interrupt();
        self.timer.start(1_000_000 / BASE_FREQ);
    }

    pub fn stop(&mut self) {
        self.timer.disable_interrupt();
    }

    pub fn step(&mut self) {
        if self.pin_high {
            self.speaker_pin.set_low().unwrap();
            self.pin_high = false;
        } else {
            self.speaker_pin.set_high().unwrap();
            self.pin_high = true;
        }

        while self.cur_time >= 2 * RISE_TIME {
            self.cur_time -= 2 * RISE_TIME;
        }
        let cycle_time = if self.cur_time < RISE_TIME {
            self.cur_time
        } else {
            2 * RISE_TIME - self.cur_time
        };
        let frequency = BASE_FREQ + FREQ_RISE * cycle_time / RISE_TIME;
        let period = 1_000_000 / frequency;

        self.cur_time += period / 2;

        self.timer.reset_event();
        self.timer.start(period / 2);
    }
}

pub static SIREN: LockMut<Siren> = LockMut::new();

#[interrupt]
fn TIMER0() {
    interrupt_free(|cs| {
        if let Some(siren) = SIREN.borrow(cs).borrow_mut().as_mut() {
            siren.handle_siren_event();
        }
    })
}
