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


#[derive(Debug)]
enum Direction {
    GoRight,
    GoDown,
    GoLeft,
    GoUp
}

impl Direction {
    fn get_direction(x: usize, y: usize) -> Self {
        if x < 4 && y == 0 {
            Direction::GoRight
        } else if x == 4 && y < 4 {
            Direction::GoDown
        } else if x > 0 && y == 4 {
            Direction::GoLeft
        } else if x == 0 && y > 0 {
            Direction::GoUp
        } else {
            panic!("LEDs cannot move in a circle because of x and y: {}, {}", x, y);
        }
    }
}


fn print_to_leds(timer: &mut Timer<TIMER0>, display: &mut Display, msg: &str, speed: u8) {
    let mut scroller = vocab::Scroller::new(msg);
    let delay_ms = 5500 / (speed as u32 + 10);
    // let delay_ms = ms_per_letter / 5;
    
    while let Ok(screen) = scroller.scroll() {
        display.show(timer, screen, delay_ms);
    }
}

fn led_circles(timer: &mut Timer<TIMER0>, display: &mut Display, speed: u8) {
    let (mut x, mut y) = (0, 0);
    let delay_ms = 5500 / (speed as u32 + 10);
    
    loop {
        let mut screen = [[0; 5]; 5];
        screen[y][x] = 1;
        display.show(timer, screen, delay_ms);

        let direction = Direction::get_direction(x, y);
        match direction {
            Direction::GoRight => x += 1,
            Direction::GoDown  => y += 1,
            Direction::GoLeft  => x -= 1,
            Direction::GoUp    => y -= 1,
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    print_to_leds(&mut timer, &mut display, "HELLO WORLD", 30);

    led_circles(&mut timer, &mut display, 100);
    loop {}
}
