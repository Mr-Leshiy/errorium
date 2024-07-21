#![allow(dead_code)]

use errorium::tags;

tags!(pub Tag1);

fn fmt_err() -> Result<(), std::fmt::Error> {
    Err(std::fmt::Error)
}

fn anyhow_err() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("anyhow error"))
}

fn tag1_err() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(anyhow_err().map_err(Tag1::tag)?)
}

#[test]
#[allow(unused_variables)]
fn basic_test() {
    if let Err(e) = tag1_err() {
        Tag1::handle(e, |e| {
            println!("Tagged 1 error: {e}");
        });
    }
}
