#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod blink;
mod display;

extern crate alloc;
use core::mem::MaybeUninit;

use cansat::prelude::*;

use hal::{clock::ClockControl, i2c::*, peripherals::Peripherals, timer::TimerGroup, IO};

use embassy_executor::Spawner;

// use esp_wifi::{initialize, EspWifiInitFor};

use crate::{
    blink::blink,
    display::{screen_counter, Display},
};

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

// type Display = ssd1306::Ssd1306<I2CInterface<I2C<'static, I2C0>>, DisplaySize128x64, TerminalMode>;

#[main]
async fn main(spawner: Spawner) -> ! {
    // init_heap();

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);

    embassy::init(&clocks, timer_group0.timer0);

    // setup logger
    // To change the log_level change the env section in .cargo/config.toml or remove it and set ESP_LOGLEVEL manually before running cargo run this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358

    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");

    // let _wifi_init = initialize(
    //     EspWifiInitFor::Wifi,
    //     timer,
    //     Rng::new(peripherals.RNG),
    //     system.radio_clock_control,
    //     &clocks,
    // )
    // .unwrap();

    let led = io.pins.gpio2.into_push_pull_output();

    let scl = io.pins.gpio22;
    let sda = io.pins.gpio21;

    let i2c = I2C::new(peripherals.I2C0, sda, scl, 400u32.kHz(), &clocks);

    let display = Display::new(i2c, ssd1306::size::DisplaySize128x64);

    spawner.spawn(blink(led.degrade())).unwrap();
    spawner.spawn(screen_counter(display)).unwrap();

    let _button = io.pins.gpio15.into_pull_up_input();

    loop {
        // let _ = button.wait_for_low().await;
        Timer::after_millis(10_000).await;
    }
}
