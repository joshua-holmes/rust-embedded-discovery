#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use lsm303agr::{Lsm303agr, AccelOutputDataRate};
use microbit::{hal::{twim, Timer}, display::blocking::Display};
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

mod leds;
mod calc;

const ACCEL_THRESHOLD: f32 = 1500.;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), twim::Frequency::K100);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz100).unwrap();
    sensor.set_accel_scale(lsm303agr::AccelScale::G8).unwrap();

    loop {
        while !sensor.accel_status().unwrap().xyz_new_data {}
        let data = sensor.accel_data().unwrap();
        let mag = calc::get_magnitude(data.x, data.y, data.z);

        if mag >= ACCEL_THRESHOLD {
            let max_g = calc::record_max_accel_as_i32(&mut sensor, AccelOutputDataRate::Hz100).unwrap();
            let screen = leds::num_to_screen(max_g).unwrap();
            display.show(&mut timer, screen, 1000);
        }
    }
}
