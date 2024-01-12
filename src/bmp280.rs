use bme280::i2c::BME280;

use crate::prelude::*;

struct BMP280 {
    bmp: BME280<SharedI2C>,
}

impl BMP280 {
    async fn new(bmp: BME280<SharedI2C>) -> Self {
        Self { bmp }
    }
}
