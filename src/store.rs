use orbtk::{Button, Click, ComboBox, Point, Rect, Window};

pub fn store_init() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600),"Smoke Installer - Store");

    // Hello Smoke
    let hello_smoke_text = hello_smoke::store::name();
    let hello_smoke_website = hello_smoke::store::website_button();
    let hello_smoke_install = hello_smoke::store::install_button();
    hello_smoke_install.on_click(move |_hello_smoke_install: &Button, _point: Point| {
        hello_smoke::install::install();
    });

    window.add(&hello_smoke_text);
    window.add(&hello_smoke_website);
    window.add(&hello_smoke_install);
    window.exec();
}
