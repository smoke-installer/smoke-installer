use orbtk::{Button, Click, Text, Point};
use std::sync::Arc;
use std::process::Command;

fn start() {
    Command::new("Hello_Smoke")
        .spawn();
}

pub fn open() -> Arc<Button> {
    let btn = Button::new();
    btn.text("Hello Smoke").on_click(move |_btn: &Button, point: Point| {
        start();
    });

    return btn;
}
