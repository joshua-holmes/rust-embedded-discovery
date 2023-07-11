#![no_main]
#![no_std]

use core::{
    fmt::Write,
    str
};

use cortex_m_rt::entry;
use heapless::Vec;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::hal::prelude::*;

#[cfg(feature = "v1")]
use microbit::{
    hal::uart,
    hal::uart::{Baudrate, Parity},
    hal::twi,
    pac::twi0::frequency::FREQUENCY_A,
};

#[cfg(feature = "v2")]
use microbit::{
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;


use lsm303agr::{
    AccelOutputDataRate, Lsm303agr
};

const MAX_INPUT_LEN: usize = 16;

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
            Baudrate::BAUD115200
        );
        UartePort::new(serial)
    };

    #[cfg(feature = "v1")]
    let mut i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz1).unwrap();

    let msg_intro = "Please type either \"accelerometer\" or \"magnetometer\" to view data from that device.\r\n";
    serial.write_str(msg_intro);
    serial.flush();

    let mut input: Vec<u8, 16> = Vec::new();
    loop {
        let byte = nb::block!(serial.read()).unwrap();

        if byte == 8 { // If 'backspace' is pressed
            input.pop();
            serial.write_str("\r");
            serial.write_str(str::from_utf8(&[32; MAX_INPUT_LEN]).unwrap());
            serial.write_str("\r");
            serial.write_str(str::from_utf8(&input).unwrap());
            serial.flush();
        } else if byte == 13 { // If 'enter' is pressed
            match str::from_utf8(&input) {
                Ok(s) => {
                    if ["magnetometer", "accelerometer"].iter().any(|o| o == &s) {
                        // TODO: print sensor data to serial
                        // let data = sensor.accel_data().unwrap();
                        // let msg = concat!("{:?}", data.x);
                        // serial.write_str(data.x);
                    } else {
                        serial.write_str("\r\nInvalid selection\r\n");
                    };
                }
                Err(_) => {
                    serial.write_str("\r\nInput invalid\r\n");
                }
            }
            serial.flush();
            input.clear();
        } else {
            match str::from_utf8(&[byte]) {
                Ok(s) => {
                    serial.write_str(s);
                    serial.flush();
                    match input.push(byte) {
                        Err(_) => {
                            serial.write_str("Too many characters! Try again.\r\n");
                            serial.flush();
                            input.clear();
                        }
                        _ => {}
                    }
                }
                _ => {}
                // Err(err) => {
                //     serial.write_str("Invalid character, try again.\r\n");
                // }
            }
        }



        rprintln!("{}", str::from_utf8(&input).unwrap());
        // if sensor.accel_status().unwrap().xyz_new_data {
        //     let data = sensor.accel_data().unwrap();
        //     rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        // }
    }
}
