use orbtk::{Button, Rect, Window, Place};
use std::fs::{File, read_to_string};
use std::io::Read;

pub fn installed_init() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600),"Smoke Installer - Store");

    let installed = read_to_string("/etc/smoke_installer/installed.txt").unwrap();

    let x = 0;
    let y = 0;

    // Buttons
    let open_hello_smoke = hello_smoke::open::open();
    // -------

    if installed.contains("hello_smoke_installer") {
        open_hello_smoke.position(x, y);
        window.add(&open_hello_smoke);
    }

    window.exec();
}
