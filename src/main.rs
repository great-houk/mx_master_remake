#![no_main]
#![no_std]
use cortex_m as _;
// use defmt::{dbg, println};
// use defmt_rtt as _;
use hal::{gpio::Level, prelude::InputPin};
use nrf52840_hal as hal;
use panic_probe as _;
use rtt_target::{rprintln as println, rtt_init};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut channels = rtt_init! {
        up: {
            0: {
                size: 1024,
                name: "Terminal",
            }
        }
        down: {
            0: {
                size: 16,
                name: "Terminal"
            }
        }
    };

    let peripherals = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let pos = port0.p0_08.into_push_pull_output(Level::Low);
    let neg = port0.p0_07.into_push_pull_output(Level::Low);
    let en_30v = port0.p0_26.into_push_pull_output(Level::Low);

    let mut read_buf = [0; 8];
    loop {
        let read = channels.down.0.read(&mut read_buf);
        println!("p");

        for c in &read_buf[..read] {
            match c {
                b'p' => println!("Pos"),
                b'n' => println!("Neg"),
                b'e' => println!("Enable"),
                _ => println!("Unknown char: {}", c),
            }
        }
    }
}
