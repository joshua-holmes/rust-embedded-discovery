#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

mod vocab;


fn print_to_leds(led_matrix: &mut [[u8; 5]; 5], msg: &str) {
    
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let light_it_all = [[0; 5]; 5];

    loop {
        display.show(&mut timer, vocab::get_led_letter("Q"), 1000);

        display.clear();
        timer.delay_ms(1000u32);
    }
}
