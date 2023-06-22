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

    // let mut string: &str = "";

    let mut bytes: [u8; 1024] = [0; 1024];
    let mut b_index: usize = 0;
    loop {
        let byte = nb::block!(serial.read()).unwrap();
        bytes[b_index] = byte;
        if byte == 13 {
            let string = from_utf8(&bytes);
            match string {
                Ok(s) => {
                    write!(serial, "{}\r\n", s).unwrap();
                    nb::block!(serial.flush()).unwrap();
                },
                Err(err) => {
                    rprintln!("Failed to decode string: {}", err);
                }
            }
            bytes = [0; 1024];
            b_index = 0;
        } else {
            b_index += 1;
        }
    }
}
