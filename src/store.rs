use orbtk::{Button, Click, ComboBox, Place, Point, Rect, Text, Window};
use std::process::exit;

pub fn store_init() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600),"Smoke Installer - Store");

    // Hello Smoke
    let hello_smoke_text = hello_smoke::store::name();
    let hello_smoke_website = hello_smoke::store::website_button();
    let hello_smoke_install = hello_smoke::store::install_button();
    hello_smoke_install.on_click(move |_hello_smoke_install: &Button, _point: Point| {
        hello_smoke::store::install();
    });

    let exit_btn = Button::new();
    exit_btn
        .position( 1055, 0).text("Exit")
        .on_click(move |_exit_btn: &Button, _point: Point| {
            exit(0);
        });

    window.add(&hello_smoke_text);
    window.add(&hello_smoke_website);
    window.add(&hello_smoke_install);
    window.add(&exit_btn);
    window.exec();
}
