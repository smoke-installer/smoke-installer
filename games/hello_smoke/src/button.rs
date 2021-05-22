use orbtk::{Button, Place, Text};
use std::sync::Arc;

pub fn button_init() -> Arc<Button> {
    let btn = Button::new();
    btn.position(110, 0).text("Hello Smoke");

    return btn;
}
