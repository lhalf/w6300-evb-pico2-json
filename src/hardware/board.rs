use embassy_rp::bind_interrupts;
use embassy_rp::config::Config;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::InterruptHandler;
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::spi::Spi;
use embassy_rp::{
    clocks::RoscRng,
    gpio::{Input, Level, Output, Pull},
};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const SPI_FREQUENCY: u32 = 15_000_000;

pub struct Board {
    pub spi: Spi<'static, PIO0, 0, embassy_rp::spi::Async>,
    pub cs: Output<'static>,
    pub w6300_int: Input<'static>,
    pub w6300_reset: Output<'static>,
    pub rng: RoscRng,
}

pub fn init() -> (Board, Output<'static>) {
    let p = embassy_rp::init(Config::default());

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    (
        Board {
            spi: Spi::new(
                &mut common,
                sm0,
                p.PIN_17,
                p.PIN_18,
                p.PIN_19,
                p.DMA_CH0,
                p.DMA_CH1,
                spi_config(),
            ),
            cs: Output::new(p.PIN_16, Level::High),
            w6300_int: Input::new(p.PIN_15, Pull::Up),
            w6300_reset: Output::new(p.PIN_22, Level::High),
            rng: RoscRng,
        },
        Output::new(p.PIN_25, Level::Low),
    )
}

fn spi_config() -> embassy_rp::spi::Config {
    let mut spi_cfg = embassy_rp::spi::Config::default();
    spi_cfg.frequency = SPI_FREQUENCY;
    spi_cfg
}
