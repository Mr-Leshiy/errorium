//! Test error conversions from any different types of errors and check the correct error
//! tagging propagation through the calling stack.

errorium::tags!(Tag);

#[test]
fn error_conversion_test() {
    fn fmt_error() -> Result<(), std::fmt::Error> {
        Err(std::fmt::Error)
    }

    fn anyhow_error() -> anyhow::Result<()> {
        Err(anyhow::anyhow!("anyhow error"))
    }

    #[derive(thiserror::Error, Debug)]
    #[error("thiserror MyError")]
    struct MyError;
    fn thiserror_error() -> Result<(), MyError> {
        Err(MyError)
    }

    fn get_error() -> errorium::Result<()> {
        fmt_error().map_err(Tag::tag)?;
        anyhow_error().map_err(Tag::tag)?;
        thiserror_error().map_err(Tag::tag)?;
        Ok(())
    }

    let _unused = get_error();
}

#[test]
fn tag_propagation_test() {
    fn tagged_error() -> errorium::Result<()> {
        Err(Tag::tag(std::fmt::Error).into())
    }
    fn fun1() -> errorium::Result<()> {
        tagged_error()
    }
    fn fun2() -> errorium::Result<()> {
        fun1()
    }

    #[allow(clippy::unreachable)]
    let Err(res) = fun2() else {
        unreachable!();
    };

    // check the tag propagation
    let mut is_handled = false;
    Tag::handle(res.as_ref(), |_| {
        is_handled = true;
    });
    assert!(is_handled);
}
