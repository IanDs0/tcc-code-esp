use esp_idf_sys as _;
// use log::*;
use std::{
    thread::sleep,
    time::Duration,
};


use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::*;

use esp_idf_svc::{
    wifi::EspWifi,
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};
use embedded_svc::wifi::{
    ClientConfiguration, 
    // Wifi, 
    Configuration
};
// use embedded_svc::wifi::AuthMethod;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();








    esp_idf_sys::link_patches();

    let mut led = match PinDriver::output(peripherals.pins.gpio2) {
        Ok(it) => it,
        Err(err) => panic!("Failed to initialize LED: {:?}", err)
    };
    







    led.set_high().unwrap();

    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs),
    ).unwrap();

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "IanDs".into(),
        password: "Ian12345".into(),
        ..Default::default()
    })).unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();
    while !wifi_driver.is_connected().unwrap() {
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }
    println!("Conectado");


    loop {

        if wifi_driver.is_connected().unwrap() {
            led.set_low().unwrap();
            println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());
            sleep(Duration::new(10, 0));
        }
        
    }


}
