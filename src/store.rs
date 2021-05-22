use orbtk::{ComboBox, Rect, Window};

pub fn store_init() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600),"Smoke Installer - Store");

    let smoke_installer_button = hello_smoke::button::button_init();

    window.exec();
}