#[test = "Test correct `From` trait implementation for all generated error tag types."]
fn error_tags_from_impl_test() {
    use errorium::errorium;
    errorium!(MainError, [ExpectedErr, UnexpectedErr]);

    fn fmt_err() -> Result<(), std::fmt::Error> {
        Err(std::fmt::Error)
    }

    fn io_err() -> Result<(), std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "io error"))
    }

    fn anyhow_err() -> anyhow::Result<()> {
        anyhow::bail!("anyhow error")
    }

    fn some_func() -> Result<(), MainError> {
        fmt_err().map_err(UnexpectedErr::new)?;
        io_err().map_err(UnexpectedErr::new)?;
        anyhow_err().map_err(UnexpectedErr::new)?;

        fmt_err().map_err(UnexpectedErr::new)?;
        io_err().map_err(UnexpectedErr::new)?;
        anyhow_err().map_err(UnexpectedErr::new)?;

        fmt_err().map_err(UnexpectedErr::from)?;
        io_err().map_err(UnexpectedErr::from)?;
        anyhow_err().map_err(UnexpectedErr::from)?;

        fmt_err().map_err(UnexpectedErr::from)?;
        io_err().map_err(UnexpectedErr::from)?;
        anyhow_err().map_err(UnexpectedErr::from)?;

        Ok(())
    }

    let _unused = some_func();
}

#[test]
fn basic_test() {
    use errorium::errorium;
    errorium!(MainError, [ExpectedErr, UnexpectedErr]);

    fn fmt_err() -> Result<(), std::fmt::Error> {
        Err(std::fmt::Error)
    }

    fn io_err() -> Result<(), std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "io error"))
    }

    fn anyhow_err() -> anyhow::Result<()> {
        anyhow::bail!("anyhow error")
    }
}
