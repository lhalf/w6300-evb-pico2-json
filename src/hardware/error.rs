#[derive(Debug)]
pub enum Error {
    Spi,
    WiznetEthernet,
    SpawnTask,
    BindPort,
}
