use std::process::Command;
use std::sync::Arc;
use orbtk::{Button, Click, Point, Text};

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
