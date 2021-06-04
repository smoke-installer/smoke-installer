use orbtk::{Button, Click, Label, Place, Point, Text};
use crate::web;
use std::sync::Arc;

pub fn install_button() -> Arc<Button> {
    let btn = Button::new();
    btn.position(110, 30).text("Install");

    return btn;
}

pub fn website_button() -> Arc<Button> {
    let btn = Button::new();
    btn.position(110, 60).text("Website").on_click(move |_btn: &Button, _point: Point| {
        web::open_website();
    });

    return btn;
}

pub fn name() -> Arc<Label> {
    let text = Label::new();
    text.position(110, 0).text("Hello Smoke");

    return text;
}
