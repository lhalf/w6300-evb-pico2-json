#[derive(Debug, defmt::Format)]
pub enum Error {
    Spi,
    WiznetEthernet,
    SpawnTask,
    BindPort,
}
