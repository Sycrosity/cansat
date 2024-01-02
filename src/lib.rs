#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

pub mod prelude {

    pub use esp32_hal as hal;
    pub use esp_backtrace as _;

    pub use embassy_executor::task;
    pub use esp_println::println;

    pub use hal::{
        embassy,
        gpio::{AnyPin, Output, PushPull},
        prelude::*,
    };

    pub use embassy_time::{Duration, Timer};

    pub use log::{debug, error, info, trace, warn};
}
