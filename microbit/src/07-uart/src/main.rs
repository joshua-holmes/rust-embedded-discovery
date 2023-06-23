#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use heapless::Vec;
use core::{
    fmt::Write,
    str,
};

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
    pac::UART0,
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

    const BUFFER_SIZE: usize = 1024;

    let mut buffer: Vec<u8, BUFFER_SIZE> = Vec::new();
    let mut utf8_char: Vec<u8, 4> = Vec::new();
    let mut end_buffer: bool = false;
    loop {
        let byte = if end_buffer {
            13
        } else {
            nb::block!(serial.read()).unwrap()
        };

        if byte == 13 {
            buffer.reverse();
            let string = str::from_utf8(&buffer[..]);
            match string {
                Ok(s) => {
                    write!(serial, "\r\n{}\r\n\r\n", s).unwrap();
                    nb::block!(serial.flush()).unwrap();
                },
                Err(err) => {
                    rprintln!("Failed to decode string: {}", err);
                }
            }
            // reset data
            buffer = Vec::new();
            utf8_char = Vec::new();
            end_buffer = false;
        } else {
            if utf8_char.push(byte).is_err() {
                rprintln!("utf8_char is full");
                continue;
            }

            let mut write_success = false;
            if let Ok(char) = str::from_utf8(&utf8_char[..]) {
                write!(serial, "{}", char).unwrap();
                nb::block!(serial.flush()).unwrap();

                let space_left = buffer.capacity() - buffer.len();
                if space_left <= utf8_char.len() {
                    write!(serial, "\r\nBuffer is full.").unwrap();
                    rprintln!("Buffer is full.");
                    end_buffer = true;
                    if space_left < utf8_char.len() {
                        continue;
                    }
                }

                for b in utf8_char.iter().rev() {
                    buffer.push(*b).unwrap();
                }

                write_success = true;
            }

            if utf8_char.is_full() || write_success {
                utf8_char = Vec::new();
            }
        }
    }
}
