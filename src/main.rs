#![no_std]
#![no_main]

mod hardware;

use crate::hardware::board::Board;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::udp::UdpSocket;
use embassy_rp::gpio::Output;
use hardware::error::Error;
use hardware::wiznet;
use panic_probe as _;
use w6300_evb_pico2_json::relay::relay;

use embassy_time::{Duration, Timer};
use embedded_hal_async::spi::SpiDevice;
use embedded_hal_bus::spi::ExclusiveDevice;
use embassy_time::Delay;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let (mut board, led) = hardware::board::init();

    let mut spi_dev = ExclusiveDevice::new(board.spi, board.cs, Delay).unwrap();

    // 1. Hardware Reset
    board.w6300_reset.set_low();
    Timer::after(Duration::from_millis(2)).await;
    board.w6300_reset.set_high();

    // 2. Wait for Stability (W6300 requires ~60.3ms) [cite: 153]
    Timer::after(Duration::from_millis(100)).await;

    // 3. Read Version Register (Address 0x0000 in Common Block)
    // W6300 SPI Frame: [Instruction, Addr_H, Addr_L, Dummy, Data]
    // Instruction:
    //   Bit 7-6 (Mode): 00 (Single SPI)
    //   Bit 5 (R/W): 0 (Read)
    //   Bit 4-0 (Block): 00000 (Common Register)
    //   => 0x00
    let cmd = [0x00, 0x00, 0x00, 0x00];
    let mut data = [0u8; 1];

    // Using embedded-hal-async transaction
    use embedded_hal_async::spi::Operation;
    let res = spi_dev.transaction(&mut [
        Operation::Write(&cmd),
        Operation::TransferInPlace(&mut data)
    ]).await;

    match res {
        Ok(_) => {
            // Expected Output: 0x61
            defmt::info!("Read Version: {:#04x}", data[0]);
            if data[0] != 0x61 {
                defmt::error!("SPI Works, but data is wrong. Check MISO/MOSI wiring.");
            }
        },
        Err(_) => defmt::error!("SPI Transaction Failed"),
    }

    // let socket = match setup(&spawner, board).await {
    //     Ok(socket) => socket,
    //     Err(error) => report_error(error, led),
    // };
    //
    // let mut buffer = [0; 4096];
    //
    // loop {
    //     relay(&socket, &mut buffer).await;
    // }
}

async fn setup(spawner: &Spawner, board: Board) -> Result<UdpSocket<'static>, Error> {
    let (socket, ethernet_runner, network_runner) = hardware::init(board).await?;

    spawner.spawn(ethernet_task(ethernet_runner).map_err(|_| Error::SpawnTask)?);

    spawner.spawn(network_task(network_runner).map_err(|_| Error::SpawnTask)?);

    Ok(socket)
}

fn report_error(error: Error, mut led: Output<'static>) -> ! {
    led.set_high();
    defmt::error!("{}", error);
    loop {}
}

#[embassy_executor::task]
pub async fn ethernet_task(runner: wiznet::Runner) {
    runner.run().await
}

#[embassy_executor::task]
pub async fn network_task(
    mut runner: embassy_net::Runner<'static, embassy_net_wiznet::Device<'static>>,
) {
    runner.run().await
}
