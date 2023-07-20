#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use lsm303agr::Lsm303agr;
use microbit::hal::twim;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), twim::Frequency::K100);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(lsm303agr::AccelOutputDataRate::Hz100);
    sensor.set_accel_scale(lsm303agr::AccelScale::G8);

    loop {}
}
