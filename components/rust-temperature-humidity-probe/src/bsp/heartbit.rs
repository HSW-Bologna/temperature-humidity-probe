use esp_idf_hal::gpio::*;
use std::time::Duration;

pub fn init(pin: impl OutputPin) -> anyhow::Result<()> {
    let led = PinDriver::output(pin).unwrap();
    let _thread = std::thread::Builder::new()
        .stack_size(2048)
        .spawn(move || heartbit(led))
        .unwrap();

    log::info!("Heartbit initialized");

    Ok(())
}

fn heartbit<'a, P: Pin>(mut driver: PinDriver<'a, P, Output>) -> anyhow::Result<()> {
    loop {
        driver.set_high()?;
        std::thread::sleep(Duration::from_millis(50));
        driver.set_low()?;
        std::thread::sleep(Duration::from_millis(1950));
    }
}
