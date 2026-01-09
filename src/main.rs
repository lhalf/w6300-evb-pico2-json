#![no_std]
#![no_main]

mod hardware;

use crate::hardware::board::Board;
use embassy_executor::Spawner;
use embassy_net::udp::UdpSocket;
use embassy_rp::gpio::Output;
use hardware::error::Error;
use hardware::wiznet;
use panic_halt as _;
use w6300_evb_pico2_json::relay::relay;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let (board, led) = hardware::board::init();

    let socket = match setup(&spawner, board).await {
        Ok(socket) => socket,
        Err(_) => setup_error(led),
    };

    let mut buffer = [0; 4096];

    loop {
        relay(&socket, &mut buffer).await;
    }
}

async fn setup(spawner: &Spawner, board: Board) -> Result<UdpSocket<'static>, Error> {
    let (socket, ethernet_runner, network_runner) = hardware::init(board).await?;

     spawner.spawn(ethernet_task(ethernet_runner).map_err(|_| Error::SpawnTask)?);

    spawner.spawn(network_task(network_runner).map_err(|_| Error::SpawnTask)?);

    Ok(socket)
}

fn setup_error(mut led: Output<'static>) -> ! {
    led.set_high();
    panic!("setup failed")
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
