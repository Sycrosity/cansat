#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(unused)]
#![allow(clippy::unused_unit)]

pub mod blink;
pub mod display;
pub mod errors;
pub mod mpu6050;
pub mod utils;
pub mod bmp280;

#[cfg(feature = "alloc")]
pub mod alloc {

    extern crate alloc;

    #[global_allocator]
    static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

    pub fn init_heap() {
        use core::mem::MaybeUninit;
    
        const HEAP_SIZE: usize = 32 * 1024;
        static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();
    
        unsafe {
            ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
        }
    }
}


pub mod prelude {

    pub type I2CShared = shared_bus::I2cProxy<
        'static,
        shared_bus::XtensaMutex<hal::i2c::I2C<'static, hal::peripherals::I2C0>>,
    >;

    pub const DEFAULT_INTERVAL: Duration = Duration::from_millis(500);

    pub const DEFAULT_MAX_ELAPSED_TIME: Duration = Duration::from_secs(5);

    pub use crate::{errors::*, utils::*};

    pub use esp32_hal as hal;
    pub use esp_backtrace as _;
    pub use esp_println as _;

    pub use embedded_error_chain::prelude::*;

    pub use embassy_executor::task;
    pub use esp_println::println;

    pub use hal::{
        embassy,
        gpio::{AnyPin, Output, PushPull},
        prelude::*,
    };

    pub use embassy_time::{Duration, Instant, Ticker, Timer};

    pub use log::{debug, error, info, trace, warn};
}
