use crate::prelude::*;

use embassy_time::Delay;
use hal::{i2c::I2C, peripherals::I2C0};
use mpu6050::*;

#[task]
// pub async fn get_sensor_data(mut mpu: Mpu6050<shared_bus::I2cProxy<'static, shared_bus::XtensaMutex<I2C<'static,I2C0>>>>) {
pub async fn get_sensor_data(mut mpu: Mpu6050<I2CShared>) {
    let mut delay = Delay;

    mpu.init(&mut delay).unwrap();

    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles().unwrap();
        info!("r/p: {:?}", acc);

        // get temp
        let temp = mpu.get_temp().unwrap();
        info!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().unwrap();
        info!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().unwrap();
        info!("acc: {:?}", acc);

        Timer::after_secs(1).await;
    }
}

