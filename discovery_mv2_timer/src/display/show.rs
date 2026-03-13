use super::DISPLAY;

use cortex_m::interrupt::free as interrupt_free;

use tiny_led_matrix::Render;

/// Clear the display (turn off all LEDs).
pub fn clear_display() {
    interrupt_free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.clear();
        }
    })
}

pub fn counter_leds(counter: u16) -> [[u8; 5]; 5] {
    let mut output: [[u8; 5]; 5] = [[0; 5]; 5];
    if counter > 60 {
        let minutes = counter / 60;
        let seconds = counter - (minutes * 60);
        output[0] = number_binary(minutes);
        if seconds > 40 {
            output[3] = [1, 1, 1, 1, 1];
            output[4] = [1, 1, 1, 1, 1];
            for i in 0..5 {
                output[2][4 - i] = if seconds > 56 - (i as u16 * 4) { 1 } else { 0 };
            }
        } else if seconds > 20 {
            output[4] = [1, 1, 1, 1, 1];
            for i in 0..5 {
                output[3][4 - i] = if seconds > 36 - (i as u16 * 4) { 1 } else { 0 };
            }
        } else {
            for i in 0..5 {
                output[4][4 - i] = if seconds > 16 - (i as u16 * 4) { 1 } else { 0 };
            }
        }
    } else {
        if counter > 40 {
            output[3] = [1, 1, 1, 1, 1];
            output[4] = [1, 1, 1, 1, 1];
            for i in 0..5 {
                output[2][4 - i] = if counter > 56 - (i as u16 * 4) { 1 } else { 0 };
            }
        } else if counter > 20 {
            output[4] = [1, 1, 1, 1, 1];
            for i in 0..5 {
                output[3][4 - i] = if counter > 36 - (i as u16 * 4) { 1 } else { 0 };
            }
        } else if counter > 0 {
            for i in 0..5 {
                output[4][4 - i] = if counter > 16 - (i as u16 * 4) { 1 } else { 0 };
            }
        }
    }
    output
}

/// Display an image.
pub fn display_image(image: &impl Render) {
    interrupt_free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.show(image);
        }
    })
}

pub fn number_binary(number: u16) -> [u8; 5] {
    let mut output: [u8; 5] = [0; 5];
    let mut num = number;
    for i in 0..5 {
        output[4 - i] = (num % 2) as u8;
        num = num / 2;
    }
    output
}
