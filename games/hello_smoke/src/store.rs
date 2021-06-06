use orbtk::{Button, Click, Label, Place, Point, Text};
use std::process::Command;
use std::sync::Arc;

pub fn install_button() -> Arc<Button> {
    let btn = Button::new();
    btn.position(110, 30).text("Install");

    return btn;
}

pub fn website_button() -> Arc<Button> {
    let btn = Button::new();
    btn.position(110, 60).text("Website").on_click(move |_btn: &Button, _point: Point| {
        open_website();
    });

    return btn;
}

pub fn name() -> Arc<Label> {
    let text = Label::new();
    text.position(110, 0).text("Hello Smoke");

    return text;
}

pub fn install() {
    Command::new("si-install")
        .args(&["2021_6_6_10_0.zip", "test1234", "hello-smoke-installer"])
        .spawn();
}

pub fn open_website() {
    webbrowser::open("https://smoke-installer.github.io/games/hello-smoke.html");
}
