#![allow(dead_code)]

use errorium::errorium;

errorium!(Error, [ExpectedErr, UnexpectedErr]);

impl Error {
    pub fn consume(self) {}
}

fn expected_err() -> Result<(), ExpectedErr> {
    Err(ExpectedErr(anyhow::anyhow!("expected error")))
}

fn unexpected_err() -> Result<(), UnexpectedErr> {
    Err(UnexpectedErr(anyhow::anyhow!("unexpected error")))
}

fn io_err() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "io error"))
}

fn anyhow_err() -> anyhow::Result<()> {
    anyhow::bail!("anyhow error")
}

fn some_func() -> Result<(), Error> {
    unexpected_err()?;
    expected_err()?;
    anyhow_err().map_err(ExpectedErr::new)?;
    io_err().map_err(ExpectedErr::new)?;

    Ok(())
}
