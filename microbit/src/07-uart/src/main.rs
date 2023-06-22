#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
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
    pac::UARTE0 as UART0,
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

    const MAX_BYTES: usize = 16;

    let mut bytes: [u8; MAX_BYTES] = [0; MAX_BYTES];
    let mut bytes_length: usize = MAX_BYTES;
    let mut b_index: usize = MAX_BYTES - 1;
    let mut utf8_char: [u8; 4] = [0; 4];
    let mut utf8_index: usize = 0;
    let mut end_buffer: bool = false;
    loop {
        rprintln!("at loop {:?}", bytes);
        let byte = if end_buffer {
            13
        } else {
            nb::block!(serial.read()).unwrap()
        };

        if byte == 13 {
            let string = str::from_utf8(&bytes[b_index..]);
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
            bytes = [0; MAX_BYTES];
            b_index = MAX_BYTES - 1;
            bytes_length = MAX_BYTES;
            utf8_char = [0; 4];
            utf8_index = 0;
            end_buffer = false;
            rprintln!("After reset {:?}", bytes);
        } else {
            utf8_char[utf8_index] = byte;
            if let Ok(char) = str::from_utf8(&utf8_char) {
                if bytes_length <= utf8_index + 1 {
                    write!(serial, "\r\nBuffer is full.");
                    rprintln!("Buffer is full.");
                    end_buffer = true;
                    if bytes_length < utf8_index + 1 {
                        continue;
                    }
                }

                for (i, b) in utf8_char[..=utf8_index].iter().enumerate() {
                    bytes[b_index + i] = *b;
                }
                utf8_index = 0;
                bytes_length = b_index;
                write!(serial, "{}", char);
                nb::block!(serial.flush()).unwrap();
            } else if utf8_index < 3 {
                utf8_index += 1;
            } else {
                rprintln!("Something ain't right: {}, {:?}", utf8_index, utf8_char);
                utf8_index = 0;
            }

            if utf8_index == 0 {
                utf8_char = [0; 4];
            }

            if b_index != 0 {
                b_index -= 1;
            }
        }
    }
}
