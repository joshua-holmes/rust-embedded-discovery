#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use core::{
    fmt::Write,
    str::from_utf8,
};

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };


    let mut bytes: [u8; 4] = [0; 4];
    let mut b_index: usize = 0;
    loop {
        bytes[b_index] = nb::block!(serial.read()).unwrap();
        let letter = from_utf8(&bytes[0..=b_index]);
        match letter {
            Ok(val) => {
                write!(serial, "{}", val).unwrap();
                nb::block!(serial.flush()).unwrap();
                bytes = [0; 4];
                b_index = 0;
            },
            Err(err) => {
                rprintln!("bad {:?}", err);
                b_index += 1;
            }
        }
    }
}
