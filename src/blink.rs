use crate::prelude::*;

#[task]
pub async fn blink(mut led: AnyPin<Output<PushPull>>) {
    let mut on = led.is_set_high().unwrap();

    loop {
        led.toggle().unwrap();
        on = !on;

        if on {
            info!("ON!")
        } else {
            info!("OFF!")
        }
        Timer::after(Duration::from_millis(1000)).await;
    }
}
