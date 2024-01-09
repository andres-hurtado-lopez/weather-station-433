#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::peripherals::USART1;
use embassy_stm32::{usart, bind_interrupts};
use embassy_stm32::usart::{Uart, Config};
use embassy_time::{Delay, Timer};
use embassy_stm32::gpio::{Output, Level, Speed};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Wireless weather station on 433MHz Band");

    let peri = p.USART1;
    let rx = p.PA10;
    let tx = p.PA9;
    let irq = Irqs;
    let tx_dma = p.DMA1_CH1;
    let rx_dma = p.DMA1_CH2;
    let config = Config::default();


    match Uart::new(
	peri,
	rx,
	tx,
	irq,
	tx_dma,
	rx_dma,
	config,
    ) {
	Ok(mut uart) => {
	},
	Err(why) => {
	    error!("Failed initializing UART1. {}", why);
	}
    }
    let mut program_wireless = Output::new(p.PB5, Level::Low, Speed::Low);

    loop {
	program_wireless.toggle();
        Timer::after_millis(100).await;
    }
}
