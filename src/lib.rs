#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(unused)]

pub mod blink;
pub mod display;
pub mod mpu6050;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

#[cfg(feature = "alloc")]
pub fn init_heap() {
    use core::mem::MaybeUninit;

    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

pub mod prelude {

    pub type I2CShared = shared_bus::I2cProxy<
        'static,
        shared_bus::XtensaMutex<hal::i2c::I2C<'static, hal::peripherals::I2C0>>,
    >;

    pub use esp32_hal as hal;
    pub use esp_backtrace as _;
    pub use esp_println as _;

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
