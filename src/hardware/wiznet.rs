use crate::hardware::board::Board;
use crate::hardware::error::Error;
use embassy_net_wiznet::chip::W6300;
use embassy_net_wiznet::{Device, State};
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::spi::Spi;
use embassy_rp::spi::Async;
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;
use static_cell::StaticCell;
use w6300_evb_pico2_json::config::MAC_ADDRESS;

pub type Runner = embassy_net_wiznet::Runner<
    'static,
    W6300,
    ExclusiveDevice<Spi<'static, PIO0, 0, Async>, Output<'static>, Delay>,
    Input<'static>,
    Output<'static>,
>;

pub async fn init(board: Board) -> Result<(Device<'static>, Runner), Error> {
    static STATE: StaticCell<State<32, 32>> = StaticCell::new();

    embassy_net_wiznet::new(
        MAC_ADDRESS,
        STATE.init(State::new()),
        ExclusiveDevice::new(board.spi, board.cs, Delay).map_err(|_| Error::Spi)?,
        board.w6300_int,
        board.w6300_reset,
    )
    .await
    .map_err(|_| Error::WiznetEthernet)
}
