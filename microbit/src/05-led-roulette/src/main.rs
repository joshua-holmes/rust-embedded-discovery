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
    pac::TIMER0,
};

mod vocab;


fn print_to_leds(timer: &mut Timer<TIMER0>, display: &mut Display, msg: &str, sec_per_letter: u32) {
    let mut scroller = vocab::Scroller::new(msg);
    let delay_ms = sec_per_letter * 1000 / 5;
    
    while let Ok(screen) = scroller.scroll() {
        display.show(timer, screen, delay_ms);
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    print_to_leds(&mut timer, &mut display, "HELLO WORLD", 1);

    loop {
        display.show(&mut timer, vocab::get_led_letter(&'Q'), 1000);

        display.clear();
        timer.delay_ms(1000u32);
    }
}
