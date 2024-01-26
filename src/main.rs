#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::cell::RefCell;

use cansat::{
    blink::blink,
    bme280::{bme280_stream, BME280},
    display::{display_numerical_data, Display},
    mpu6050::mpu6050_stream,
    prelude::*,
};

use embassy_time::Ticker;
use hal::{
    clock::ClockControl,
    i2c::*,
    peripherals::{Peripherals, I2C0},
    timer::TimerGroup,
    xtensa_lx::singleton,
    IO,
};

use embassy_executor::Spawner;
use esp_println::println;
use mpu6050::Mpu6050;

use embedded_hal_bus::i2c::CriticalSectionDevice;

#[main]
async fn main(spawner: Spawner) -> ! {
    #[cfg(feature = "alloc")]
    cansat::alloc::init_heap();

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);

    embassy::init(&clocks, timer_group0);

    // To change the log_level change the env section in .cargo/config.toml or remove it and set ESP_LOGLEVEL manually before running cargo run this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    #[cfg(feature = "log")]
    esp_println::logger::init_logger_from_env();
    info!("Logger is setup");
    println!("Hello world!");

    // #[cfg(feature = "net")]
    // let _wifi_init = esp32_wifi::initialize(
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

    //we must share the i2c bus between the two, as otherwise the functions want to "own" the i2c bus themselves
    let i2c_mutex =
        singleton!(:Mutex<RefCell<I2C<'static,I2C0>>> = Mutex::new(RefCell::new(i2c))).unwrap();

    let display = Display::new(
        CriticalSectionDevice::new(i2c_mutex),
        ssd1306::size::DisplaySize128x64,
    )
    .await
    .expect("display initialisation failed");

    let mpu = Mpu6050::new(CriticalSectionDevice::new(i2c_mutex));

    // let bme = BME280::new(shared_i2c.acquire_i2c()).map_err(|e| { error!("{e:?}") }).unwrap();

    spawner.spawn(blink(led.degrade())).unwrap();
    spawner.spawn(display_numerical_data(display)).unwrap();

    spawner.spawn(mpu6050_stream(mpu)).unwrap();
    // spawner.spawn(bme280_stream(bme)).unwrap();

    let mut ticker = Ticker::every(Duration::from_secs(10));

    loop {
        trace!("KeepAlive tick");
        ticker.next().await;
    }
}
