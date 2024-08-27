#![allow(dead_code)]

#[errorium::errors]
fn some_func() {
    return;
}

fn main() {
    some_func();
}
