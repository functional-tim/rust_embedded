#![no_main]
#![no_std]

mod display;
mod controls;
mod counter;
mod speaker;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};
use microbit::{board::Board, display::nonblocking::GreyscaleImage, hal::{gpio, Timer}, pac};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use controls::{init_buttons, get_counter};
use counter::Controller;
use display::{clear_display, counter_leds, display_image, init_display};
use speaker::{Siren, SIREN};

// The period is the time per cycle. It is 1/f where f is frequency in Hz.
const PERIOD: u32 = 1000 / 220;

// Number of cycles for 10 seconds of output.
const CYCLES: u32 = 10000 / PERIOD;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut controller = Controller::new();

    init_display(board.TIMER1, board.display_pins);
    init_buttons(board.GPIOTE, board.buttons);

    let speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::Low).degrade();
    let siren = Siren::new(speaker_pin, timer);
    SIREN.init(siren);
    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER0) };
    pac::NVIC::unpend(pac::Interrupt::TIMER0);

    loop {
//        if button_a.is_low().unwrap() {
//            if counter <= 1860 {
//                counter += 60;
//            }
//        } else if button_b.is_low().unwrap() {
//            if counter <= 1620 {
//                counter += 300;
//            }
//        } 
        let matrix = GreyscaleImage::new(&counter_leds(controller.counter));
        display_image(&matrix);
        timer.delay_ms(1000u32);
        match controller.counter {
            1 => {
            controller.count_down();
            clear_display();
            SIREN.with_lock(|siren| siren.start());
            timer.delay_ms(5_000);
            SIREN.with_lock(|siren| siren.stop());
            },
            0 => {
                clear_display();
            },
            _ => {
                controller.step(get_counter(true));
            }
        }
    }
}
