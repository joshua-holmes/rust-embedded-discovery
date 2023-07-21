use libm;
use lsm303agr::{Lsm303agr, interface::I2cInterface, mode::MagOneShot, AccelOutputDataRate};
use microbit::{pac::TWIM0, hal::Twim};


#[derive(Debug)]
pub enum CalcError {
    FrequencyNotSupported,
}


pub fn get_magnitude_as_i32(a: i32, b: i32, c: i32) -> i32 {
    let a = a as f32;
    let b = b as f32;
    let c = c as f32;
    let magnitude = libm::sqrtf(a * a + b * b + c * c);
    libm::roundf(magnitude / 1000.) as i32
}

pub fn get_magnitude(a: i32, b: i32, c: i32) -> f32 {
    let a = a as f32;
    let b = b as f32;
    let c = c as f32;
    libm::sqrtf(a * a + b * b + c * c)
}


pub fn record_max_accel_as_i32(sensor: &mut Lsm303agr<I2cInterface<Twim<TWIM0>>, MagOneShot>, accel_output_data_rate: AccelOutputDataRate) -> Result<i32, CalcError> {
    let frequency = match accel_output_data_rate {
        AccelOutputDataRate::Hz1 => 1,
        AccelOutputDataRate::Hz10 => 10,
        AccelOutputDataRate::Hz25 => 25,
        AccelOutputDataRate::Hz50 => 50,
        AccelOutputDataRate::Hz100 => 100,
        AccelOutputDataRate::Hz200 => 200,
        AccelOutputDataRate::Hz400 => 400,
        AccelOutputDataRate::Khz1_344 => 1344,
        AccelOutputDataRate::Khz1_620LowPower => 1620,
        AccelOutputDataRate::Khz5_376LowPower => 5376,
    };

    let mut max = 0;
    for _ in 0..frequency {
        while !sensor.accel_status().unwrap().xyz_new_data {}
        let data = sensor.accel_data().unwrap();
        let mag = get_magnitude_as_i32(data.x, data.y, data.z);

        if mag > max { max = mag; }
    }

    Ok(max)
}
