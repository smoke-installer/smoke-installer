extern crate orbtk;
use orbtk::{Button, Click, Color, Place, Point, Text, Rect, Window};
use std::process::exit;

mod help;
mod installed;
mod store;

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600),"Smoke Installer");

    let store_btn = Button::new();
    store_btn
        .position(0, 0).text("Store")
        .on_click(move |_store_btn: &Button, _point: Point| {
            store::store_init();
        });

    let installed_btn = Button::new();
    installed_btn
        .position(100, 0).text("Installed")
        .on_click(move |_installed_btn: &Button, _point: Point| {
            installed::installed_init();
        });

    let help_btn = Button::new();
    help_btn
        .position(230, 0).text("Help")
        .on_click(move |_help_btn: &Button, _point: Point| {
            help::help_init();
        });

    let exit_btn = Button::new();
    exit_btn
        .position(315, 0).text("Exit")
        .on_click(move |_exit_btn: &Button, _point: Point| {
            exit(0);
        });

    window.add(&store_btn);
    window.add(&installed_btn);
    window.add(&help_btn);
    window.add(&exit_btn);
    window.exec();
}
