use microbit::{
    display::blocking::Display,
    hal::Timer,
    pac::TIMER0,
};

use super::vocab::Scroller;

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

pub fn circles(timer: &mut Timer<TIMER0>, display: &mut Display, speed: u8) {
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

pub fn print(timer: &mut Timer<TIMER0>, display: &mut Display, msg: &str, speed: u8) {
    let mut scroller = Scroller::new(msg);
    let delay_ms = 5500 / (speed as u32 + 10);
    // let delay_ms = ms_per_letter / 5;
    
    while let Ok(screen) = scroller.scroll() {
        display.show(timer, screen, delay_ms);
    }
}

