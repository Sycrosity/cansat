use bme280::Measurements;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use crate::prelude::*;

pub type BmeSignal = Signal<CriticalSectionRawMutex, BmeData>;
pub static BME_SIGNAL: BmeSignal = Signal::new();
// pub type BmeData = bme280::Measurements<hal::i2c::Error>;

#[derive(Clone, Copy, ErrorCategory)]
#[error_category(links(BMEError))]
#[repr(u8)]
pub enum BMEError {
    DataErr,
    InterfaceError,
    InitialisationError,
    DelayError,
    TimeoutError,
    Ack,
}

impl From<bme280::Error<hal::i2c::Error>> for BMEError {
    fn from(value: bme280::Error<hal::i2c::Error>) -> Self {
        match value {
            bme280::Error::CompensationFailed => Self::DataErr,
            bme280::Error::Bus(_) => Self::InterfaceError,
            // Self::InterfaceError,
            bme280::Error::InvalidData => Self::DataErr,
            bme280::Error::NoCalibrationData => Self::InitialisationError,
            bme280::Error::UnsupportedChip => Self::InitialisationError,
            bme280::Error::Delay => Self::DelayError,
        }
    }
}

type Result<T> = core::result::Result<T, BMEError>;

pub struct BME280 {
    pub bme: bme280::i2c::BME280<SharedI2C>,
}

impl BME280 {
    pub fn new(i2c: SharedI2C) -> Result<Self> {
        let mut bme280 = Self {
            bme: bme280::i2c::BME280::new_primary(i2c),
        };

        bme280.init()?;

        // bme280.try_init().await?;

        Ok(bme280)
    }

    pub async fn try_init(&mut self) -> Result<()> {
        Backoff::new(|| self.init()).retry().await?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        let mut delay: Delay = Delay;

        self.bme.init(&mut delay)?;
        Ok(())
    }

    // pub fn init(&mut self) -> Result<()> {
    //     let mut delay: Delay = Delay;

    //     self.bme.init(&mut delay)?;
    //     Ok(())
    // }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct BmeData {
    /// temperature in degrees celsius
    pub temperature: f32,
    /// pressure in pascals
    pub pressure: f32,
    /// percent relative humidity (`0` with BMP280)
    pub humidity: f32,
}

impl<E: embedded_hal::i2c::Error> From<Measurements<E>> for BmeData {
    fn from(value: Measurements<E>) -> Self {
        Self {
            temperature: value.temperature,
            pressure: value.pressure,
            humidity: value.humidity,
        }
    }
}

#[task]
pub async fn bme280_stream(
    mut bme: BME280,
    //  control: &'static MpuSignal
) {
    let mut delay: Delay = Delay;

    bme.init().print_warn();

    loop {
        let bme_data: BmeData = bme
            .bme
            .measure(&mut delay)
            .map(Into::<BmeData>::into)
            .map_err(|e| error!("{e:?}"))
            .unwrap_or_default();

        // let mpu_data = MpuData {
        //     roll_pitch: mpu.get_acc_angles().unwrap(),
        //     temp: mpu.get_temp().unwrap(),
        //     gyro: mpu.get_gyro().unwrap(),
        //     acc: mpu.get_acc().unwrap(),
        // };

        info!("{bme_data:?}");

        BME_SIGNAL.signal(bme_data);

        Timer::after_millis(1000).await;
    }
}
