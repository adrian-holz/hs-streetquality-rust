use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::config::Config;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::{Hertz, KiloHertz};
use mpu6886::Mpu6886;

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Street Quality!");

    let peripherals = Peripherals::take().unwrap();
    let mut i2c_config = Config::new();
    i2c_config.baudrate = Hertz::from(KiloHertz(100));

    let i2c0 = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio21,
        peripherals.pins.gpio22,
        &i2c_config,
    )
    .unwrap();
    let mut mpu = Mpu6886::new(i2c0);

    let mut delay = FreeRtos;
    mpu.init(&mut delay).unwrap();

    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles().unwrap();
        log::info!("r/p: {:?}", acc);

        // get sensor temp
        let temp = mpu.get_temp().unwrap();
        log::info!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().unwrap();
        log::info!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().unwrap();
        log::info!("acc: {:?}", acc);
    }
}
