#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;
use microbit::{board::Board, display::nonblocking::GreyscaleImage, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

mod display;

use display::{clear_display, display_image, init_display};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut counter: u16 = 0;

    init_display(board.TIMER1, board.display_pins);
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    loop {
        if button_a.is_low().unwrap() {
            if counter <= 1740 {
                counter += 60;
            }
        } else if button_b.is_low().unwrap() {
            if counter <= 1500 {
                counter += 300;
            }
        } 
        let matrix = GreyscaleImage::new(&counter_leds(counter));
        display_image(&matrix);
        timer.delay_ms(200u32);
        if counter > 0 {
            counter -= 1;
        } else {
            clear_display();
        }
        timer.delay_ms(800_u32)
    }
}

fn number_binary(number: u16) -> [u8; 5] {
    let mut output: [u8; 5] = [0; 5];
    let mut num = number;
    for i in 0..5 {
        output[i] = (num % 2) as u8;
        num = num / 2;
    }
    output
}

fn counter_leds(counter: u16) -> [[u8; 5]; 5] {
    let mut output: [[u8; 5]; 5] = [[0; 5]; 5];
    if counter > 60 {
        let minutes = counter / 60;
        let seconds = counter - (minutes * 60);
        output[0] = number_binary(minutes);
        if seconds > 40 {
            output[3] = [1, 1, 1, 1, 1];
            output[4] = [1, 1, 1, 1, 1];
            for i in 0..5 {
                output[2][4 - i] = if seconds >= 60 - (i as u16 * 4)  { 1 } else { 0 };
            }
        } else if seconds > 20 {
            output[4] = [1, 1, 1, 1, 1];
            for i in 0..5 {
                output[3][4 - i] = if seconds >= 40 - (i as u16 * 4)  { 1 } else { 0 };
            }
        } else {
            for i in 0..5 {
                output[4][4 - i] = if seconds >= 20 - (i as u16 * 4)  { 1 } else { 0 };
            }
        }
    } else {
        if counter > 40 {
            output[3] = [1, 1, 1, 1, 1];
            output[4] = [1, 1, 1, 1, 1];
            for i in 0..5 {
                output[2][4 - i] = if counter >= 60 - (i as u16 * 4)  { 1 } else { 0 };
            }
        } else if counter > 20 {
            output[4] = [1, 1, 1, 1, 1];
            for i in 0..5 {
                output[3][4 - i] = if counter >= 40 - (i as u16 * 4)  { 1 } else { 0 };
            }
        } else if counter > 0 {
            for i in 0..5 {
                output[4][4 - i] = if counter >= 20 - (i as u16 * 4)  { 1 } else { 0 };
            }
        }
    }
    output
}
