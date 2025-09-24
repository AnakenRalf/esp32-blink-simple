use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    
    esp_idf_svc::log::EspLogger::initialize_default();

    // Initialize the ESP32 periphireals
    let periphireals = Peripherals::take().unwrap();

    // Select the GPIO5 pin
    let mut led = PinDriver::output(periphireals.pins.gpio5).unwrap();

    // And there funn part
    log::info!("There we start");
    loop {
        led.set_high().unwrap();
        log::info!("Led ON");
        thread::sleep(Duration::from_millis(1000));
        log::info!("Led Off");
        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}