extern crate orbtk;
use orbtk::{Button, Click, Place, Point, Text, Rect, Window};

mod store;

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600),"Smoke Installer");

    let x = 10;
    let mut y = 0;

    let store_btn = Button::new();
    store_btn
        .position(0, 0).text("Store")
        .on_click(move |_store_btn: &Button, _point: Point| {
            store::store_init();
        });

    window.add(&store_btn);
    window.exec();
}
