use core::fmt::Debug;

use crate::prelude::*;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Delay;

use mpu6050::*;
use nalgebra::{Vector2, Vector3};

pub type MpuSignal = Signal<CriticalSectionRawMutex, MpuData>;
pub static MPU_SIGNAL: MpuSignal = Signal::new();

#[derive(Clone, Copy, ErrorCategory)]
#[repr(u8)]
pub enum MpuError {
    InitFailed,
    ReadoutFailed,
}

#[derive(Debug)]
pub struct MpuData {
    pub roll_pitch: Vector2<f32>,
    pub temp: f32,
    pub gyro: Vector3<f32>,
    pub acc: Vector3<f32>,
}

#[task]
pub async fn get_sensor_data(mut mpu: Mpu6050<SharedI2C>) {
    let mut delay = Delay;

    mpu.init(&mut delay).unwrap();

    loop {
        // get roll and pitch estimate
        let acc: Vector2<f32> = mpu.get_acc_angles().unwrap();
        info!("r/p: {:?}", acc);

        // get temp
        let temp = mpu.get_temp().unwrap();
        info!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity
        let gyro: Vector3<f32> = mpu.get_gyro().unwrap();
        info!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc: Vector3<f32> = mpu.get_acc().unwrap();
        info!("acc: {:?}", acc);

        Timer::after_secs(1).await;
    }
}

#[task]
pub async fn mpu6050_stream(
    mut mpu: Mpu6050<SharedI2C>,
    //  control: &'static MpuSignal
) {
    let mut delay = Delay;

    mpu.init(&mut delay).unwrap();

    loop {
        let mpu_data = MpuData {
            roll_pitch: mpu.get_acc_angles().print_warn(),
            temp: mpu.get_temp().print_warn(),
            gyro: mpu.get_gyro().print_warn().map(|x| x.to_degrees()),
            acc: mpu.get_acc().print_warn(),
        };

        trace!("{mpu_data:?}");

        MPU_SIGNAL.signal(mpu_data);

        Timer::after_millis(100).await;
    }
}
