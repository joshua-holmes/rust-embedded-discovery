#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use lsm303agr::{Lsm303agr, AccelOutputDataRate};
use microbit::{
    hal::twim,
    display::nonblocking::{
        Display, GreyscaleImage
    }
};
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

mod leds;
mod calc;

const ACCEL_THRESHOLD: f32 = 1500.;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let mut display = Display::new(board.TIMER0, board.display_pins);

    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), twim::Frequency::K100);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz100).unwrap();
    sensor.set_accel_scale(lsm303agr::AccelScale::G8).unwrap();
    rprintln!("first");

    let recording_screen = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 7, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    critical_section::with(|| {
        display.show(&GreyscaleImage::new(&recording_screen));
    });
    rprintln!("second");
    loop {
    rprintln!("in loop");
        while !sensor.accel_status().unwrap().xyz_new_data {}
        let data = sensor.accel_data().unwrap();
        let mag = calc::get_magnitude(data.x, data.y, data.z);

        if mag >= ACCEL_THRESHOLD {
            rprintln!("met threshold");
            display.show(&GreyscaleImage::new(&recording_screen));
            let max_g = calc::record_max_accel_as_i32(&mut sensor, AccelOutputDataRate::Hz100).unwrap();
            let screen = leds::num_to_screen(max_g).unwrap();
            display.show(&GreyscaleImage::new(&screen));
        }
    }
}
