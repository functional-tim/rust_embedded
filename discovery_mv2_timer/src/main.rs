#![no_main]
#![no_std]

mod display;
mod controls;
mod counter;
mod speaker;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board::Board, display::nonblocking::GreyscaleImage, hal::{gpio, Timer}};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use controls::{init_buttons, get_counter};
use counter::Controller;
use display::{clear_display, counter_leds, display_image, init_display};

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

    let mut speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::Low).degrade();

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
        if controller.counter == 1 {
            controller.count_down();
            clear_display();
            for _ in 0..CYCLES {
                speaker_pin.set_high().unwrap();
                timer.delay_ms(PERIOD / 2);
                speaker_pin.set_low().unwrap();
                timer.delay_ms(PERIOD / 2);
            }
        } else if controller.counter == 0 {
            clear_display();
        }
        controller.step(get_counter(true));
    }
}
