use orbtk::{Button, Label, Place, Text};
use std::sync::Arc;

pub fn install_button() -> Arc<Button> {
    let btn = Button::new();
    btn.position(110, 30).text("Install");

    return btn;
}

pub fn website_button() -> Arc<Button> {
    let btn = Button::new();
    btn.position(110, 60).text("Website");

    return btn;
}

pub fn name() -> Arc<Label> {
    let text = Label::new();
    text.position(110, 0).text("Hello Smoke");

    return text;
}
