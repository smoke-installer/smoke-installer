extern crate orbtk;
use orbtk::{Button, Click, Color, Place, Point, Text, Rect, Window};

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
        .position(75, 0).text("Installed")
        .on_click(move |_installed_btn: &Button, _point: Point| {
            installed::installed_init();
        });

    window.add(&store_btn);
    window.add(&installed_btn);
    window.exec();
}
