extern crate orbtk;
use orbtk::{Button, Click, Color, Place, Point, Text, Rect, Window};
use orbtk::theme::Theme;

mod store;

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600),"Smoke Installer");

    let store_btn = Button::new();
    store_btn
        .position(0, 0).text("Store")
        .on_click(move |_store_btn: &Button, _point: Point| {
            // window.close();
            store::store_init();
        });

    window.add(&store_btn);
    window.exec();
}
